//! What the agent sees while a transaction is open, and what the host does not.

use super::support::{Fixture, stage_all};
use latch_transaction::{Change, State, TransactionError};

#[test]
fn staged_work_is_invisible_to_the_host() {
    let fixture = Fixture::new("invisible");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    stage_all(&mut transaction);

    assert_eq!(
        before.differences(&fixture.snapshot()),
        vec![],
        "the host must not change while a transaction is open"
    );
    // The agent sees its own work, not the host's state.
    assert_eq!(
        transaction.read("src/main.rs").unwrap(),
        b"fn main() { changed() }"
    );
    assert_eq!(transaction.read("docs/manual.md").unwrap(), b"# readme");
    assert!(matches!(
        transaction.read("keep.txt"),
        Err(TransactionError::NotFound(_))
    ));

    let changes = transaction.changes().unwrap();
    let described: Vec<String> = changes
        .changes()
        .iter()
        .map(|change| match change {
            Change::Created { path, .. } => format!("created {}", path.as_str()),
            Change::Modified { path, .. } => format!("modified {}", path.as_str()),
            Change::Deleted { path, .. } => format!("deleted {}", path.as_str()),
            Change::Renamed { from, to, .. } => {
                format!("renamed {} -> {}", from.as_str(), to.as_str())
            }
        })
        .collect();
    assert_eq!(
        described,
        [
            "renamed docs/readme.md -> docs/manual.md",
            "created generated/deep/file.txt",
            "deleted keep.txt",
            "modified src/main.rs",
        ],
        "src/lib.rs was rewritten with identical content and is not a change"
    );
    let summary = changes.summary();
    assert_eq!((summary.created, summary.modified), (1, 1));
    assert_eq!((summary.deleted, summary.renamed), (1, 1));
    assert_eq!(
        summary.to_string(),
        "1 modified file\n1 new file\n1 deleted file\n1 renamed file"
    );
}

#[test]
fn rollback_before_commit_leaves_no_trace() {
    let fixture = Fixture::new("discard");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    stage_all(&mut transaction);
    transaction.rollback().unwrap();

    assert_eq!(transaction.state(), State::RolledBack);
    assert_eq!(before.differences(&fixture.snapshot()), vec![]);
    assert!(transaction.changes().unwrap().is_empty());
    assert!(matches!(
        transaction.write("src/main.rs", b"late"),
        Err(TransactionError::NotOpen(State::RolledBack))
    ));
}
