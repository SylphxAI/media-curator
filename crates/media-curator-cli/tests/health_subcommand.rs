use std::process::Command;

#[test]
fn health_subcommand_emits_ok_rust_authority_json() {
    let binary = env!("CARGO_BIN_EXE_media-curator-cli");
    let output = Command::new(binary)
        .arg("health")
        .output()
        .expect("binary must execute");

    assert!(
        output.status.success(),
        "health failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""status":"ok""#));
    assert!(stdout.contains(r#""authority":"rust""#));
    assert!(stdout.contains(r#""stub":false"#));
}
