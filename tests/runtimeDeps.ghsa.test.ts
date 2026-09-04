import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import sharp from 'sharp';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { createExifTool, readExifTags } from '../src/external/ExifToolService';
import {
  createSharpInstance,
  grayscaleImage,
  imageToBuffer,
  resizeImage,
} from '../src/external/SharpServiceWrapper';
import { ExternalToolError } from '../src/errors';
import type { ExifTool } from 'exiftool-vendored';

function parseVersion(version: string): number[] {
  return version.split('.').map((part) => {
    const n = Number.parseInt(part, 10);
    return Number.isFinite(n) ? n : 0;
  });
}

function versionGte(actual: string, floor: string): boolean {
  const actualParts = parseVersion(actual);
  const floorParts = parseVersion(floor);
  const len = Math.max(actualParts.length, floorParts.length);
  for (let i = 0; i < len; i += 1) {
    const a = actualParts[i] ?? 0;
    const b = floorParts[i] ?? 0;
    if (a > b) {
      return true;
    }
    if (a < b) {
      return false;
    }
  }
  return true;
}

async function makeJpegBuffer(): Promise<Buffer> {
  return sharp({
    create: {
      width: 16,
      height: 16,
      channels: 3,
      background: { r: 32, g: 64, b: 128 },
    },
  })
    .jpeg()
    .toBuffer();
}

describe('GHSA-f88m-g3jw-g9cj sharp/libvips', () => {
  it('loads patched sharp/libvips and still resizes JPEG through the product wrapper', async () => {
    expect(sharp.versions.sharp).toBeTruthy();
    expect(versionGte(sharp.versions.sharp, '0.35.0')).toBe(true);
    expect(sharp.versions.vips).toBeTruthy();
    expect(versionGte(sharp.versions.vips, '8.18.3')).toBe(true);

    const jpeg = await makeJpegBuffer();
    const processed = grayscaleImage(
      resizeImage(createSharpInstance(jpeg), 8, 8),
    );
    const bufferResult = await imageToBuffer(processed.raw());
    expect(bufferResult.isOk()).toBe(true);
    expect(bufferResult._unsafeUnwrap().byteLength).toBeGreaterThan(0);
  });
});

describe('GHSA-cw26-7653-2rp5 exiftool-vendored argument injection', () => {
  let exifTool: ExifTool;
  let workDir: string;

  beforeAll(async () => {
    workDir = await mkdtemp(join(tmpdir(), 'media-curator-ghsa-'));
    exifTool = createExifTool(1);
  });

  afterAll(async () => {
    await exifTool.end();
    await rm(workDir, { recursive: true, force: true });
  });

  it('rejects a newline in the read path before ExifTool -stay_open interpolation', async () => {
    const result = await readExifTags(join(workDir, 'bad\nname.jpg'), exifTool);
    expect(result.isErr()).toBe(true);
    const error = result._unsafeUnwrapErr();
    expect(error).toBeInstanceOf(ExternalToolError);
    expect(error.message).toMatch(/control character/);
  });

  it('still reads tags from a JPEG the product wrapper can process', async () => {
    const filePath = join(workDir, 'ok.jpg');
    await writeFile(filePath, await makeJpegBuffer());
    const result = await readExifTags(filePath, exifTool);
    expect(result.isOk()).toBe(true);
    const tags = result._unsafeUnwrap();
    expect(tags.FileType).toBe('JPEG');
    expect(tags.ImageWidth).toBe(16);
    expect(tags.ImageHeight).toBe(16);
  });
});
