#!/usr/bin/env bun
/**
 * Media Curator frozen TS oracle for bounded differential parity (rej-010).
 * Slices: cli/metadata-extraction | cli/discovery-gather | cli/perceptual-hash-lsh
 *          | cli/deduplication-engine | cli/transfer-reporting | cli/cache-persistence
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
      const op = (testCase.input.op as string | undefined) ?? 'exactDup';
      if (op === 'lshKeys') {
        const phashHex = String(testCase.input.phashHex ?? '').trim();
        if (phashHex.length !== 16 || !/^[0-9a-fA-F]+$/.test(phashHex)) {
          return { keys: null };
        }
        return {
          keys: [
            phashHex.slice(0, 4),
            phashHex.slice(4, 8),
            phashHex.slice(8, 12),
            phashHex.slice(12, 16),
          ],
        };
      }
      if (op === 'adaptiveThreshold') {
        const d1 = Number(testCase.input.duration1 ?? 0);
        const d2 = Number(testCase.input.duration2 ?? 0);
        const img = Number(testCase.input.imageSimilarityThreshold);
        const imgVid = Number(testCase.input.imageVideoSimilarityThreshold);
        const vid = Number(testCase.input.videoSimilarityThreshold);
        const isImage1 = d1 === 0;
        const isImage2 = d2 === 0;
        let threshold: number;
        if (isImage1 && isImage2) threshold = img;
        else if (isImage1 || isImage2) threshold = imgVid;
        else threshold = vid;
        return { threshold };
      }
      if (op === 'mergeClusters') {
        const clusters = testCase.input.clusters as string[][];
        // TS mergeAndDeduplicateClusters semantics via union-find for stable oracle.
        const parent = new Map<string, string>();
        const find = (x: string): string => {
          const p = parent.get(x) ?? x;
          if (p === x) return p;
          const root = find(p);
          parent.set(x, root);
          return root;
        };
        const union = (a: string, b: string) => {
          const ra = find(a);
          const rb = find(b);
          if (ra !== rb) {
            if (ra < rb) parent.set(rb, ra);
            else parent.set(ra, rb);
          }
        };
        for (const cluster of clusters) {
          if (cluster.length === 0) continue;
          for (const el of cluster) parent.set(el, parent.get(el) ?? el);
          for (let i = 1; i < cluster.length; i++)
            union(cluster[0]!, cluster[i]!);
        }
        const groups = new Map<string, string[]>();
        for (const el of parent.keys()) {
          const root = find(el);
          const list = groups.get(root) ?? [];
          list.push(el);
          groups.set(root, list);
        }
        const merged = [...groups.values()].map((g) => g.sort());
        merged.sort((a, b) => a[0]!.localeCompare(b[0]!));
        return { clusters: merged };
      }
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
    case 'cli/transfer-reporting': {
      const uniqueFiles = (testCase.input.uniqueFiles as string[]) ?? [];
      const duplicateSets =
        (testCase.input.duplicateSets as Array<{
          bestFile: string;
          duplicates: string[];
        }>) ?? [];
      const errorFiles = (testCase.input.errorFiles as string[]) ?? [];
      const hasDuplicateDir = Boolean(testCase.input.hasDuplicateDir);
      const hasErrorDir = Boolean(testCase.input.hasErrorDir);
      const basename = (path: string) => {
        const parts = path.split(/[/\\]/);
        return parts[parts.length - 1] ?? path;
      };
      const stem = (path: string) => {
        const base = basename(path);
        const i = base.lastIndexOf('.');
        return i > 0 ? base.slice(0, i) : base;
      };
      type Action = {
        sourcePath: string;
        bucket: string;
        relativeKey: string;
      };
      const actions: Action[] = [];
      const seen = new Set<string>();
      for (const path of uniqueFiles) {
        if (seen.has(path)) continue;
        seen.add(path);
        actions.push({
          sourcePath: path,
          bucket: 'target',
          relativeKey: basename(path),
        });
      }
      for (const set of duplicateSets) {
        const folder = stem(set.bestFile);
        for (const dup of set.duplicates) {
          if (seen.has(dup)) continue;
          seen.add(dup);
          if (hasDuplicateDir) {
            actions.push({
              sourcePath: dup,
              bucket: 'duplicate',
              relativeKey: `${folder}/${basename(dup)}`,
            });
          } else {
            actions.push({ sourcePath: dup, bucket: 'skip', relativeKey: '' });
          }
        }
      }
      for (const path of errorFiles) {
        if (seen.has(path)) continue;
        seen.add(path);
        if (hasErrorDir) {
          actions.push({
            sourcePath: path,
            bucket: 'error',
            relativeKey: basename(path),
          });
        } else {
          actions.push({ sourcePath: path, bucket: 'skip', relativeKey: '' });
        }
      }
      actions.sort((a, b) => a.sourcePath.localeCompare(b.sourcePath));
      const count = (bucket: string) =>
        actions.filter((a) => a.bucket === bucket).length;
      return {
        targetCount: count('target'),
        duplicateCount: count('duplicate'),
        errorCount: count('error'),
        skipCount: count('skip'),
        actions,
      };
    }
    case 'cli/cache-persistence': {
      if (testCase.input.markers) {
        return { msgpack: 0, sharedArrayBuffer: 1, date: 2 };
      }
      if ('phashHex' in testCase.input) {
        const h = testCase.input.phashHex as string | null;
        if (h && h.length === 16 && /^[0-9a-fA-F]+$/.test(h)) {
          return {
            lshKeys: [
              h.slice(0, 4),
              h.slice(4, 8),
              h.slice(8, 12),
              h.slice(12, 16),
            ],
          };
        }
        return { lshKeys: [null, null, null, null] };
      }
      const jobName = String(testCase.input.jobName ?? '');
      const hashKey = String(testCase.input.hashKey ?? '');
      return {
        rootDir: '.mediadb',
        metadataPath: '.mediadb/metadata.sqlite',
        resultsDb: `${jobName}_results`,
        configDb: `${jobName}_config`,
        mutexKey: `${jobName}:${hashKey}`,
      };
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
