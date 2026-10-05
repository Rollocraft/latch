use super::*;
use crate::inspection_test_support::{Fixture, invoke};
use latch_transaction::{Selection, Snapshot};

#[test]
fn status_aliases_report_canonical_paths_without_mutation() {
    let fixture = Fixture::new("status");
    let root = fixture.0.join("root");
    let workspace = fixture.0.join("workspace");
    std::fs::create_dir(&root).unwrap();
    std::fs::write(root.join("existing"), b"before").unwrap();
    let mut tx = Transaction::begin(&root, &workspace).unwrap();
    tx.write("existing", b"after").unwrap();
    tx.write("new", b"new").unwrap();
    let before_root = Snapshot::of(&root, 100).unwrap();
    let before_workspace = Snapshot::of(&workspace, 100).unwrap();
    let alias_path = workspace.join(".").into_os_string();
    let output = invoke(vec!["status".into(), alias_path.clone()]).unwrap();
    assert_eq!(
        output,
        invoke(vec!["tx".into(), "status".into(), alias_path]).unwrap()
    );
    assert!(output.contains(&format!(
        "workspace: {:?}",
        workspace.canonicalize().unwrap()
    )));
    assert!(output.contains(&format!("root: {:?}", root.canonicalize().unwrap())));
    for expected in [
        "state: OPEN",
        "1 modified file",
        "1 new file",
        "not commit history",
    ] {
        assert!(output.contains(expected), "{output}");
    }
    assert_eq!(Snapshot::of(&root, 100).unwrap(), before_root);
    assert_eq!(Snapshot::of(&workspace, 100).unwrap(), before_workspace);
}

#[test]
fn closed_status_is_not_a_commit_history() {
    let fixture = Fixture::new("closed");
    let root = fixture.0.join("root");
    let workspace = fixture.0.join("workspace");
    std::fs::create_dir(&root).unwrap();
    let mut tx = Transaction::begin(&root, &workspace).unwrap();
    tx.write("selected", b"one").unwrap();
    tx.write("retained", b"two").unwrap();
    tx.commit(&Selection::paths(["selected"]).unwrap()).unwrap();
    let args = vec!["status".into(), workspace.clone().into_os_string()];
    let output = invoke(args.clone()).unwrap();
    assert!(output.contains("COMMITTED (closed"));
    assert!(output.contains("1 new file"));
    std::fs::write(root.join("retained"), b"two").unwrap();
    assert!(invoke(args.clone()).unwrap().contains("no changes"));
    tx.rollback().unwrap();
    assert!(invoke(args).unwrap().contains("ROLLED-BACK (closed)"));
}

#[test]
fn missing_or_invalid_workspace_is_an_error() {
    let fixture = Fixture::new("invalid");
    for path in [fixture.0.clone(), fixture.0.join("absent")] {
        let error = invoke(vec!["status".into(), path.into_os_string()]).unwrap_err();
        assert!(error.contains("cannot inspect workspace"));
    }
    for args in [
        vec!["status".into()],
        vec!["status".into(), "a".into(), "b".into()],
    ] {
        assert!(invoke(args).is_err());
    }
    assert!(state_label(State::Inconsistent).contains("effects may be partially applied"));
}
