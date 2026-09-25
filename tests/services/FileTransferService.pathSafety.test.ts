import { mkdtempSync, rmSync } from 'fs';
import { tmpdir } from 'os';
import { join, relative, resolve, sep } from 'path';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import type { ExifTool } from 'exiftool-vendored';
import { FileTransferService } from '../../src/services/FileTransferService';
import {
  resolveInside,
  sanitizePathSegment,
} from '../../src/services/safeTargetPath';
import type { LmdbCache } from '../../src/caching/LmdbCache';
import type { WorkerPool } from '../../src/contexts/types';
import type { FileInfo, FileProcessorConfig } from '../../src/types';

type TargetPathFn = (
  format: string,
  targetDir: string,
  fileInfo: FileInfo,
  sourcePath: string,
) => string;

function fileInfoWithCamera(cameraModel: string): FileInfo {
  const date = new Date('2024-05-06T07:08:09Z');
  return {
    media: { duration: 0, frames: [] },
    fileStats: {
      hash: new SharedArrayBuffer(0),
      size: 1,
      createdAt: date,
      modifiedAt: date,
    },
    metadata: { width: 1, height: 1, cameraModel, imageDate: date },
  };
}

function isInside(root: string, candidate: string): boolean {
  const rel = relative(resolve(root), candidate);
  return rel !== '' && !rel.startsWith(`..${sep}`) && rel !== '..';
}

describe('FileTransferService target path containment', () => {
  let targetDir: string;
  let generateTargetPath: TargetPathFn;

  beforeEach(() => {
    targetDir = mkdtempSync(join(tmpdir(), 'mc-target-'));
    const service = new FileTransferService(
      {} as FileProcessorConfig,
      {} as LmdbCache,
      {} as ExifTool,
      {} as WorkerPool,
    );
    // generateTargetPath is private; the containment property is what matters.
    generateTargetPath = (
      service as unknown as { generateTargetPath: TargetPathFn }
    ).generateTargetPath.bind(service);
  });

  afterEach(() => {
    rmSync(targetDir, { recursive: true, force: true });
  });

  it.each([
    '../../../../etc/cron.d/evil',
    '..',
    '..\\..\\windows',
    '/etc/passwd',
    'Canon\u0000/../../x',
  ])('keeps a malicious EXIF camera model %j inside the target', (camera) => {
    const out = generateTargetPath(
      '{CAM}/{D.YYYY}',
      targetDir,
      fileInfoWithCamera(camera),
      '/photos/IMG_0001.jpg',
    );

    expect(isInside(targetDir, out)).toBe(true);
    expect(out.endsWith(`${sep}IMG_0001.jpg`)).toBe(true);
    // The camera value fills exactly one directory segment.
    expect(relative(targetDir, out).split(sep)).toHaveLength(3);
  });

  it('keeps a traversal inside a filename-bearing format inside the target', () => {
    const out = generateTargetPath(
      '{CAM}/{NAME}{EXT}',
      targetDir,
      fileInfoWithCamera('../..'),
      '/photos/..jpg',
    );
    expect(isInside(targetDir, out)).toBe(true);
  });

  it('still honours an ordinary camera model and date layout', () => {
    const out = generateTargetPath(
      '{CAM}/{D.YYYY}',
      targetDir,
      fileInfoWithCamera('Canon EOS R5'),
      '/photos/IMG_0001.jpg',
    );
    expect(out).toBe(join(targetDir, 'Canon EOS R5', '2024', 'IMG_0001.jpg'));
  });
});

describe('sanitizePathSegment', () => {
  it.each([
    ['..', '_'],
    ['.', '_'],
    ['', '_'],
    ['a/b', 'a_b'],
    ['a\\b', 'a_b'],
    ['nul\u0000byte', 'nul_byte'],
    ['Canon EOS R5', 'Canon EOS R5'],
  ])('%j becomes %j', (input, expected) => {
    expect(sanitizePathSegment(input)).toBe(expected);
  });
});

describe('resolveInside', () => {
  it('rejects a path that escapes the root', () => {
    expect(() => resolveInside('/target', '..', 'etc')).toThrow(
      /outside the target directory/,
    );
    expect(() => resolveInside('/target', '/etc/passwd')).toThrow();
    expect(() => resolveInside('/target')).toThrow();
  });

  it('returns the resolved path when it stays inside', () => {
    expect(resolveInside('/target', 'a', 'b.jpg')).toBe(
      resolve('/target/a/b.jpg'),
    );
  });
});
