import { constants } from 'node:fs';
import { copyFile, lstat, mkdir, unlink } from 'node:fs/promises';
import { dirname } from 'node:path';
import { FileSystemError } from './errors.js';

/**
 * Execute one file transfer and surface failures to the owning caller.
 *
 * A move is an exclusive copy followed by source removal. This deliberately
 * avoids rename-overwrite races: the destination is created with EXCL and the
 * source is removed only after the copy succeeds. Any failure is normalized
 * and propagated so callers cannot report a false successful organize.
 */
export async function transferOrCopyFile(
  sourcePath: string,
  targetPath: string,
  isCopy: boolean,
): Promise<void> {
  try {
    await mkdir(dirname(targetPath), { recursive: true });

    try {
      await lstat(targetPath);
      throw new Error(`target already exists: ${targetPath}`);
    } catch (error) {
      if (
        !(error instanceof Error && 'code' in error && error.code === 'ENOENT')
      ) {
        throw error;
      }
    }

    await copyFile(sourcePath, targetPath, constants.COPYFILE_EXCL);
    if (!isCopy) await unlink(sourcePath);
  } catch (error) {
    throw new FileSystemError(
      `Error ${isCopy ? 'copying' : 'moving'} file ${sourcePath} to ${targetPath}: ${error instanceof Error ? error.message : String(error)}`,
      {
        cause: error instanceof Error ? error : undefined,
        context: {
          path: sourcePath,
          operation: isCopy ? 'copy' : 'move',
        },
      },
    );
  }
}
