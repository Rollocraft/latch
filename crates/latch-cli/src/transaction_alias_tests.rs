use crate::workflow_test_fixtures::*;
use latch_transaction::Transaction;

#[test]
fn top_level_transaction_aliases_preserve_explicit_workspace_and_selection() {
    let files = Files::new();
    let root = files.0.join("root");
    let workspace = files.0.join("workspace").into_os_string();
    std::fs::create_dir(&root).unwrap();
    std::fs::write(root.join("a"), b"before").unwrap();
    assert!(
        call(&[
            "tx".as_ref(),
            "begin".as_ref(),
            root.as_os_str(),
            &workspace
        ])
        .0
        .is_ok()
    );
    let mut tx = Transaction::open(&workspace).unwrap();
    tx.write("a", b"after").unwrap();
    tx.write("b", b"new").unwrap();
    drop(tx);
    assert_eq!(
        call(&["diff".as_ref(), &workspace]),
        call(&["tx".as_ref(), "changes".as_ref(), &workspace])
    );
    assert!(
        call(&["commit".as_ref(), &workspace, "a".as_ref()])
            .0
            .is_ok()
    );
    assert_eq!(std::fs::read(root.join("a")).unwrap(), b"after");
    assert!(!root.join("b").exists());
    assert!(call(&["commit".as_ref(), &workspace]).0.is_err());
    assert!(call(&["rollback".as_ref(), &workspace]).0.is_ok());
    assert_eq!(std::fs::read(root.join("a")).unwrap(), b"before");
    assert!(!root.join("b").exists());
    let workspace = files.0.join("workspace-all").into_os_string();
    assert!(
        call(&[
            "tx".as_ref(),
            "begin".as_ref(),
            root.as_os_str(),
            &workspace
        ])
        .0
        .is_ok()
    );
    let mut tx = Transaction::open(&workspace).unwrap();
    tx.write("a", b"after").unwrap();
    tx.write("b", b"new").unwrap();
    drop(tx);
    assert!(call(&["commit".as_ref(), &workspace]).0.is_ok());
    assert_eq!(std::fs::read(root.join("a")).unwrap(), b"after");
    assert_eq!(std::fs::read(root.join("b")).unwrap(), b"new");
    assert!(call(&["rollback".as_ref(), &workspace]).0.is_ok());
    assert_eq!(std::fs::read(root.join("a")).unwrap(), b"before");
    assert!(!root.join("b").exists());
}
