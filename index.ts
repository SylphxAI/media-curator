#!/usr/bin/env node

// ADR-168 S0: health fast-path before loading pipeline dependencies.
if (process.argv.includes('--health')) {
  const { runHealthProbe } = await import('./src/cli/healthProbe.js');
  const code = await runHealthProbe();
  process.exit(code);
}

function optionValue(name: string): string | undefined {
  const exactIndex = process.argv.indexOf(name);
  if (exactIndex >= 0) return process.argv[exactIndex + 1];
  const prefix = `${name}=`;
  const inline = process.argv.find((argument) => argument.startsWith(prefix));
  return inline?.slice(prefix.length);
}

const applyPlanPath = optionValue('--apply-plan');
if (applyPlanPath !== undefined) {
  const { applyOrganizationPlan } = await import('./src/organizationPlan.js');
  try {
    const result = await applyOrganizationPlan(applyPlanPath);
    console.log(
      `Applied organization plan ${result.planId}: ${result.completedActions}/${result.totalActions} actions complete. Journal: ${result.journalPath}${result.resumed ? ' (resumed)' : ''}`,
    );
  } catch (error) {
    console.error(
      `Organization plan apply failed: ${error instanceof Error ? error.message : String(error)}`,
    );
    process.exitCode = 1;
  }
} else {
  await import('./src/pipelineMain.js');
}
