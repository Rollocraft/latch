//! Path escapes, symlinks, workspace ownership and the finished state.

use super::support::{Fixture, stage_all};
use latch_transaction::{Selection, State, Transaction, TransactionError};

#[test]
fn paths_that_could_escape_the_root_are_refused() {
    let fixture = Fixture::new("escape");
    let mut transaction = fixture.begin();
    for path in [
        "../escape.txt",
        "src/../../escape.txt",
        "/etc/passwd",
        "C:/windows/system32/drivers/etc/hosts",
        "src\\main.rs",
        "",
        "src/../src/main.rs",
    ] {
        assert!(
            matches!(
                transaction.write(path, b"x"),
                Err(TransactionError::Path(_))
            ),
            "{path:?} was accepted"
        );
        assert!(matches!(
            transaction.read(path),
            Err(TransactionError::Path(_))
        ));
    }
    assert!(!fixture.base.join("escape.txt").exists());
    assert!(transaction.changes().unwrap().is_empty());
}

#[cfg(unix)]
#[test]
fn symbolic_links_are_never_followed_out_of_the_root() {
    let fixture = Fixture::new("symlink");
    let outside = fixture.base.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("secret.txt"), "secret").unwrap();
    std::os::unix::fs::symlink(&outside, fixture.host("link")).unwrap();
    std::os::unix::fs::symlink(outside.join("secret.txt"), fixture.host("secret.txt")).unwrap();

    let mut transaction = fixture.begin();
    for path in ["link/secret.txt", "secret.txt"] {
        assert!(
            matches!(
                transaction.write(path, b"overwritten"),
                Err(TransactionError::Path(
                    latch_transaction::PathError::Symlink
                ))
            ),
            "{path} was followed"
        );
        assert!(matches!(
            transaction.read(path),
            Err(TransactionError::Path(
                latch_transaction::PathError::Symlink
            ))
        ));
        assert!(matches!(
            transaction.remove(path),
            Err(TransactionError::Path(
                latch_transaction::PathError::Symlink
            ))
        ));
    }
    assert_eq!(
        std::fs::read_to_string(outside.join("secret.txt")).unwrap(),
        "secret"
    );
}

#[test]
fn a_workspace_is_private_and_reopenable() {
    let fixture = Fixture::new("workspace");
    let before = fixture.snapshot();
    let mut transaction = fixture.begin();
    stage_all(&mut transaction);
    drop(transaction);

    assert!(matches!(
        Transaction::begin(fixture.root(), fixture.workspace()),
        Err(TransactionError::WorkspaceExists)
    ));
    assert!(matches!(
        Transaction::begin(fixture.root(), fixture.host("inside")),
        Err(TransactionError::WorkspaceInsideRoot)
    ));

    // Staging and committing may happen in different processes.
    let mut reopened = Transaction::open(fixture.workspace()).unwrap();
    assert_eq!(reopened.state(), State::Open);
    assert_eq!(reopened.changes().unwrap().summary().created, 1);
    reopened.commit(&Selection::All).unwrap();
    drop(reopened);

    let mut reopened = Transaction::open(fixture.workspace()).unwrap();
    assert_eq!(reopened.state(), State::Committed);
    assert_eq!(fixture.read("generated/deep/file.txt"), "new file");
    reopened.rollback().unwrap();
    assert_eq!(
        before.differences(&fixture.snapshot()),
        vec![],
        "a journal written by one process must be enough to undo the commit"
    );
}

#[test]
fn a_finished_transaction_refuses_further_work() {
    let fixture = Fixture::new("finished");
    let mut transaction = fixture.begin();
    transaction.write("src/main.rs", b"changed").unwrap();
    transaction.commit(&Selection::All).unwrap();

    for result in [
        transaction.write("src/lib.rs", b"late"),
        transaction.remove("keep.txt"),
        transaction.rename("keep.txt", "moved.txt"),
        transaction.set_mode("keep.txt", 0o400),
    ] {
        assert!(matches!(
            result,
            Err(TransactionError::NotOpen(State::Committed))
        ));
    }
    assert!(matches!(
        transaction.commit(&Selection::All),
        Err(TransactionError::NotOpen(State::Committed))
    ));
    assert_eq!(fixture.read("keep.txt"), "keep");
}
