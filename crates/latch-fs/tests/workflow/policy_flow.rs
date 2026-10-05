//! The whole path a file action takes: identity, policy, budget, staged work,
//! audit, and a human deciding what to keep. Each layer is tested on its own
//! elsewhere; what matters here is that together they hold.

use super::support::*;
use latch_audit::EventResult;
use latch_fs::*;
use latch_policy::*;
use latch_runtime::gate::*;
use latch_transaction::Selection;

#[test]
fn an_allowed_write_is_staged_audited_and_left_for_review() {
    let fixture = Fixture::new("allowed");
    let before = fixture.snapshot();
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    gate.execute(
        &action("a1", WRITE, "src/main.rs", &["fn main() { fixed() }"]),
        20,
        &mut adapter,
    )
    .unwrap();

    assert_eq!(
        before.differences(&fixture.snapshot()),
        vec![],
        "an allowed action still must not touch the host before a commit"
    );
    let results: Vec<EventResult> = gate.audit().events().iter().map(|e| e.result).collect();
    assert_eq!(results, [EventResult::Started, EventResult::Succeeded]);

    let mut transaction = adapter.into_transaction();
    assert_eq!(transaction.changes().unwrap().summary().modified, 1);
    transaction.commit(&Selection::All).unwrap();
    assert_eq!(fixture.read("src/main.rs"), "fn main() { fixed() }");
    transaction.rollback().unwrap();
    assert_eq!(before.differences(&fixture.snapshot()), vec![]);
}

#[test]
fn a_denied_action_stages_nothing() {
    let fixture = Fixture::new("denied");
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    let error = gate
        .execute(
            &action("a1", WRITE, "secret.env", &["TOKEN=stolen"]),
            20,
            &mut adapter,
        )
        .unwrap_err();
    assert!(matches!(&error, ExecutionError::Policy(decision)
        if decision.outcome == Outcome::Deny && decision.reason == Reason::ExplicitDeny));

    // Not merely unwritten to the host: never staged, so a later commit of
    // unrelated work cannot carry it along.
    assert!(adapter.transaction().changes().unwrap().is_empty());
    assert_eq!(fixture.read("secret.env"), "TOKEN=live");
    let results: Vec<EventResult> = gate.audit().events().iter().map(|e| e.result).collect();
    assert_eq!(results, [EventResult::Denied]);
}

#[test]
fn an_unmatched_resource_falls_through_to_default_deny() {
    let fixture = Fixture::new("default");
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    let error = gate
        .execute(
            &action("a1", WRITE, "docs/notes.md", &["notes"]),
            20,
            &mut adapter,
        )
        .unwrap_err();
    assert!(matches!(&error, ExecutionError::Policy(decision)
        if decision.reason == Reason::DefaultDeny));
    assert!(adapter.transaction().changes().unwrap().is_empty());
}

#[test]
fn the_budget_bounds_what_an_allowed_agent_can_write() {
    let fixture = Fixture::new("budget");
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    gate.execute(
        &action("a1", WRITE, "src/first.rs", &["x".repeat(40).as_str()]),
        20,
        &mut adapter,
    )
    .unwrap();
    let error = gate
        .execute(
            &action("a2", WRITE, "src/second.rs", &["y".repeat(40).as_str()]),
            21,
            &mut adapter,
        )
        .unwrap_err();
    assert!(
        matches!(&error, ExecutionError::Budget(_)),
        "expected budget refusal, got {error:?}"
    );

    let changes = adapter.transaction().changes().unwrap();
    assert_eq!(
        changes.summary().created,
        1,
        "the refused write left nothing"
    );
    let results: Vec<EventResult> = gate.audit().events().iter().map(|e| e.result).collect();
    assert_eq!(
        results,
        [
            EventResult::Started,
            EventResult::Succeeded,
            EventResult::BudgetRejected
        ]
    );
}

#[test]
fn a_deletion_waits_for_a_human_and_is_reviewable_afterwards() {
    let fixture = Fixture::new("approval");
    let before = fixture.snapshot();
    let mut gate = gate();
    let mut adapter = fixture.adapter();
    let deletion = action("a1", DELETE, "src/main.rs", &[]);

    let error = gate.execute(&deletion, 20, &mut adapter).unwrap_err();
    assert!(matches!(&error, ExecutionError::Policy(decision)
        if decision.outcome == Outcome::ApprovalRequired));
    assert!(adapter.transaction().changes().unwrap().is_empty());

    gate.approve_once(&deletion, "david".into(), 21, 60)
        .unwrap();
    gate.execute(&deletion, 22, &mut adapter).unwrap();

    let mut transaction = adapter.into_transaction();
    assert_eq!(transaction.changes().unwrap().summary().deleted, 1);
    transaction.commit(&Selection::All).unwrap();
    assert!(!fixture.root().join("src/main.rs").exists());

    // The reviewer changed their mind: the deletion is undone exactly.
    transaction.rollback().unwrap();
    assert_eq!(before.differences(&fixture.snapshot()), vec![]);
    let results: Vec<EventResult> = gate.audit().events().iter().map(|e| e.result).collect();
    assert_eq!(
        results,
        [
            EventResult::ApprovalRequired,
            EventResult::Authorized, // runtime.approval.grant for david
            EventResult::Started,
            EventResult::Succeeded
        ]
    );
}

#[test]
fn a_move_is_charged_for_the_content_it_restages() {
    let fixture = Fixture::new("rename");
    let mut gate = gate();
    let mut adapter = fixture.adapter();

    gate.execute(
        &action("a1", RENAME, "src/main.rs", &["src/app.rs"]),
        20,
        &mut adapter,
    )
    .unwrap();

    let mut transaction = adapter.into_transaction();
    let changes = transaction.changes().unwrap();
    assert_eq!(changes.summary().renamed, 1, "{:?}", changes.changes());
    transaction.commit(&Selection::All).unwrap();
    assert_eq!(fixture.read("src/app.rs"), "fn main() {}");
    assert!(!fixture.root().join("src/main.rs").exists());
}
