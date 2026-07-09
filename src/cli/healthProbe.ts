import { healthViaRust, rustCliDelegationEnabled } from '../external/rustCli';

/** ADR-168 S0 health fast-path — no pipeline dependency load. */
export async function runHealthProbe(): Promise<number> {
  if (!rustCliDelegationEnabled()) {
    console.error(
      'Health probe requires MEDIA_CURATOR_RUST_CLI=1 (ADR-168 S0 Rust delegation).',
    );
    return 2;
  }

  const body = healthViaRust();
  console.log(JSON.stringify(body));
  return 0;
}
