import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { FileSystemError } from '../src/errors';
import { transferOrCopyFile } from '../src/transferOps';

const temporaryRoots: string[] = [];

afterEach(async () => {
  await Promise.all(
    temporaryRoots
      .splice(0)
      .map((root) => rm(root, { recursive: true, force: true })),
  );
});

async function makeRoot(): Promise<string> {
  const root = await mkdtemp(join(tmpdir(), 'media-curator-transfer-'));
  temporaryRoots.push(root);
  return root;
}

describe('transferOrCopyFile', () => {
  it('creates the target parent and copies bytes', async () => {
    const root = await makeRoot();
    const source = join(root, 'source.jpg');
    const target = join(root, 'nested', 'target.jpg');
    await writeFile(source, 'media');

    await transferOrCopyFile(source, target, true);

    await expect(readFile(source, 'utf8')).resolves.toBe('media');
    await expect(readFile(target, 'utf8')).resolves.toBe('media');
  });

  it('moves bytes and removes the source', async () => {
    const root = await makeRoot();
    const source = join(root, 'source.jpg');
    const target = join(root, 'nested', 'target.jpg');
    await writeFile(source, 'media');

    await transferOrCopyFile(source, target, false);

    await expect(readFile(target, 'utf8')).resolves.toBe('media');
    await expect(readFile(source)).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('normalizes and propagates a failed transfer', async () => {
    const root = await makeRoot();
    const source = join(root, 'missing.jpg');
    const target = join(root, 'nested', 'target.jpg');

    await expect(
      transferOrCopyFile(source, target, true),
    ).rejects.toBeInstanceOf(FileSystemError);
    await expect(readFile(target)).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('refuses to overwrite an existing target', async () => {
    const root = await makeRoot();
    const source = join(root, 'source.jpg');
    const target = join(root, 'target.jpg');
    await writeFile(source, 'new media');
    await writeFile(target, 'existing media');

    await expect(
      transferOrCopyFile(source, target, true),
    ).rejects.toBeInstanceOf(FileSystemError);
    await expect(readFile(source, 'utf8')).resolves.toBe('new media');
    await expect(readFile(target, 'utf8')).resolves.toBe('existing media');
  });

  it('refuses to overwrite an existing target when moving', async () => {
    const root = await makeRoot();
    const source = join(root, 'source.jpg');
    const target = join(root, 'target.jpg');
    await writeFile(source, 'new media');
    await writeFile(target, 'existing media');

    await expect(
      transferOrCopyFile(source, target, false),
    ).rejects.toBeInstanceOf(FileSystemError);
    await expect(readFile(source, 'utf8')).resolves.toBe('new media');
    await expect(readFile(target, 'utf8')).resolves.toBe('existing media');
  });
});
