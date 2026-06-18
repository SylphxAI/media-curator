import type { FileInfo, FileProcessorConfig } from './types.js'; // Removed unused FileStatsConfig, AdaptiveExtractionConfig
import type { LmdbCache } from './caching/LmdbCache.js';
import type { ExifTool } from 'exiftool-vendored';
import type { WorkerPool } from './contexts/types.js';
import { processFileStats } from './jobs/fileStats.js'; // Returns AppResult<FileStats>
import { processMetadata } from './jobs/metadataExtraction.js'; // Returns AppResult<Metadata>
import { processAdaptiveExtraction } from './jobs/adaptiveExtraction.js'; // Returns AppResult<MediaInfo>
import type { AppResult } from './errors.js';
import { ok, err } from './errors.js'; // Removed unused AnyAppError

// FileProcessorConfig interface moved to src/types.ts

/**
 * Processes a single media file to gather all necessary information (stats, metadata, media info).
 * Orchestrates calls to individual processing functions, leveraging caching internally within those functions.
 *
 * @param filePath The path to the media file.
 * @param config Combined configuration object holding configs for underlying jobs.
 * @param cache LmdbCache instance, passed down to job functions.
 * @param exifTool ExifTool instance, passed down to metadata job function.
 * @param workerPool WorkerPool instance, passed down to adaptive extraction job function.
 * @returns A Promise resolving to an AppResult containing the complete FileInfo object or an error.
 */
export async function processSingleFile(
  filePath: string,
  config: FileProcessorConfig,
  cache: LmdbCache,
  exifTool: ExifTool,
  workerPool: WorkerPool,
): Promise<AppResult<FileInfo>> {
  // Update return type
  // Run processing steps concurrently, now expecting AppResult from each
  const results = await Promise.all([
    processFileStats(filePath, config.fileStats, cache),
    processMetadata(filePath, exifTool, config.fileStats, cache),
    processAdaptiveExtraction(
      filePath,
      config.adaptiveExtraction,
      config.fileStats,
      cache,
      workerPool,
    ),
  ]);

  // Check results for errors
  const fileStatsResult = results[0];
  const metadataResult = results[1];
  const mediaResult = results[2];

  if (fileStatsResult.isErr()) {
    console.error(
      `Failed to get file stats for ${filePath}:`,
      fileStatsResult.error,
    );
    return err(fileStatsResult.error); // Propagate the specific error
  }
  if (metadataResult.isErr()) {
    console.error(
      `Failed to get metadata for ${filePath}:`,
      metadataResult.error,
    );
    return err(metadataResult.error);
  }
  if (mediaResult.isErr()) {
    console.error(
      `Failed adaptive extraction for ${filePath}:`,
      mediaResult.error,
    );
    return err(mediaResult.error);
  }

  // If all successful, combine the unwrapped results
  const fileInfo: FileInfo = {
    fileStats: fileStatsResult.value,
    metadata: metadataResult.value,
    media: mediaResult.value,
  };

  return ok(fileInfo); // Return combined result wrapped in ok
}
