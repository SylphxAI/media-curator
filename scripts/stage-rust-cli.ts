#!/usr/bin/env bun
/**
 * Stage media-curator-cli into bin/native for npm pack (fail-closed).
 * Without this, published packages default MEDIA_CURATOR_RUST=1 but ship no binary.
 */
import { copyFileSync, chmodSync, existsSync, mkdirSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';

const root = path.resolve(import.meta.dir, '..');
const outDir = path.join(root, 'bin/native');
const outBin = path.join(outDir, 'media-curator-cli');

const candidates = [
  path.join(root, 'target/release/media-curator-cli'),
  path.join(root, 'target/debug/media-curator-cli'),
];

let src = candidates.find((c) => existsSync(c));
if (!src) {
  console.log('[stage-rust-cli] building release media-curator-cli...');
  const build = spawnSync(
    'cargo',
    ['build', '--release', '-p', 'media-curator-cli'],
    {
      cwd: root,
      stdio: 'inherit',
    },
  );
  if (build.status !== 0) {
    console.error('[stage-rust-cli] cargo build failed');
    process.exit(1);
  }
  src = path.join(root, 'target/release/media-curator-cli');
}

if (!existsSync(src)) {
  console.error('[stage-rust-cli] missing binary after build:', src);
  process.exit(1);
}

mkdirSync(outDir, { recursive: true });
copyFileSync(src, outBin);
chmodSync(outBin, 0o755);
console.log(`[stage-rust-cli] staged ${outBin}`);
