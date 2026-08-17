import { createHash } from 'node:crypto';
import {
  mkdir,
  readFile,
  rename,
  stat,
  unlink,
  writeFile,
} from 'node:fs/promises';
import { dirname, isAbsolute, relative, resolve } from 'node:path';
import { fileStatsViaRust } from './external/rustCli.js';
import { FileSystemError, ValidationError } from './errors.js';
import { transferOrCopyFile } from './transferOps.js';

export const ORGANIZATION_PLAN_SCHEMA_VERSION = 1 as const;

export type OrganizationPlanOperation = 'copy' | 'move';
export type OrganizationPlanActionKind = 'organize' | 'duplicate' | 'error';

export interface OrganizationPlanFingerprint {
  size: number;
  modifiedAt: string;
  md5: string;
}

export interface OrganizationPlanAction {
  id: string;
  kind: OrganizationPlanActionKind;
  source: string;
  target: string;
  reason: string;
  representative?: string;
  fingerprint: OrganizationPlanFingerprint;
}

export interface OrganizationPlanSummary {
  total: number;
  organize: number;
  duplicate: number;
  error: number;
}

export interface OrganizationPlan {
  schemaVersion: typeof ORGANIZATION_PLAN_SCHEMA_VERSION;
  planId: string;
  createdAt: string;
  review: {
    approved: boolean;
    notes: string;
  };
  operation: OrganizationPlanOperation;
  sourceRoots: string[];
  destinationRoot: string;
  duplicateRoot?: string;
  errorRoot?: string;
  format: string;
  actions: OrganizationPlanAction[];
  summary: OrganizationPlanSummary;
}

export interface OrganizationPlanDraft {
  sourceRoots: string[];
  destinationRoot: string;
  duplicateRoot?: string;
  errorRoot?: string;
  format: string;
  operation: OrganizationPlanOperation;
  actions: OrganizationPlanAction[];
}

interface OrganizationPlanJournalAction {
  status: 'pending' | 'completed';
  attempts: number;
  completedAt?: string;
  lastError?: string;
}

interface OrganizationPlanJournal {
  schemaVersion: typeof ORGANIZATION_PLAN_SCHEMA_VERSION;
  planId: string;
  createdAt: string;
  updatedAt: string;
  actions: Record<string, OrganizationPlanJournalAction>;
}

export interface ApplyOrganizationPlanResult {
  planId: string;
  journalPath: string;
  completedActions: number;
  totalActions: number;
  resumed: boolean;
}

function canonicalPlanPayload(plan: {
  schemaVersion: typeof ORGANIZATION_PLAN_SCHEMA_VERSION;
  operation: OrganizationPlanOperation;
  sourceRoots: string[];
  destinationRoot: string;
  duplicateRoot?: string;
  errorRoot?: string;
  format: string;
  actions: OrganizationPlanAction[];
}): string {
  return JSON.stringify({
    schemaVersion: plan.schemaVersion,
    operation: plan.operation,
    sourceRoots: plan.sourceRoots,
    destinationRoot: plan.destinationRoot,
    duplicateRoot: plan.duplicateRoot ?? null,
    errorRoot: plan.errorRoot ?? null,
    format: plan.format,
    actions: plan.actions,
  });
}

function calculatePlanId(plan: {
  schemaVersion: typeof ORGANIZATION_PLAN_SCHEMA_VERSION;
  operation: OrganizationPlanOperation;
  sourceRoots: string[];
  destinationRoot: string;
  duplicateRoot?: string;
  errorRoot?: string;
  format: string;
  actions: OrganizationPlanAction[];
}): string {
  return createHash('sha256').update(canonicalPlanPayload(plan)).digest('hex');
}

function normalizePath(value: string, field: string): string {
  if (!value.trim()) {
    throw new ValidationError(`${field} must not be empty`, {
      context: { validationDetails: { field } },
    });
  }
  return resolve(value);
}

function summarizeActions(
  actions: OrganizationPlanAction[],
): OrganizationPlanSummary {
  const summary: OrganizationPlanSummary = {
    total: actions.length,
    organize: 0,
    duplicate: 0,
    error: 0,
  };
  for (const action of actions) {
    summary[action.kind] += 1;
  }
  return summary;
}

