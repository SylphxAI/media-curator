import { describe, expect, it, vi } from 'vitest';
import { ok } from '../../src/errors';
import type { FileInfo } from '../../src/types';

vi.mock('exiftool-vendored', () => ({
  ExifTool: class {},
}));

vi.mock('../../src/fileProcessor', () => ({
  processSingleFile: vi.fn(),
}));

vi.mock('../../src/organizationPlan', async () => {
  const actual = await vi.importActual<
    typeof import('../../src/organizationPlan')
  >('../../src/organizationPlan');
  return {
    ...actual,
    fingerprintFile: vi.fn(async (source: string) => ({
      size: source.length,
      modifiedAt: '2026-08-17T00:00:00.000Z',
      md5: 'a'.repeat(32),
    })),
  };
});

const { processSingleFile } = await import('../../src/fileProcessor');
const { fingerprintFile } = await import('../../src/organizationPlan');
const { FileTransferService } = await import(
  '../../src/services/FileTransferService'
);

const fileInfo: FileInfo = {
  fileStats: {
    hash: new SharedArrayBuffer(16),
    size: 10,
    createdAt: new Date('2026-08-17T00:00:00.000Z'),
    modifiedAt: new Date('2026-08-17T00:00:00.000Z'),
  },
  metadata: { width: 100, height: 100 },
  media: { duration: 0, frames: [] },
};

describe('FileTransferService.planOrganizedFiles', () => {
  it('emits explicit organize, duplicate, and error recommendations', async () => {
    vi.mocked(processSingleFile).mockResolvedValue(ok(fileInfo));
    const service = new FileTransferService(
      {} as never,
      {} as never,
      {} as never,
      {} as never,
    );
    const sourceRoot = '/synthetic/media-source';
    const destinationRoot = '/synthetic/media-destination';
    const actions = await service.planOrganizedFiles(
      {
        validFiles: [`${sourceRoot}/unique.jpg`, `${sourceRoot}/duplicate.jpg`],
        errorFiles: [`${sourceRoot}/broken.jpg`],
      },
      {
        uniqueFiles: new Set([`${sourceRoot}/unique.jpg`]),
        duplicateSets: [
          {
            bestFile: `${sourceRoot}/representative.jpg`,
            representatives: new Set([`${sourceRoot}/representative.jpg`]),
            duplicates: new Set([`${sourceRoot}/duplicate.jpg`]),
          },
        ],
      },
      destinationRoot,
      '/synthetic/media-duplicates',
      '/synthetic/media-errors',
      '{NAME}{EXT}',
    );

    expect(actions.map((action) => action.kind)).toEqual([
      'organize',
      'duplicate',
      'error',
    ]);
    expect(actions[0]?.reason).toContain('selected representative');
    expect(actions[1]?.reason).toContain('representative');
    expect(actions[1]?.representative).toBe(`${sourceRoot}/representative.jpg`);
    expect(actions[2]?.reason).toContain('processing error');
    expect(
      actions.every((action) => action.fingerprint.md5 === 'a'.repeat(32)),
    ).toBe(true);
    expect(fingerprintFile).toHaveBeenCalledTimes(3);
  });
});
