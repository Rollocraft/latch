//! The files under `examples/` are documentation; these tests keep them true.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
}

fn latch(args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_latch"))
        .args(args)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

#[test]
fn example_policy_is_valid() {
    let output = latch(&[
        "policy".as_ref(),
        "validate".as_ref(),
        &example("policy.yaml"),
    ]);
    assert!(output.status.success(), "{output:?}");
}

#[test]
fn example_scenarios_pass_against_example_policy() {
    let output = latch(&[
        "policy".as_ref(),
        "test".as_ref(),
        &example("policy.yaml"),
        &example("scenarios.json"),
    ]);
    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("7 passed; 0 failed"));
}

#[test]
fn example_manifest_diff_requires_review() {
    for name in ["manifests/coder-v1.json", "manifests/coder-v2.json"] {
        let output = latch(&["manifest".as_ref(), "check".as_ref(), &example(name)]);
        assert!(output.status.success(), "{name}: {output:?}");
    }
    let output = latch(&[
        "manifest".as_ref(),
        "diff".as_ref(),
        &example("manifests/coder-v1.json"),
        &example("manifests/coder-v2.json"),
    ]);
    assert!(output.status.success(), "{output:?}");
    assert!(stdout(&output).contains("requires_review: true"));
}
