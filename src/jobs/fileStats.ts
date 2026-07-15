import { FileStats, FileStatsConfig } from '../types';
import { LmdbCache } from '../caching/LmdbCache';
import {
  getFileStats,
  calculateFileHash,
  sharedArrayBufferToHex,
  hexToSharedArrayBuffer,
} from '../utils';
import {
  fileStatsViaRust,
  rustCliDelegationEnabled,
} from '../external/rustCli';
import { AppResult, ok, err, DatabaseError, FileSystemError } from '../errors';

const JOB_NAME = 'fileStats';

/**
 * TS parity baseline for file-stats (MD5 + size + dates). Used only when
 * MEDIA_CURATOR_RUST is explicitly opted out (ts|0|false|no).
 */
export async function processFileStatsTsCore(
  filePath: string,
  config: FileStatsConfig,
): Promise<AppResult<FileStats>> {
  const statsResult = await getFileStats(filePath);
  if (statsResult.isErr()) {
    return err(statsResult.error);
  }
  const stats = statsResult.value;

  const hashResult = await calculateFileHash(
    filePath,
    stats.size,
    config.maxChunkSize,
  );
  if (hashResult.isErr()) {
    return err(hashResult.error);
  }
  const hash = hashResult.value;

  return ok({
    hash,
    size: stats.size,
    createdAt: stats.birthtime,
    modifiedAt: stats.mtime,
  });
}

/**
 * Production file-stats: Rust MD5+size authority by default (fail-closed).
 * Dates still come from local fs metadata. Cache wrapper remains in TS.
 */
export async function processFileStats(
  filePath: string,
  config: FileStatsConfig,
  cache: LmdbCache,
): Promise<AppResult<FileStats>> {
  const cacheKey = filePath;

  const configCheckResult = await cache.checkConfig(JOB_NAME, cacheKey, config);
  if (configCheckResult.isErr()) {
    console.warn(
      `Cache config check failed for ${filePath}, proceeding with calculation:`,
      configCheckResult.error,
    );
  } else if (configCheckResult.value.isValid) {
    const cacheGetResult = await cache.getCache<FileStats>(JOB_NAME, cacheKey);
    if (cacheGetResult.isErr()) {
      console.warn(
        `Cache get failed for ${filePath}, proceeding with calculation:`,
        cacheGetResult.error,
      );
    } else if (cacheGetResult.value.hit) {
      return ok(cacheGetResult.value.data!);
    }
  }

  let result: FileStats;

  if (rustCliDelegationEnabled()) {
    try {
      const rust = fileStatsViaRust(filePath);
      const datesResult = await getFileStats(filePath);
      if (datesResult.isErr()) {
        return err(datesResult.error);
      }
      const dates = datesResult.value;
      const hashResult = hexToSharedArrayBuffer(rust.md5);
      if (hashResult.isErr()) {
        return err(hashResult.error);
      }
      result = {
        hash: hashResult.value,
        size: rust.size,
        createdAt: dates.birthtime,
        modifiedAt: dates.mtime,
      };
    } catch (error) {
      return err(
        new FileSystemError(
          `Rust file-stats failed for ${filePath}: ${
            error instanceof Error ? error.message : String(error)
          }`,
          {
            cause: error instanceof Error ? error : undefined,
            context: { path: filePath, operation: 'fileStatsViaRust' },
          },
        ),
      );
    }
  } else {
    const coreResult = await processFileStatsTsCore(filePath, config);
    if (coreResult.isErr()) {
      return err(coreResult.error);
    }
    result = coreResult.value;
  }

  const setResult = await cache.setCache(JOB_NAME, cacheKey, result, config);
  if (setResult.isErr()) {
    console.warn(
      `Cache set failed for ${filePath}, but returning calculated result:`,
      setResult.error,
    );
  }

  return ok(result);
}

export async function getFileStatsHashKey(
  filePath: string,
  config: FileStatsConfig,
  cache: LmdbCache,
): Promise<AppResult<string>> {
  const statsResult = await processFileStats(filePath, config, cache);
  if (statsResult.isErr()) {
    return err(statsResult.error);
  }
  const stats = statsResult.value;

  try {
    const hexKey = sharedArrayBufferToHex(stats.hash);
    return ok(hexKey);
  } catch (error) {
    return err(
      new DatabaseError(
        `Failed to convert hash to hex key for ${filePath}: ${error instanceof Error ? error.message : String(error)}`,
        {
          cause: error instanceof Error ? error : undefined,
          context: { operation: 'hexConvert' },
        },
      ),
    );
  }
}
