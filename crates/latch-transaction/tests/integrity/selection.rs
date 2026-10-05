//! Partial commits: what was selected is applied, nothing more.

use super::support::{Fixture, stage_all};
use latch_transaction::{Selection, State, TransactionError};

#[test]
fn a_partial_commit_applies_only_what_was_selected() {
    let fixture = Fixture::new("partial");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    stage_all(&mut transaction);

    // Naming the source of a move selects the whole move.
    let selection = Selection::paths(["src/main.rs", "docs/readme.md"]).unwrap();
    let summary = transaction.commit(&selection).unwrap();
    assert_eq!(summary.modified, 1);
    assert_eq!(summary.renamed, 1);
    assert_eq!((summary.created, summary.deleted), (0, 0));

    assert_eq!(fixture.read("src/main.rs"), "fn main() { changed() }");
    assert_eq!(fixture.read("docs/manual.md"), "# readme");
    assert!(!fixture.host("docs/readme.md").exists());
    assert_eq!(
        fixture.read("keep.txt"),
        "keep",
        "an unselected deletion must not be applied"
    );
    assert!(!fixture.host("generated").exists());

    transaction.rollback().unwrap();
    assert_eq!(before.differences(&fixture.snapshot()), vec![]);
}

#[test]
fn a_selection_that_names_nothing_commits_nothing() {
    let fixture = Fixture::new("selection");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    stage_all(&mut transaction);

    let selection = Selection::paths(["src/main.rs", "src/absent.rs"]).unwrap();
    assert!(matches!(
        transaction.commit(&selection),
        Err(TransactionError::UnknownSelection(path)) if path == "src/absent.rs"
    ));
    assert_eq!(
        before.differences(&fixture.snapshot()),
        vec![],
        "a rejected commit must not apply the part it understood"
    );
    assert_eq!(transaction.state(), State::Open);
    transaction.commit(&Selection::All).unwrap();
    assert_eq!(fixture.read("src/main.rs"), "fn main() { changed() }");
}

#[test]
fn an_ambiguous_move_is_reported_as_what_is_certain() {
    let fixture = Fixture::new("ambiguous");
    fixture.write("a.txt", "same");
    fixture.write("b.txt", "same");
    let mut transaction = fixture.begin();
    transaction.remove("a.txt").unwrap();
    transaction.remove("b.txt").unwrap();
    transaction.write("c.txt", b"same").unwrap();

    let changes = transaction.changes().unwrap();
    assert_eq!(changes.summary().renamed, 0, "two sources, one destination");
    assert_eq!(changes.summary().created, 1);
    assert_eq!(changes.summary().deleted, 2);
}
