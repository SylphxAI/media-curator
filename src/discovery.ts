import chalk from 'chalk';
import { getFileTypeByExt } from './utils';
import { CliReporter } from './reporting/CliReporter';
import { FileSystemError } from './errors';
import { discoverViaRust } from './external/rustCli';

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
 * Production discovery: Rust owns filesystem traversal; TypeScript presents it.
 */
export async function discoverFilesFn(
  sourceDirs: string[],
  concurrency: number = 10,
  reporter: CliReporter,
): Promise<Map<string, string[]>> {
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
