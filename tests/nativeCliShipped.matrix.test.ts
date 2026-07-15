import { existsSync, chmodSync, readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { describe, expect, it } from 'vitest';

const repoRoot = path.resolve(import.meta.dirname, '..');
const staged = path.join(repoRoot, 'bin/native/media-curator-cli');

describe('native media-curator-cli shipped in npm pack (adversarial)', () => {
  it('resolve path prefers bin/native', async () => {
    const { resolveRustCliBinary } = await import('../src/external/rustCli');
    process.env.MEDIA_CURATOR_RUST_CLI_BIN = '';
    delete process.env.MEDIA_CURATOR_RUST_CLI_BIN;
    delete process.env.MEDIA_CURATOR_RUST_BIN;
    // ensure staged exists
    if (!existsSync(staged)) {
      const r = spawnSync('bun', ['run', 'stage:rust'], {
        cwd: repoRoot,
        encoding: 'utf8',
      });
      expect(r.status).toBe(0);
    }
    expect(existsSync(staged)).toBe(true);
    const resolved = resolveRustCliBinary();
    expect(resolved).toBe(staged);
  });

  it('package files include bin', () => {
    const pkg = JSON.parse(
      readFileSync(path.join(repoRoot, 'package.json'), 'utf8'),
    );
    expect(Array.isArray(pkg.files)).toBe(true);
    expect(pkg.files.some((f: string) => String(f).includes('bin'))).toBe(true);
  });

  it('check-native-packaging passes', () => {
    if (!existsSync(staged)) {
      spawnSync('bun', ['run', 'stage:rust'], { cwd: repoRoot });
    }
    chmodSync(staged, 0o755);
    const r = spawnSync('bash', ['scripts/check-native-packaging.sh'], {
      cwd: repoRoot,
      encoding: 'utf8',
    });
    expect(r.status).toBe(0);
    expect(r.stdout).toContain('PASS');
  });

  it('check-no-ts-file-stats-backend passes', () => {
    const r = spawnSync('bash', ['scripts/check-no-ts-file-stats-backend.sh'], {
      cwd: repoRoot,
      encoding: 'utf8',
    });
    expect(r.status).toBe(0);
    expect(r.stdout).toContain('PASS');
  });
});
