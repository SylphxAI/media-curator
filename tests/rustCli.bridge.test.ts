import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { afterEach, beforeAll, describe, expect, it } from 'vitest';
import {
  healthViaRust,
  resolveRustCliBinary,
  rustCliDelegationEnabled,
} from '../src/external/rustCli';

const repoRoot = path.resolve(import.meta.dirname, '..');
const rustBinary = path.join(repoRoot, 'target/debug/media-curator-cli');

beforeAll(() => {
  if (!existsSync(rustBinary)) {
    execFileSync('cargo', ['build', '-p', 'media-curator-cli'], {
      cwd: repoRoot,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
  }
});

afterEach(() => {
  delete process.env.MEDIA_CURATOR_RUST_CLI;
  delete process.env.MEDIA_CURATOR_RUST_CLI_BIN;
});

describe('rustCli bridge (ADR-168 S0)', () => {
  it('rustCliDelegationEnabled respects MEDIA_CURATOR_RUST_CLI', () => {
    delete process.env.MEDIA_CURATOR_RUST_CLI;
    expect(rustCliDelegationEnabled()).toBe(false);

    process.env.MEDIA_CURATOR_RUST_CLI = '1';
    expect(rustCliDelegationEnabled()).toBe(true);

    process.env.MEDIA_CURATOR_RUST_CLI = 'true';
    expect(rustCliDelegationEnabled()).toBe(true);
  });

  it('resolveRustCliBinary honors MEDIA_CURATOR_RUST_CLI_BIN override', () => {
    process.env.MEDIA_CURATOR_RUST_CLI_BIN = rustBinary;
    expect(resolveRustCliBinary()).toBe(rustBinary);
  });

  it('healthViaRust returns S0 stub JSON from media-curator-cli', () => {
    process.env.MEDIA_CURATOR_RUST_CLI_BIN = rustBinary;
    const body = healthViaRust();

    expect(body.status).toBe('ok');
    expect(body.stub).toBe(true);
  });

  it('CLI --health delegates to Rust when MEDIA_CURATOR_RUST_CLI=1', () => {
    const result = spawnSync(
      'bun',
      ['run', path.join(repoRoot, 'index.ts'), '--health'],
      {
        cwd: repoRoot,
        encoding: 'utf8',
        env: {
          ...process.env,
          MEDIA_CURATOR_RUST_CLI: '1',
          MEDIA_CURATOR_RUST_CLI_BIN: rustBinary,
        },
        stdio: ['ignore', 'pipe', 'pipe'],
      },
    );

    expect(result.status).toBe(0);
    const payload = JSON.parse(result.stdout.trim()) as {
      status: string;
      stub: boolean;
    };
    expect(payload.status).toBe('ok');
    expect(payload.stub).toBe(true);
  });
});
