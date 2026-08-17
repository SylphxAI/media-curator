import {
  mkdir,
  mkdtemp,
  readFile,
  rm,
  stat,
  writeFile,
} from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import {
  applyOrganizationPlan,
  createOrganizationPlan,
  fingerprintFile,
  readOrganizationPlan,
  writeOrganizationPlan,
} from '../src/organizationPlan.js';

const temporaryRoots: string[] = [];

afterEach(async () => {
  await Promise.all(
    temporaryRoots
      .splice(0)
      .map((root) => rm(root, { recursive: true, force: true })),
  );
});

async function makePlan(operation: 'copy' | 'move' = 'copy') {
  const root = await mkdtemp(join(tmpdir(), 'media-curator-plan-'));
  temporaryRoots.push(root);
  const sourceRoot = join(root, 'source');
  const destinationRoot = join(root, 'organized');
  const source = join(sourceRoot, 'photo.jpg');
  const target = join(destinationRoot, 'photo.jpg');
  await mkdir(sourceRoot, { recursive: true });
  await writeFile(source, 'synthetic media bytes');
  const fingerprint = await fingerprintFile(source);
  const plan = createOrganizationPlan({
    sourceRoots: [sourceRoot],
    destinationRoot,
    format: '{NAME}{EXT}',
    operation,
    actions: [
      {
        id: 'organize-000001',
        kind: 'organize',
        source,
        target,
        reason: 'selected representative for the organized library',
        fingerprint,
      },
    ],
  });
  return { root, source, target, plan };
}

async function approvePlan(planPath: string): Promise<void> {
  const plan = JSON.parse(await readFile(planPath, 'utf8')) as {
    review: { approved: boolean; notes: string };
  };
  plan.review = {
    approved: true,
    notes: 'Synthetic fixture reviewed by the operator.',
  };
  await writeFile(planPath, `${JSON.stringify(plan, null, 2)}\n`);
}

describe('organization plan journey', () => {
  it('exports a review gate and resumes a completed copy idempotently', async () => {
    const { root, source, target, plan } = await makePlan();
    const planPath = join(root, 'review', 'organization-plan.json');
    await writeOrganizationPlan(planPath, plan);

    await expect(applyOrganizationPlan(planPath)).rejects.toThrow(
      'not approved',
    );
    await approvePlan(planPath);

    const applied = await applyOrganizationPlan(planPath);
    expect(applied.completedActions).toBe(1);
    expect(applied.totalActions).toBe(1);
    expect(applied.resumed).toBe(false);
    await expect(readFile(source, 'utf8')).resolves.toBe(
      'synthetic media bytes',
    );
    await expect(readFile(target, 'utf8')).resolves.toBe(
      'synthetic media bytes',
    );

    const resumed = await applyOrganizationPlan(planPath);
    expect(resumed.completedActions).toBe(1);
    expect(resumed.resumed).toBe(true);
    await expect(readFile(target, 'utf8')).resolves.toBe(
      'synthetic media bytes',
    );
    const journal = JSON.parse(
      await readFile(`${planPath}.journal.json`, 'utf8'),
    ) as { actions: Record<string, { status: string }> };
    expect(journal.actions['organize-000001']?.status).toBe('completed');
  });

  it('recovers a move after the destination was created before interruption', async () => {
    const { root, source, target, plan } = await makePlan('move');
    const planPath = join(root, 'organization-plan.json');
    await writeOrganizationPlan(planPath, plan);
    await approvePlan(planPath);
    await mkdir(join(root, 'organized'), { recursive: true });
    await writeFile(target, 'synthetic media bytes');

    const result = await applyOrganizationPlan(planPath);
    expect(result.completedActions).toBe(1);
    await expect(stat(source)).rejects.toMatchObject({ code: 'ENOENT' });
    await expect(readFile(target, 'utf8')).resolves.toBe(
      'synthetic media bytes',
    );
  });

  it('recovers a move after the source was removed before the journal was written', async () => {
    const { root, source, target, plan } = await makePlan('move');
    const planPath = join(root, 'source-removed-plan.json');
    await writeOrganizationPlan(planPath, plan);
    await approvePlan(planPath);
    await mkdir(join(root, 'organized'), { recursive: true });
    await writeFile(target, 'synthetic media bytes');
    await rm(source);

    const result = await applyOrganizationPlan(planPath);
    expect(result.completedActions).toBe(1);
    await expect(readFile(target, 'utf8')).resolves.toBe(
      'synthetic media bytes',
    );
  });

  it('fails closed when a reviewed target conflicts or leaves its root', async () => {
    const { root, source, target, plan } = await makePlan();
    const conflictPath = join(root, 'conflict-plan.json');
    await writeOrganizationPlan(conflictPath, plan);
    await approvePlan(conflictPath);
    await mkdir(join(root, 'organized'), { recursive: true });
    await writeFile(target, 'operator file');

    await expect(applyOrganizationPlan(conflictPath)).rejects.toThrow(
      'target fingerprint differs',
    );
    await expect(readFile(source, 'utf8')).resolves.toBe(
      'synthetic media bytes',
    );
    await expect(readFile(target, 'utf8')).resolves.toBe('operator file');

    const unsafePlan = createOrganizationPlan({
      ...plan,
      actions: [
        {
          ...plan.actions[0]!,
          target: join(root, '..', 'outside-media-curator-plan.jpg'),
        },
      ],
    });
    const unsafePath = join(root, 'unsafe-plan.json');
    await writeOrganizationPlan(unsafePath, unsafePlan);
    await approvePlan(unsafePath);
    await expect(applyOrganizationPlan(unsafePath)).rejects.toThrow(
      'leaves its declared target root',
    );
  });

  it('rejects a source that changed after human review and records the pending action', async () => {
    const { root, source, target, plan } = await makePlan();
    const planPath = join(root, 'changed-source-plan.json');
    await writeOrganizationPlan(planPath, plan);
    await approvePlan(planPath);
    await writeFile(source, 'changed after review');

    await expect(applyOrganizationPlan(planPath)).rejects.toThrow(
      'source fingerprint differs',
    );
    await expect(stat(target)).rejects.toMatchObject({ code: 'ENOENT' });
    const journal = JSON.parse(
      await readFile(`${planPath}.journal.json`, 'utf8'),
    ) as {
      actions: Record<string, { status: string; lastError?: string }>;
    };
    expect(journal.actions['organize-000001']?.status).toBe('pending');
    expect(journal.actions['organize-000001']?.lastError).toContain(
      'source fingerprint differs',
    );
  });

  it('rejects an overwritten plan file so reviewed bytes are not replaced silently', async () => {
    const { root, plan } = await makePlan();
    const planPath = join(root, 'organization-plan.json');
    await writeOrganizationPlan(planPath, plan);
    await expect(writeOrganizationPlan(planPath, plan)).rejects.toThrow(
      'Could not write organization plan',
    );
    const loaded = await readOrganizationPlan(planPath);
    expect(loaded.planId).toBe(plan.planId);
    expect(loaded.review.approved).toBe(false);
  });
});
