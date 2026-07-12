use std::process::Command;

#[test]
fn version_flag_prints_workspace_semver() {
    let binary = env!("CARGO_BIN_EXE_media-curator-cli");
    let output = Command::new(binary)
        .arg("--version")
        .output()
        .expect("binary must execute");

    assert!(
        output.status.success(),
        "--version failed: stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "expected version {} in stdout: {stdout}",
        env!("CARGO_PKG_VERSION")
    );
}
