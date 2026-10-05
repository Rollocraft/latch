use std::process::Command;

#[test]
fn binary_reports_real_exit_statuses() {
    let directory = std::env::temp_dir().join(format!(
        "latch-cli-exit-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&directory).unwrap();
    let policy = directory.join("policy.json");
    let scenario = directory.join("scenario.json");
    std::fs::write(&policy, br#"{"version":1,"policies":[]}"#).unwrap();
    let binary = env!("CARGO_BIN_EXE_latch");
    let help = Command::new(binary).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(
        String::from_utf8(help.stdout)
            .unwrap()
            .contains("latch policy test")
    );
    let help = Command::new(binary)
        .args(["policy", "test", "--help"])
        .output()
        .unwrap();
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    let start = help.find("{\"version\"").unwrap();
    let end = help.find("\nExpected outcomes:").unwrap();
    std::fs::write(&scenario, &help[start..end]).unwrap();
    let failed = Command::new(binary)
        .args(["policy", "test"])
        .arg(&policy)
        .arg(&scenario)
        .output()
        .unwrap();
    assert_eq!(failed.status.code(), Some(1));
    assert!(
        String::from_utf8(failed.stdout)
            .unwrap()
            .contains("0 passed; 1 failed")
    );
    let mut value: serde_json::Value = serde_json::from_str(&help[start..end]).unwrap();
    value["scenarios"][0]["expected"] = serde_json::json!("deny");
    std::fs::write(&scenario, value.to_string()).unwrap();
    let passed = Command::new(binary)
        .args(["policy", "test"])
        .arg(&policy)
        .arg(&scenario)
        .output()
        .unwrap();
    assert!(passed.status.success());
    assert!(
        String::from_utf8(passed.stdout)
            .unwrap()
            .contains("1 passed; 0 failed")
    );
    let unsupported = Command::new(binary)
        .args(["run", "--", "does-not-exist"])
        .output()
        .unwrap();
    assert_eq!(unsupported.status.code(), Some(1));
    assert!(unsupported.stdout.is_empty());
    assert!(
        String::from_utf8(unsupported.stderr)
            .unwrap()
            .contains("no process was started")
    );
    let invalid = Command::new(binary)
        .args(["manifest", "check"])
        .arg(&policy)
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(1));
    assert!(invalid.stdout.is_empty());
    std::fs::remove_dir_all(directory).unwrap();
}
