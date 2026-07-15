#!/usr/bin/env bun
/**
 * Media Curator frozen TS oracle for bounded differential parity (rej-010).
 * Slices: cli/metadata-extraction | cli/discovery-gather | cli/perceptual-hash-lsh
 */
import { createHash } from 'node:crypto';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, extname } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = join(fileURLToPath(new URL('.', import.meta.url)), '../..');
const corpusPath = join(
  repoRoot,
  'scripts/differential/fixtures/media-curator-corpus.json',
);
const corpus = JSON.parse(readFileSync(corpusPath, 'utf8')) as {
  corpusVersion: number;
  slice: string;
  cases: Array<{
    id: string;
    slice: string;
    input: Record<string, unknown>;
    output: Record<string, unknown>;
  }>;
};

const SUPPORTED = new Set([
  'jpg',
  'jpeg',
  'png',
  'gif',
  'webp',
  'heic',
  'heif',
  'tif',
  'tiff',
  'bmp',
  'raw',
  'mp4',
  'mov',
  'm4v',
  'mkv',
  'avi',
  'webm',
  'mpg',
  'mpeg',
  '3gp',
  'wmv',
  'flv',
  'divx',
]);

function popcount64(n: bigint): bigint {
  n = n - ((n >> 1n) & 0x5555555555555555n);
  n = (n & 0x3333333333333333n) + ((n >> 2n) & 0x3333333333333333n);
  n = (n + (n >> 4n)) & 0x0f0f0f0f0f0f0f0fn;
  n = n + (n >> 8n);
  n = n + (n >> 16n);
  n = n + (n >> 32n);
  return n & 0x7fn;
}

function hammingHex(hash1Hex: string, hash2Hex: string): number {
  const hash1 = Buffer.from(hash1Hex, 'hex');
  const hash2 = Buffer.from(hash2Hex, 'hex');
  const minLen = Math.min(hash1.length, hash2.length);
  const commonChunks = Math.floor(minLen / 8);
  let distance = 0n;
  for (let chunk = 0; chunk < commonChunks; chunk += 1) {
    const offset = chunk * 8;
    const a = hash1.readBigUInt64LE(offset);
    const b = hash2.readBigUInt64LE(offset);
    distance += popcount64(a ^ b);
  }
  for (let index = commonChunks * 8; index < minLen; index += 1) {
    distance += BigInt(
      (hash1[index] ^ hash2[index]).toString(2).split('1').length - 1,
    );
  }
  return Number(distance);
}

function toRepoRelative(absolutePath: string): string {
  const normalizedRoot = repoRoot.endsWith('/') ? repoRoot : `${repoRoot}/`;
  return absolutePath.startsWith(normalizedRoot)
    ? absolutePath.slice(normalizedRoot.length)
    : absolutePath;
}

function discover(sourceDirs: string[]): {
  byExtension: Record<string, string[]>;
  stats: { fileCount: number; dirCount: number };
} {
  const byExtension: Record<string, string[]> = {};
  let fileCount = 0;
  let dirCount = 0;

  function scan(dirPath: string): void {
    dirCount += 1;
    for (const entry of readdirSync(dirPath, { withFileTypes: true })) {
      const entryPath = join(dirPath, entry.name);
      if (entry.isDirectory()) {
        scan(entryPath);
      } else {
        const ext = extname(entry.name).slice(1).toLowerCase();
        if (SUPPORTED.has(ext)) {
          fileCount += 1;
          byExtension[ext] ??= [];
          byExtension[ext].push(toRepoRelative(entryPath));
        }
      }
    }
  }

  for (const source of sourceDirs) {
    scan(join(repoRoot, source));
  }
  for (const ext of Object.keys(byExtension)) {
    byExtension[ext].sort();
  }
  return { byExtension, stats: { fileCount, dirCount } };
}

function runCase(testCase: (typeof corpus.cases)[number]) {
  switch (testCase.slice) {
    case 'cli/metadata-extraction': {
      const file = join(repoRoot, testCase.input.file as string);
      const bytes = readFileSync(file);
      return {
        md5: createHash('md5').update(bytes).digest('hex'),
        size: bytes.length,
      };
    }
    case 'cli/discovery-gather': {
      const sourceDirs = testCase.input.sourceDirs as string[];
      return discover(sourceDirs);
    }
    case 'cli/perceptual-hash-lsh': {
      return {
        distance: hammingHex(
          testCase.input.hash1Hex as string,
          testCase.input.hash2Hex as string,
        ),
      };
    }
    case 'cli/deduplication-engine': {
      // Exact pHash cluster step (TS deduplicateFilesFn step 1) — oracle for Rust SSOT.
      const entries = testCase.input.entries as Array<{
        path: string;
        phashHex?: string;
      }>;
      const byHash = new Map<string, string[]>();
      const missingPhash: string[] = [];
      for (const entry of entries) {
        const hash = (entry.phashHex ?? '').trim();
        if (!hash) {
          missingPhash.push(entry.path);
          continue;
        }
        const list = byHash.get(hash) ?? [];
        list.push(entry.path);
        byHash.set(hash, list);
      }
      const clusters: Array<{ phashHex: string; paths: string[] }> = [];
      const singletons: string[] = [];
      for (const [phashHex, paths] of [...byHash.entries()].sort(([a], [b]) =>
        a.localeCompare(b),
      )) {
        paths.sort();
        if (paths.length >= 2) {
          clusters.push({ phashHex, paths });
        } else if (paths.length === 1) {
          singletons.push(paths[0]!);
        }
      }
      singletons.sort();
      missingPhash.sort();
      return { clusters, singletons, missingPhash };
    }
    default:
      throw new Error(`unsupported slice: ${testCase.slice}`);
  }
}

const cases = corpus.cases.map((testCase) => {
  const output = runCase(testCase);
  if (JSON.stringify(output) !== JSON.stringify(testCase.output)) {
    console.error(`::error::oracle drift for ${testCase.id}`);
    console.error('expected', testCase.output);
    console.error('actual', output);
    process.exit(1);
  }
  return { ...testCase, output };
});

const fixtureCorpusHash = createHash('sha256')
  .update(readFileSync(corpusPath))
  .digest('hex');
const behaviorSpecHash = createHash('sha256')
  .update(readFileSync(corpusPath))
  .update(JSON.stringify(cases))
  .digest('hex');

process.stdout.write(
  `${JSON.stringify({ corpusVersion: corpus.corpusVersion, slice: corpus.slice, fixtureCorpusHash, behaviorSpecHash, cases })}\n`,
);