function validateActionSet(actions: OrganizationPlanAction[]): void {
  const ids = new Set<string>();
  const sources = new Set<string>();
  for (const action of actions) {
    if (
      !action ||
      typeof action.id !== 'string' ||
      (action.kind !== 'organize' &&
        action.kind !== 'duplicate' &&
        action.kind !== 'error') ||
      typeof action.source !== 'string' ||
      typeof action.target !== 'string' ||
      typeof action.reason !== 'string' ||
      !action.fingerprint ||
      typeof action.fingerprint !== 'object'
    ) {
      throw new ValidationError('Organization plan contains an invalid action');
    }
    if (ids.has(action.id)) {
      throw new ValidationError(
        `Organization plan repeats action id ${action.id}`,
      );
    }
    ids.add(action.id);
    if (sources.has(action.source)) {
      throw new ValidationError(
        `Organization plan repeats source file ${action.source}`,
      );
    }
    sources.add(action.source);
    if (!isAbsolute(action.source) || !isAbsolute(action.target)) {
      throw new ValidationError(
        `Organization plan action ${action.id} must use absolute paths`,
      );
    }
    if (!action.reason.trim()) {
      throw new ValidationError(
        `Organization plan action ${action.id} must explain its recommendation`,
      );
    }
    if (
      !Number.isSafeInteger(action.fingerprint.size) ||
      action.fingerprint.size < 0 ||
      !/^[a-f0-9]{32}$/i.test(action.fingerprint.md5) ||
      Number.isNaN(Date.parse(action.fingerprint.modifiedAt))
    ) {
      throw new ValidationError(
        `Organization plan action ${action.id} has an invalid source fingerprint`,
      );
    }
    if (action.representative && !isAbsolute(action.representative)) {
      throw new ValidationError(
        `Organization plan action ${action.id} has a non-absolute representative`,
      );
    }
  }
}

export function createOrganizationPlan(
  draft: OrganizationPlanDraft,
): OrganizationPlan {
  const sourceRoots = draft.sourceRoots.map((root) =>
    normalizePath(root, 'source root'),
  );
  const destinationRoot = normalizePath(
    draft.destinationRoot,
    'destination root',
  );
  const duplicateRoot = draft.duplicateRoot
    ? normalizePath(draft.duplicateRoot, 'duplicate root')
    : undefined;
  const errorRoot = draft.errorRoot
    ? normalizePath(draft.errorRoot, 'error root')
    : undefined;
  if (!draft.format.trim()) {
    throw new ValidationError('Organization plan format must not be empty');
  }

  const actions = draft.actions.map((action) => {
    const normalized = {
      ...action,
      source: normalizePath(action.source, `action ${action.id} source`),
      target: normalizePath(action.target, `action ${action.id} target`),
      fingerprint: {
        ...action.fingerprint,
        md5: action.fingerprint.md5.toLowerCase(),
      },
    };
    if (action.representative) {
      normalized.representative = normalizePath(
        action.representative,
        `action ${action.id} representative`,
      );
    } else {
      delete normalized.representative;
    }
    return normalized;
  });
  validateActionSet(actions);

  const base: {
    schemaVersion: typeof ORGANIZATION_PLAN_SCHEMA_VERSION;
    operation: OrganizationPlanOperation;
    sourceRoots: string[];
    destinationRoot: string;
    duplicateRoot?: string;
    errorRoot?: string;
    format: string;
    actions: OrganizationPlanAction[];
  } = {
    schemaVersion: ORGANIZATION_PLAN_SCHEMA_VERSION,
    operation: draft.operation,
    sourceRoots,
    destinationRoot,
    format: draft.format,
    actions,
  };
  if (duplicateRoot) base.duplicateRoot = duplicateRoot;
  if (errorRoot) base.errorRoot = errorRoot;

  return {
    ...base,
    planId: calculatePlanId(base),
    createdAt: new Date().toISOString(),
    review: { approved: false, notes: '' },
    summary: summarizeActions(actions),
  };
}

