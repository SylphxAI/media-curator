#!/usr/bin/env node

// ADR-168 S0: health fast-path before loading pipeline dependencies.
if (process.argv.includes('--health')) {
  const { runHealthProbe } = await import('./src/cli/healthProbe.js');
  const code = await runHealthProbe();
  process.exit(code);
}

await import('./src/pipelineMain.js');
