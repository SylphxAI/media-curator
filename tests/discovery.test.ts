import { discoverFilesFn } from '../src/discovery';
import { CliReporter } from '../src/reporting/CliReporter';
import { discoverViaRust } from '../src/external/rustCli';
import { FileSystemError } from '../src/errors';
import { vi, describe, it, expect, beforeEach } from 'vitest';

// Rust owns filesystem traversal (ADR-168); discoverFilesFn only presents the
// bridge's result. The bridge is mocked here; tests/discovery.integration.test.ts
// runs the real native CLI.
vi.mock('../src/external/rustCli', () => ({
  discoverViaRust: vi.fn(),
}));

vi.mock('../src/reporting/CliReporter', () => {
  const CliReporterMock = class {
    startSpinner = vi.fn(() => {});
    stopSpinnerSuccess = vi.fn(() => {});
    logInfo = vi.fn(() => {});
    logSuccess = vi.fn(() => {});
    logError = vi.fn(() => {});
  };
  return { CliReporter: CliReporterMock };
});

describe('discoverFilesFn', () => {
  let reporter: CliReporter;

  beforeEach(() => {
    vi.resetAllMocks();
    reporter = new CliReporter(false);
  });

  it('should return an empty map when the bridge finds nothing', async () => {
    vi.mocked(discoverViaRust).mockReturnValue({
      byExtension: {},
      stats: { fileCount: 0, dirCount: 0 },
    });

    const result = await discoverFilesFn([], 1, reporter);

    expect(result.size).toBe(0);
    expect(reporter.startSpinner).toHaveBeenCalledTimes(1);
    expect(reporter.stopSpinnerSuccess).toHaveBeenCalledTimes(1);
  });

  it('should pass the source dirs and concurrency to the bridge', async () => {
    vi.mocked(discoverViaRust).mockReturnValue({
      byExtension: {},
      stats: { fileCount: 0, dirCount: 0 },
    });

    await discoverFilesFn(['/a', '/b'], 7, reporter);

    expect(discoverViaRust).toHaveBeenCalledWith(['/a', '/b'], 7);
  });

  it('should map the bridge result by extension and report the totals', async () => {
    vi.mocked(discoverViaRust).mockReturnValue({
      byExtension: {
        jpg: ['/root/a.jpg', '/root/sub/b.jpg'],
        mp4: ['/root/c.mp4'],
      },
      stats: { fileCount: 3, dirCount: 2 },
    });

    const result = await discoverFilesFn(['/root'], 1, reporter);

    expect(result.size).toBe(2);
    expect(result.get('jpg')).toEqual(['/root/a.jpg', '/root/sub/b.jpg']);
    expect(result.get('mp4')).toEqual(['/root/c.mp4']);
    expect(reporter.stopSpinnerSuccess).toHaveBeenCalledWith(
      'Discovery completed: Found 3 files in 2 directories',
    );
    expect(reporter.logSuccess).toHaveBeenCalledTimes(1);
  });

  it('should wrap a bridge failure in a FileSystemError', async () => {
    vi.mocked(discoverViaRust).mockImplementation(() => {
      throw new Error('native CLI missing');
    });

    const failure = discoverFilesFn(['/root'], 1, reporter);

    await expect(failure).rejects.toBeInstanceOf(FileSystemError);
    await expect(failure).rejects.toThrow(
      'Rust discovery failed: native CLI missing',
    );
    expect(reporter.stopSpinnerSuccess).toHaveBeenCalledWith(
      'Discovery failed',
    );
  });
});