export function validateOrganizationPlan(input: unknown): OrganizationPlan {
  if (!input || typeof input !== 'object') {
    throw new ValidationError('Organization plan must be a JSON object');
  }
  const plan = input as Partial<OrganizationPlan>;
  if (plan.schemaVersion !== ORGANIZATION_PLAN_SCHEMA_VERSION) {
    throw new ValidationError(
      `Unsupported organization plan schema version: ${String(plan.schemaVersion)}`,
    );
  }
  if (!plan.planId || !/^[a-f0-9]{64}$/.test(plan.planId)) {
    throw new ValidationError('Organization plan has an invalid planId');
  }
  if (plan.operation !== 'copy' && plan.operation !== 'move') {
    throw new ValidationError(
      'Organization plan operation must be copy or move',
    );
  }
  if (!Array.isArray(plan.sourceRoots) || plan.sourceRoots.length === 0) {
    throw new ValidationError('Organization plan must declare source roots');
  }
  if (!plan.destinationRoot || !isAbsolute(plan.destinationRoot)) {
    throw new ValidationError(
      'Organization plan destinationRoot must be an absolute path',
    );
  }
  if (
    (plan.duplicateRoot !== undefined && !isAbsolute(plan.duplicateRoot)) ||
    (plan.errorRoot !== undefined && !isAbsolute(plan.errorRoot))
  ) {
    throw new ValidationError(
      'Organization plan optional roots must be absolute paths',
    );
  }
  if (
    !plan.sourceRoots.every(
      (root) => typeof root === 'string' && isAbsolute(root),
    )
  ) {
    throw new ValidationError(
      'Organization plan source roots must be absolute paths',
    );
  }
  if (typeof plan.format !== 'string' || !plan.format.trim()) {
    throw new ValidationError('Organization plan format must not be empty');
  }
  if (!plan.review || typeof plan.review.approved !== 'boolean') {
    throw new ValidationError(
      'Organization plan review.approved must be a boolean',
    );
  }
  if (typeof plan.review.notes !== 'string') {
    throw new ValidationError(
      'Organization plan review.notes must be a string',
    );
  }
  if (!Array.isArray(plan.actions)) {
    throw new ValidationError('Organization plan actions must be an array');
  }
  validateActionSet(plan.actions);

  const expectedSummary = summarizeActions(plan.actions);
  if (JSON.stringify(plan.summary) !== JSON.stringify(expectedSummary)) {
    throw new ValidationError(
      'Organization plan summary does not match actions',
    );
  }

  const base: {
    schemaVersion: typeof ORGANIZATION_PLAN_SCHEMA_VERSION;
    operation: OrganizationPlanOperation;
    sourceRoots: string[];
    destinationRoot: string;
    duplicateRoot?: string;
    errorRoot?: string;
    format: string;
    actions: OrganizationPlanAction[];
  } = {
    schemaVersion: plan.schemaVersion,
    operation: plan.operation,
    sourceRoots: plan.sourceRoots,
    destinationRoot: plan.destinationRoot,
    format: plan.format,
    actions: plan.actions,
  };
  if (plan.duplicateRoot) base.duplicateRoot = plan.duplicateRoot;
  if (plan.errorRoot) base.errorRoot = plan.errorRoot;
  if (calculatePlanId(base) !== plan.planId) {
    throw new ValidationError(
      'Organization plan contents do not match planId; regenerate the plan instead of editing actions',
    );
  }

  return plan as OrganizationPlan;
}

export async function writeOrganizationPlan(
  planPath: string,
  plan: OrganizationPlan,
): Promise<string> {
  const resolvedPath = resolve(planPath);
  await mkdir(dirname(resolvedPath), { recursive: true });
  try {
    await writeFile(resolvedPath, `${JSON.stringify(plan, null, 2)}\n`, {
      encoding: 'utf8',
      flag: 'wx',
      mode: 0o600,
    });
  } catch (error) {
    throw new FileSystemError(
      `Could not write organization plan ${resolvedPath}: ${error instanceof Error ? error.message : String(error)}`,
      { cause: error, context: { path: resolvedPath, operation: 'writePlan' } },
    );
  }
  return resolvedPath;
}

