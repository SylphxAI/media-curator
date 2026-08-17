import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('exiftool-vendored', () => ({
  ExifTool: class ExifTool {},
}));

import { MediaComparator } from '../MediaComparator';
import { AppError, err, ok } from '../src/errors';

type ProcessResultsHarness = {
  processResults: MediaComparator['processResults'];
  selectRepresentatives: ReturnType<typeof vi.fn>;
};

describe('MediaComparator.processResults', () => {
  let comparator: ProcessResultsHarness;

  beforeEach(() => {
    comparator = Object.create(
      MediaComparator.prototype,
    ) as ProcessResultsHarness;
  });

  it('keeps every selected representative in the target set', async () => {
    comparator.selectRepresentatives = vi
      .fn()
      .mockResolvedValue(ok(['video.mp4', 'capture.jpg']));

    const result = await comparator.processResults(
      [new Set(['video.mp4', 'capture.jpg', 'duplicate.jpg'])],
      vi.fn(),
    );

    expect(result.isOk()).toBe(true);
    if (result.isOk()) {
      expect(result.value.uniqueFiles).toEqual(
        new Set(['video.mp4', 'capture.jpg']),
      );
      expect(result.value.duplicateSets[0]?.duplicates).toEqual(
        new Set(['duplicate.jpg']),
      );
    }
  });

  it('fails closed when representative selection fails', async () => {
    const selectionError = new AppError('metadata unavailable');
    comparator.selectRepresentatives = vi
      .fn()
      .mockResolvedValue(err(selectionError));

    const result = await comparator.processResults(
      [new Set(['a.jpg', 'b.jpg'])],
      vi.fn(),
    );

    expect(result.isErr()).toBe(true);
    if (result.isErr()) {
      expect(result.error).toBe(selectionError);
    }
  });
});
