use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tartarust-cli");
const PEPPER: &str = "integration_test_pepper";

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tartarus_cli_test_{}_{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn run_in(dir: &PathBuf, args: &[&str], pepper: bool) -> Output {
    let mut cmd = Command::new(BIN);
    cmd.args(args).current_dir(dir).env("NO_COLOR", "1");
    if pepper {
        cmd.env("TARTARUS_PEPPER", PEPPER);
    }
    cmd.output().unwrap()
}

fn stdout_str(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn extract_label(text: &str, label: &str) -> String {
    text.lines()
        .find_map(|line| line.trim().strip_prefix(label).map(|value| value.trim().to_string()))
        .expect("label present in output")
}

#[test]
fn params_shows_defaults_without_config() {
    let dir = temp_dir("no_config");
    let text = stdout_str(&run_in(&dir, &["params"], false));

    assert!(text.contains("Current parameters:"));
    assert!(text.contains("Memory: 128 MB"));
    assert!(text.contains("Iterations: 3"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn params_saves_and_persists() {
    let dir = temp_dir("persist");

    let changed = stdout_str(&run_in(&dir, &["params", "-m", "64"], false));
    assert!(changed.contains("Parameters changed:"));
    assert!(changed.contains("Memory: 128 MB -> 64 MB"));
    assert!(changed.contains("Iterations: 3 -> 3"));

    let config = dir.join("tartarus_params.toml");
    assert!(config.exists());
    assert_eq!(std::fs::read_to_string(&config).unwrap(), "memory = 64\niterations = 3\n");

    let shown = stdout_str(&run_in(&dir, &["params"], false));
    assert!(shown.contains("Memory: 64 MB"));
    assert!(shown.contains("Iterations: 3"));

    let iter_only = stdout_str(&run_in(&dir, &["params", "--iterations", "2"], false));
    assert!(iter_only.contains("Memory: 64 MB -> 64 MB"));
    assert!(iter_only.contains("Iterations: 3 -> 2"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn hash_then_verify_round_trip() {
    let dir = temp_dir("roundtrip");

    let hash_out = stdout_str(&run_in(&dir, &["hash", "-p", "password123"], true));
    let salt = extract_label(&hash_out, "Salt:");
    let hash = extract_label(&hash_out, "Hash:");

    let ok = stdout_str(&run_in(&dir, &["verify", "-p", "password123", "-s", &salt, "-H", &hash], true));
    assert!(ok.contains("Success: the password is correct."));

    let bad = stdout_str(&run_in(&dir, &["verify", "-p", "wrong-password", "-s", &salt, "-H", &hash], true));
    assert!(bad.contains("Failure: the password is incorrect."));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn hash_requires_pepper() {
    let dir = temp_dir("no_pepper");

    let text = stdout_str(&run_in(&dir, &["hash", "-p", "password123"], false));
    assert!(text.contains("[-]"));
    assert!(text.contains("TARTARUS_PEPPER"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn verify_rejects_malformed_inputs() {
    let dir = temp_dir("bad_inputs");

    let bad_salt = stdout_str(&run_in(&dir, &["verify", "-p", "password123", "-s", "zzz", "-H", "ab"], true));
    assert!(bad_salt.contains("valid hex salt"));

    let valid_salt = "00".repeat(16);
    let bad_hash = stdout_str(&run_in(&dir, &["verify", "-p", "password123", "-s", &valid_salt, "-H", "zzz"], true));
    assert!(bad_hash.contains("valid hex hash"));

    std::fs::remove_dir_all(&dir).ok();
}