export async function readOrganizationPlan(
  planPath: string,
): Promise<OrganizationPlan> {
  const resolvedPath = resolve(planPath);
  let raw: string;
  try {
    raw = await readFile(resolvedPath, 'utf8');
  } catch (error) {
    throw new FileSystemError(
      `Could not read organization plan ${resolvedPath}: ${error instanceof Error ? error.message : String(error)}`,
      { cause: error, context: { path: resolvedPath, operation: 'readPlan' } },
    );
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (error) {
    throw new ValidationError(
      `Organization plan ${resolvedPath} is not valid JSON`,
      { cause: error, context: { validationDetails: { path: resolvedPath } } },
    );
  }
  return validateOrganizationPlan(parsed);
}

export async function fingerprintFile(
  filePath: string,
): Promise<OrganizationPlanFingerprint> {
  const resolvedPath = resolve(filePath);
  try {
    const fileStat = await stat(resolvedPath);
    if (!fileStat.isFile()) {
      throw new Error('path is not a regular file');
    }
    const rustStats = fileStatsViaRust(resolvedPath);
    if (rustStats.size !== fileStat.size) {
      throw new Error(
        `size changed during fingerprinting (Rust=${rustStats.size}, fs=${fileStat.size})`,
      );
    }
    return {
      size: rustStats.size,
      modifiedAt: fileStat.mtime.toISOString(),
      md5: rustStats.md5.toLowerCase(),
    };
  } catch (error) {
    if (error instanceof FileSystemError) throw error;
    throw new FileSystemError(
      `Could not fingerprint ${resolvedPath}: ${error instanceof Error ? error.message : String(error)}`,
      {
        cause: error,
        context: { path: resolvedPath, operation: 'fingerprint' },
      },
    );
  }
}

function isWithinRoot(root: string, candidate: string): boolean {
  const relativePath = relative(resolve(root), resolve(candidate));
  return (
    relativePath !== '' &&
    !relativePath.startsWith('..') &&
    !isAbsolute(relativePath)
  );
}

function assertActionRoots(plan: OrganizationPlan): void {
  for (const action of plan.actions) {
    if (!plan.sourceRoots.some((root) => isWithinRoot(root, action.source))) {
      throw new ValidationError(
        `Organization plan action ${action.id} leaves the declared source roots`,
      );
    }
    const targetRoot =
      action.kind === 'organize'
        ? plan.destinationRoot
        : action.kind === 'duplicate'
          ? plan.duplicateRoot
          : plan.errorRoot;
    if (!targetRoot) {
      throw new ValidationError(
        `Organization plan action ${action.id} has no declared target root`,
      );
    }
    if (!isWithinRoot(targetRoot, action.target)) {
      throw new ValidationError(
        `Organization plan action ${action.id} leaves its declared target root`,
      );
    }
    if (
      action.representative &&
      !plan.sourceRoots.some((root) =>
        isWithinRoot(root, action.representative!),
      )
    ) {
      throw new ValidationError(
        `Organization plan action ${action.id} has an out-of-root representative`,
      );
    }
  }
}

function newJournal(plan: OrganizationPlan): OrganizationPlanJournal {
  const now = new Date().toISOString();
  return {
    schemaVersion: ORGANIZATION_PLAN_SCHEMA_VERSION,
    planId: plan.planId,
    createdAt: now,
    updatedAt: now,
    actions: Object.fromEntries(
      plan.actions.map((action) => [
        action.id,
        {
          status: 'pending',
          attempts: 0,
        } satisfies OrganizationPlanJournalAction,
      ]),
    ),
  };
}

async function readJournal(
  journalPath: string,
  plan: OrganizationPlan,
): Promise<{ journal: OrganizationPlanJournal; existed: boolean }> {
  try {
    const raw = await readFile(journalPath, 'utf8');
    let parsed: unknown;
    try {
      parsed = JSON.parse(raw);
    } catch (error) {
      throw new ValidationError(
        `Apply journal ${journalPath} is not valid JSON`,
        {
          cause: error,
        },
      );
    }
    if (!parsed || typeof parsed !== 'object') {
      throw new ValidationError(
        `Apply journal ${journalPath} must be an object`,
      );
    }
    const journal = parsed as Partial<OrganizationPlanJournal>;
    if (
      journal.schemaVersion !== ORGANIZATION_PLAN_SCHEMA_VERSION ||
      journal.planId !== plan.planId ||
      !journal.actions ||
      typeof journal.actions !== 'object'
    ) {
      throw new ValidationError(
        `Apply journal ${journalPath} does not belong to plan ${plan.planId}`,
      );
    }
    const actionIds = new Set(plan.actions.map((action) => action.id));
    for (const [id, record] of Object.entries(journal.actions)) {
      if (!actionIds.has(id) || !record || typeof record !== 'object') {
        throw new ValidationError(
          `Apply journal ${journalPath} has an unknown action`,
        );
      }
      if (
        (record as OrganizationPlanJournalAction).status !== 'pending' &&
        (record as OrganizationPlanJournalAction).status !== 'completed'
      ) {
        throw new ValidationError(
          `Apply journal ${journalPath} has an invalid status`,
        );
      }
    }
    return { journal: journal as OrganizationPlanJournal, existed: true };
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      return { journal: newJournal(plan), existed: false };
    }
    throw error;
  }
}

