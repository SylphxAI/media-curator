import { FileStats, FileStatsConfig } from '../types';
import { LmdbCache } from '../caching/LmdbCache';
import {
  getFileStats,
  sharedArrayBufferToHex,
  hexToSharedArrayBuffer,
} from '../utils';
import { fileStatsViaRust } from '../external/rustCli';
import { AppResult, ok, err, DatabaseError, FileSystemError } from '../errors';

const JOB_NAME = 'fileStats';

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
