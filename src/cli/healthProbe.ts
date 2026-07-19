import { healthViaRust } from '../external/rustCli';

/** ADR-168 health fast-path — Rust authority by default (no pipeline load). */
export async function runHealthProbe(): Promise<number> {
  const body = healthViaRust();
  console.log(JSON.stringify(body));
  return 0;
}
