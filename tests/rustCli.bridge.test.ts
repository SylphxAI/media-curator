import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { afterEach, beforeAll, describe, expect, it } from 'vitest';
import {
  healthViaRust,
  resolveRustCliBinary,
  rustCliDelegationEnabled,
  fileStatsViaRust,
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
  delete process.env.MEDIA_CURATOR_RUST;
  delete process.env.MEDIA_CURATOR_RUST_BIN;
});

describe('rustCli bridge (ADR-168 production authority)', () => {
  it('rustCliDelegationEnabled defaults to true when unset', () => {
    delete process.env.MEDIA_CURATOR_RUST;
    delete process.env.MEDIA_CURATOR_RUST_CLI;
    expect(rustCliDelegationEnabled()).toBe(true);

    process.env.MEDIA_CURATOR_RUST = 'ts';
    expect(rustCliDelegationEnabled()).toBe(false);

    process.env.MEDIA_CURATOR_RUST = '1';
    expect(rustCliDelegationEnabled()).toBe(true);
  });

  it('resolveRustCliBinary honors MEDIA_CURATOR_RUST_CLI_BIN override', () => {
    process.env.MEDIA_CURATOR_RUST_CLI_BIN = rustBinary;
    expect(resolveRustCliBinary()).toBe(rustBinary);
  });

  it('healthViaRust returns rust authority JSON', () => {
    process.env.MEDIA_CURATOR_RUST_CLI_BIN = rustBinary;
    const body = healthViaRust();

    expect(body.status).toBe('ok');
    expect(body.stub).toBe(false);
    expect(body.authority).toBe('rust');
  });

  it('fileStatsViaRust returns md5+size for fixture', () => {
    process.env.MEDIA_CURATOR_RUST_CLI_BIN = rustBinary;
    const stats = fileStatsViaRust(
      path.join(repoRoot, 'fixtures/file-stats/sample-a.txt'),
    );
    expect(stats.size).toBeGreaterThan(0);
    expect(stats.md5).toMatch(/^[a-f0-9]{32}$/);
  });

  it('CLI --health delegates to Rust by default', () => {
    const result = spawnSync(
      'bun',
      ['run', path.join(repoRoot, 'index.ts'), '--health'],
      {
        cwd: repoRoot,
        encoding: 'utf8',
        env: {
          ...process.env,
          MEDIA_CURATOR_RUST_CLI_BIN: rustBinary,
        },
        stdio: ['ignore', 'pipe', 'pipe'],
      },
    );

    expect(result.status).toBe(0);
    const payload = JSON.parse(result.stdout.trim()) as {
      status: string;
      stub: boolean;
      authority: string;
    };
    expect(payload.status).toBe('ok');
    expect(payload.authority).toBe('rust');
  });
});
