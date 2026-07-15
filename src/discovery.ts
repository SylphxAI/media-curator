import { readdir } from 'fs/promises';
import path from 'path';
import { Semaphore } from 'async-mutex';
import chalk from 'chalk';
import { ALL_SUPPORTED_EXTENSIONS, getFileTypeByExt } from './utils';
import { CliReporter } from './reporting/CliReporter';
import { FileSystemError, safeTryAsync } from './errors';
import {
  discoverViaRust,
  rustCliDelegationEnabled,
} from './external/rustCli';

/**
 * TS parity baseline for discovery. Used only when MEDIA_CURATOR_RUST is
 * explicitly opted out (ts|0|false|no).
 */
export async function discoverFilesFnTsCore(
  sourceDirs: string[],
  concurrency: number = 10,
  reporter: CliReporter,
): Promise<Map<string, string[]>> {
  const allFiles: string[] = [];
  let dirCount = 0;
  let fileCount = 0;
  const semaphore = new Semaphore(concurrency);
  reporter.startSpinner('Discovering files...');

  async function scanDirectory(dirPath: string): Promise<void> {
    dirCount++;
    const readDirResult = await safeTryAsync(
      readdir(dirPath, { withFileTypes: true }),
      (e) =>
        new FileSystemError(
          `Error scanning directory ${dirPath}: ${e instanceof Error ? e.message : String(e)}`,
          {
            cause: e instanceof Error ? e : undefined,
            context: { path: dirPath, operation: 'readdir' },
          },
        ),
    );

    if (readDirResult.isErr()) {
      reporter.logError(readDirResult.error.message);
      reporter.updateSpinnerText(
        `Processed ${dirCount} directories, found ${fileCount} files... (Error in ${dirPath})`,
      );
      return;
    }

    const entries = readDirResult.value;
    const promises: Promise<void>[] = [];
    for (const entry of entries) {
      const entryPath = path.join(dirPath, entry.name);
      if (entry.isDirectory()) {
        promises.push(semaphore.runExclusive(() => scanDirectory(entryPath)));
      } else if (
        ALL_SUPPORTED_EXTENSIONS.has(
          path.extname(entry.name).slice(1).toLowerCase(),
        )
      ) {
        allFiles.push(entryPath);
        fileCount++;
      }
    }
    try {
      await Promise.all(promises);
    } catch (promiseAllError) {
      reporter.logError(
        `Error during concurrent directory scan under ${dirPath}:`,
        promiseAllError instanceof Error ? promiseAllError : undefined,
      );
    }
    reporter.updateSpinnerText(
      `Processed ${dirCount} directories, found ${fileCount} files...`,
    );
  }

  const initialScanPromises = sourceDirs.map((dirPath) =>
    semaphore.runExclusive(() => scanDirectory(dirPath)),
  );
  await Promise.all(initialScanPromises);

  reporter.stopSpinnerSuccess(
    `Discovery completed: Found ${fileCount} files in ${dirCount} directories`,
  );

  const result = new Map<string, string[]>();
  for (const file of allFiles) {
    const ext = path.extname(file).slice(1).toLowerCase();
    if (!result.has(ext)) {
      result.set(ext, []);
    }
    result.get(ext)!.push(file);
  }

  logDiscoveryStats(reporter, result, fileCount);
  return result;
}

function logDiscoveryStats(
  reporter: CliReporter,
  result: Map<string, string[]>,
  fileCount: number,
): void {
  reporter.logInfo('\nFile Format Statistics:');
  const sortedFormats = Array.from(result.keys()).sort(
    (a, b) =>
      getFileTypeByExt(a).unwrapOr(0) - getFileTypeByExt(b).unwrapOr(0) ||
      result.get(b)!.length - result.get(a)!.length,
  );
  for (const format of sortedFormats) {
    const count = result.get(format)!.length;
    reporter.logInfo(
      `${chalk.white(format.padEnd(6))}: ${count.toString().padStart(8)}`,
    );
  }
  reporter.logSuccess(
    `${chalk.green('Total'.padEnd(6))}: ${fileCount.toString().padStart(8)}`,
  );
}

/**
 * Production discovery: Rust authority by default (fail-closed).
 */
export async function discoverFilesFn(
  sourceDirs: string[],
  concurrency: number = 10,
  reporter: CliReporter,
): Promise<Map<string, string[]>> {
  if (!rustCliDelegationEnabled()) {
    return discoverFilesFnTsCore(sourceDirs, concurrency, reporter);
  }

  reporter.startSpinner('Discovering files (Rust)...');
  try {
    const rust = discoverViaRust(sourceDirs, concurrency);
    const result = new Map<string, string[]>();
    for (const [ext, paths] of Object.entries(rust.byExtension ?? {})) {
      result.set(ext, paths);
    }
    reporter.stopSpinnerSuccess(
      `Discovery completed: Found ${rust.stats.fileCount} files in ${rust.stats.dirCount} directories`,
    );
    logDiscoveryStats(reporter, result, rust.stats.fileCount);
    return result;
  } catch (error) {
    reporter.stopSpinnerSuccess('Discovery failed');
    throw new FileSystemError(
      `Rust discovery failed: ${
        error instanceof Error ? error.message : String(error)
      }`,
      {
        cause: error instanceof Error ? error : undefined,
        context: { operation: 'discoverViaRust' },
      },
    );
  }
}
