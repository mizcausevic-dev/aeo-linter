use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("samples")
        .join(name)
}

fn fresh_ready_fixture(name: &str) -> PathBuf {
    let raw = std::fs::read_to_string(fixture("manifest-ready.json")).expect("read sample");
    let mut manifest: serde_json::Value = serde_json::from_str(&raw).expect("parse sample");
    let today = chrono::Utc::now().date_naive().to_string();
    manifest["llm_txt"]["last_updated"] = today.clone().into();
    for source in manifest["sources"].as_array_mut().expect("sources array") {
        source["last_updated"] = today.clone().into();
    }
    let path = std::env::temp_dir().join(format!("aeo-linter-{}-{name}.json", std::process::id()));
    std::fs::write(&path, serde_json::to_vec(&manifest).expect("serialize sample"))
        .expect("write temporary sample");
    path
}

#[test]
fn lint_ready_manifest_succeeds() {
    let path = fresh_ready_fixture("text");
    let output = Command::new(env!("CARGO_BIN_EXE_aeo-linter"))
        .arg("lint")
        .arg(&path)
        .output()
        .expect("run aeo-linter");
    std::fs::remove_file(path).expect("remove temporary sample");

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
    let path = fresh_ready_fixture("json");
    let output = Command::new(env!("CARGO_BIN_EXE_aeo-linter"))
        .arg("lint")
        .arg(&path)
        .arg("--format")
        .arg("json")
        .output()
        .expect("run aeo-linter json");
    std::fs::remove_file(path).expect("remove temporary sample");

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
