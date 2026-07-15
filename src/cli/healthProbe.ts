import { healthViaRust, rustCliDelegationEnabled } from '../external/rustCli';

/** ADR-168 health fast-path — Rust authority by default (no pipeline load). */
export async function runHealthProbe(): Promise<number> {
  if (!rustCliDelegationEnabled()) {
    console.error(
      'Health probe requires Rust authority (default). Unset MEDIA_CURATOR_RUST or set MEDIA_CURATOR_RUST=1.',
    );
    return 2;
  }

  const body = healthViaRust();
  console.log(JSON.stringify(body));
  return 0;
}