async function persistJournal(
  journalPath: string,
  journal: OrganizationPlanJournal,
): Promise<void> {
  await mkdir(dirname(journalPath), { recursive: true });
  const tempPath = `${journalPath}.${process.pid}.${Date.now()}.tmp`;
  try {
    await writeFile(tempPath, `${JSON.stringify(journal, null, 2)}\n`, {
      encoding: 'utf8',
      mode: 0o600,
    });
    await rename(tempPath, journalPath);
  } catch (error) {
    await unlink(tempPath).catch(() => undefined);
    throw new FileSystemError(
      `Could not persist apply journal ${journalPath}: ${error instanceof Error ? error.message : String(error)}`,
      {
        cause: error,
        context: { path: journalPath, operation: 'writeJournal' },
      },
    );
  }
}

async function captureIfPresent(
  filePath: string,
): Promise<OrganizationPlanFingerprint | null> {
  try {
    await stat(filePath);
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      return null;
    }
    throw error;
  }
  return fingerprintFile(filePath);
}

function assertFingerprintMatches(
  action: OrganizationPlanAction,
  current: OrganizationPlanFingerprint,
  label: string,
): void {
  if (
    current.size !== action.fingerprint.size ||
    current.md5 !== action.fingerprint.md5
  ) {
    throw new ValidationError(
      `Organization plan action ${action.id} ${label} fingerprint differs from the reviewed plan`,
    );
  }
}

async function reconcileExistingTarget(
  plan: OrganizationPlan,
  action: OrganizationPlanAction,
): Promise<boolean> {
  const targetFingerprint = await captureIfPresent(action.target);
  if (!targetFingerprint) return false;
  assertFingerprintMatches(action, targetFingerprint, 'target');

  if (plan.operation === 'move') {
    const sourceFingerprint = await captureIfPresent(action.source);
    if (sourceFingerprint) {
      assertFingerprintMatches(action, sourceFingerprint, 'source');
      await unlink(action.source);
    }
  }
  return true;
}

export async function applyOrganizationPlan(
  planPath: string,
  journalPath = `${resolve(planPath)}.journal.json`,
): Promise<ApplyOrganizationPlanResult> {
  const plan = await readOrganizationPlan(planPath);
  if (!plan.review.approved) {
    throw new ValidationError(
      'Organization plan is not approved; inspect it and set review.approved to true before applying',
    );
  }
  assertActionRoots(plan);
  const resolvedJournalPath = resolve(journalPath);
  const { journal, existed } = await readJournal(resolvedJournalPath, plan);
  let completedActions = 0;

  for (const action of plan.actions) {
    const record =
      journal.actions[action.id] ??
      ({
        status: 'pending',
        attempts: 0,
      } satisfies OrganizationPlanJournalAction);
    if (record.status === 'completed') {
      const targetFingerprint = await captureIfPresent(action.target);
      if (!targetFingerprint) {
        throw new FileSystemError(
          `Apply journal marks ${action.id} complete but target is missing`,
          {
            context: { path: action.target, operation: 'resumeApply' },
          },
        );
      }
      assertFingerprintMatches(action, targetFingerprint, 'completed target');
      completedActions++;
      continue;
    }

    record.attempts += 1;
    try {
      const alreadyApplied = await reconcileExistingTarget(plan, action);
      if (!alreadyApplied) {
        const sourceFingerprint = await captureIfPresent(action.source);
        if (!sourceFingerprint) {
          throw new FileSystemError(
            `Source file is missing: ${action.source}`,
            {
              context: { path: action.source, operation: 'applyPlan' },
            },
          );
        }
        assertFingerprintMatches(action, sourceFingerprint, 'source');
        await transferOrCopyFile(
          action.source,
          action.target,
          plan.operation === 'copy',
        );
      }

      record.status = 'completed';
      record.completedAt = new Date().toISOString();
      delete record.lastError;
      completedActions++;
    } catch (error) {
      record.status = 'pending';
      record.lastError = error instanceof Error ? error.message : String(error);
      journal.actions[action.id] = record;
      journal.updatedAt = new Date().toISOString();
      await persistJournal(resolvedJournalPath, journal);
      throw error;
    }
    journal.actions[action.id] = record;
    journal.updatedAt = new Date().toISOString();
    await persistJournal(resolvedJournalPath, journal);
  }

  return {
    planId: plan.planId,
    journalPath: resolvedJournalPath,
    completedActions,
    totalActions: plan.actions.length,
    resumed: existed,
  };
}
