import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const PACKAGE_ROOT = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  '../..',
);

export type RustHealthBody = {
  status: string;
  stub: boolean;
  authority?: string;
};

export type RustFileStats = {
  md5: string;
  size: number;
};

export type RustDiscoveryMap = {
  byExtension: Record<string, string[]>;
  stats: { fileCount: number; dirCount: number };
};

export function resolveRustCliBinary(): string | null {
  const override =
    process.env.MEDIA_CURATOR_RUST_CLI_BIN?.trim() ||
    process.env.MEDIA_CURATOR_RUST_BIN?.trim();
  if (override) {
    return existsSync(override) ? override : null;
  }

  const candidates = [
    path.join(PACKAGE_ROOT, 'bin/native/media-curator-cli'),
    path.join(PACKAGE_ROOT, 'target/release/media-curator-cli'),
    path.join(PACKAGE_ROOT, 'target/debug/media-curator-cli'),
  ];
  return candidates.find((candidate) => existsSync(candidate)) ?? null;
}

function runRustCli(args: string[]): string {
  const binary = resolveRustCliBinary();
  if (!binary) {
    throw new Error(
      'MEDIA_CURATOR_RUST is enabled but media-curator-cli binary was not found. Build with `cargo build -p media-curator-cli --release` (or `bun run stage:rust`) or set MEDIA_CURATOR_RUST_CLI_BIN.',
    );
  }

  const result = spawnSync(binary, args, {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  if (result.status !== 0) {
    const stderr = result.stderr?.trim() ?? '';
    throw new Error(
      `media-curator-cli ${args.join(' ')} failed (exit ${
        result.status ?? 'unknown'
      }): ${stderr || 'no stderr'}`,
    );
  }

  const stdout = result.stdout?.trim();
  if (!stdout) {
    throw new Error(
      `media-curator-cli ${args.join(' ')} returned empty stdout.`,
    );
  }
  return stdout;
}

export function healthViaRust(): RustHealthBody {
  const parsed = JSON.parse(runRustCli(['health'])) as RustHealthBody;
  if (parsed.status !== 'ok') {
    throw new Error(
      'media-curator-cli health returned unexpected status (expected ok).',
    );
  }
  return parsed;
}

export function fileStatsViaRust(filePath: string): RustFileStats {
  const parsed = JSON.parse(
    runRustCli(['file-stats', filePath]),
  ) as RustFileStats;
  if (
    typeof parsed.md5 !== 'string' ||
    typeof parsed.size !== 'number' ||
    !parsed.md5
  ) {
    throw new Error('media-curator-cli file-stats returned unexpected shape.');
  }
  return parsed;
}

export function discoverViaRust(
  sourceDirs: string[],
  concurrency: number = 4,
): RustDiscoveryMap {
  const args = ['discover', '--concurrency', String(concurrency)];
  for (const dir of sourceDirs) {
    args.push('--source', dir);
  }
  const parsed = JSON.parse(runRustCli(args)) as RustDiscoveryMap;
  if (!parsed.byExtension || !parsed.stats) {
    throw new Error('media-curator-cli discover returned unexpected shape.');
  }
  return parsed;
}

export function hammingViaRust(hash1Hex: string, hash2Hex: string): number {
  const parsed = JSON.parse(runRustCli(['hamming', hash1Hex, hash2Hex])) as {
    distance: number;
  };
  if (typeof parsed.distance !== 'number') {
    throw new Error('media-curator-cli hamming returned unexpected shape.');
  }
  return parsed.distance;
}
