//! Commits apply exactly what was asked and rollbacks undo them exactly.

use super::support::{Fixture, stage_all};
use latch_transaction::{Change, Selection, State};

#[test]
fn commit_applies_every_change_and_rollback_undoes_them_exactly() {
    let fixture = Fixture::new("roundtrip");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    stage_all(&mut transaction);

    let summary = transaction.commit(&Selection::All).unwrap();
    assert_eq!(transaction.state(), State::Committed);
    assert_eq!((summary.created, summary.renamed), (1, 1));
    assert_eq!(fixture.read("src/main.rs"), "fn main() { changed() }");
    assert_eq!(fixture.read("generated/deep/file.txt"), "new file");
    assert_eq!(fixture.read("docs/manual.md"), "# readme");
    assert!(!fixture.host("keep.txt").exists());
    assert!(!fixture.host("docs/readme.md").exists());

    transaction.rollback().unwrap();
    assert_eq!(transaction.state(), State::RolledBack);
    assert_eq!(
        before.differences(&fixture.snapshot()),
        vec![],
        "rollback must restore contents, permissions and directory structure"
    );
    assert!(
        !fixture.host("generated").exists(),
        "directories created by the commit must not survive the rollback"
    );
    // Rolling back twice is how a retry after a partial failure works.
    transaction.rollback().unwrap();
    assert_eq!(before.differences(&fixture.snapshot()), vec![]);
}

#[test]
fn permission_changes_are_staged_committed_and_restored() {
    let fixture = Fixture::new("permissions");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    transaction.set_mode("keep.txt", 0o444).unwrap();

    let changes = transaction.changes().unwrap();
    assert!(
        matches!(
            changes.changes(),
            [Change::Modified { path, before, after, permissions: true }]
                if path.as_str() == "keep.txt" && before == after
        ),
        "a permission-only change keeps the content digest: {:?}",
        changes.changes()
    );

    transaction.commit(&Selection::All).unwrap();
    let committed = fixture.snapshot();
    assert_ne!(
        before.differences(&committed),
        vec![],
        "the committed permission change must be visible"
    );
    assert_eq!(fixture.read("keep.txt"), "keep");

    transaction.rollback().unwrap();
    assert_eq!(before.differences(&fixture.snapshot()), vec![]);
}
