use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("samples")
        .join(name)
}

#[test]
fn lint_ready_manifest_succeeds() {
    let output = Command::new(env!("CARGO_BIN_EXE_aeo-linter"))
        .arg("lint")
        .arg(fixture("manifest-ready.json"))
        .output()
        .expect("run aeo-linter");

    assert!(output.status.success(), "expected success");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("READY"), "stdout missing READY: {stdout}");
    assert!(stdout.contains("score"), "stdout missing score: {stdout}");
}

#[test]
fn lint_problem_manifest_returns_non_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_aeo-linter"))
        .arg("lint")
        .arg(fixture("manifest-problem.json"))
        .output()
        .expect("run aeo-linter");

    assert!(!output.status.success(), "expected non-zero exit");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("BLOCKED") || stdout.contains("WATCH"));
}

#[test]
fn lint_json_format_emits_machine_readable_payload() {
    let output = Command::new(env!("CARGO_BIN_EXE_aeo-linter"))
        .arg("lint")
        .arg(fixture("manifest-ready.json"))
        .arg("--format")
        .arg("json")
        .output()
        .expect("run aeo-linter json");

    assert!(output.status.success(), "expected success");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"summary\""),
        "stdout missing summary: {stdout}"
    );
    assert!(
        stdout.contains("\"findings\""),
        "stdout missing findings: {stdout}"
    );
}
