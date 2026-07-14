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
};

export function rustCliDelegationEnabled(): boolean {
  const flag = process.env.MEDIA_CURATOR_RUST_CLI?.trim().toLowerCase();
  return flag === '1' || flag === 'true' || flag === 'yes';
}

export function resolveRustCliBinary(): string | null {
  const override = process.env.MEDIA_CURATOR_RUST_CLI_BIN?.trim();
  if (override) {
    return existsSync(override) ? override : null;
  }

  const candidates = [
    path.join(PACKAGE_ROOT, 'target/release/media-curator-cli'),
    path.join(PACKAGE_ROOT, 'target/debug/media-curator-cli'),
  ];
  return candidates.find((candidate) => existsSync(candidate)) ?? null;
}

export function healthViaRust(): RustHealthBody {
  const binary = resolveRustCliBinary();
  if (!binary) {
    throw new Error(
      'MEDIA_CURATOR_RUST_CLI is enabled but media-curator-cli binary was not found. Build with `cargo build -p media-curator-cli` or set MEDIA_CURATOR_RUST_CLI_BIN.',
    );
  }

  const result = spawnSync(binary, ['health'], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  if (result.status !== 0) {
    const stderr = result.stderr?.trim() ?? '';
    throw new Error(
      `media-curator-cli health failed (exit ${result.status ?? 'unknown'}): ${
        stderr || 'no stderr'
      }`,
    );
  }

  const stdout = result.stdout?.trim();
  if (!stdout) {
    throw new Error('media-curator-cli health returned empty stdout.');
  }

  const parsed = JSON.parse(stdout) as RustHealthBody;
  if (parsed.status !== 'ok' || !parsed.stub) {
    throw new Error(
      'media-curator-cli health returned unexpected S0 stub shape.',
    );
  }
  return parsed;
}
