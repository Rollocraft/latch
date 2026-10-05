//! What the adapter itself refuses: escapes, malformed actions and the exact
//! audit trail each outcome leaves behind.

use super::support::*;
use latch_audit::EventResult;
use latch_fs::*;
use latch_runtime::gate::*;

#[test]
fn a_path_that_escapes_the_root_fails_in_the_adapter() {
    let fixture = Fixture::new("escape");
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    // The policy is written for src/, and the traversal keeps that prefix:
    // string-level policy matching is not what stops this, the adapter is.
    let error = gate
        .execute(
            &action("a1", WRITE, "src/../secret.env", &["TOKEN=stolen"]),
            20,
            &mut adapter,
        )
        .unwrap_err();
    assert!(
        matches!(&error, ExecutionError::Adapter(FileError::Transaction(_))),
        "expected the adapter to refuse the path, got {error:?}"
    );
    assert_eq!(fixture.read("secret.env"), "TOKEN=live");
    assert!(adapter.transaction().changes().unwrap().is_empty());
}

#[test]
fn an_action_the_adapter_cannot_represent_is_refused_before_it_runs() {
    let fixture = Fixture::new("malformed");
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    // Each of these is allowed by policy and only then found to be unusable:
    // a decision the adapter cannot carry out is refused, never approximated.
    for (id, name, arguments) in [
        ("a1", WRITE, vec![]),
        ("a2", WRITE, vec!["one", "two"]),
        ("a3", RENAME, vec![]),
    ] {
        let error = gate
            .execute(
                &action(id, name, "src/main.rs", &arguments),
                20,
                &mut adapter,
            )
            .unwrap_err();
        assert!(
            matches!(&error, ExecutionError::Adapter(FileError::MalformedAction)),
            "{id} produced {error:?}"
        );
    }
    assert!(adapter.transaction().changes().unwrap().is_empty());
    assert!(
        gate.audit()
            .events()
            .iter()
            .all(|event| event.result == EventResult::PreparationFailed)
    );
}
