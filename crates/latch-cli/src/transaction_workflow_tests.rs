use crate::command_test_support::{invoke, temporary};
use latch_transaction::Transaction;
use std::ffi::OsString;

#[test]
fn transactions_are_driven_end_to_end() {
    let base = temporary("tx");
    let root = base.join("root");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("keep.txt"), "keep").unwrap();
    std::fs::write(root.join("edit.txt"), "before").unwrap();
    let workspace: OsString = base.join("workspace").into();
    let (tx, changes) = (OsString::from("tx"), OsString::from("changes"));

    let output = invoke(&[&tx, &"begin".into(), &root.as_os_str().into(), &workspace]).unwrap();
    assert_eq!(
        Transaction::open(&workspace).unwrap().root(),
        root.canonicalize().unwrap()
    );
    assert_eq!(std::fs::read(root.join("edit.txt")).unwrap(), b"before");
    assert!(output.contains("transaction open"));

    let mut transaction = Transaction::open(&workspace).unwrap();
    transaction.write("edit.txt", b"after").unwrap();
    transaction.write("added.txt", b"added").unwrap();
    transaction.remove("keep.txt").unwrap();
    drop(transaction);

    let output = invoke(&[&tx, &changes, &workspace]).unwrap();
    assert!(output.contains("new      \"added.txt\""), "{output}");
    assert!(output.contains("modified  \"edit.txt\""), "{output}");
    assert!(output.contains("deleted  \"keep.txt\""), "{output}");
    assert!(output.contains("1 modified file"), "{output}");

    // A partial commit applies only the named path.
    let output = invoke(&[&tx, &"commit".into(), &workspace, &"edit.txt".into()]).unwrap();
    assert!(output.contains("committed"), "{output}");
    assert_eq!(
        std::fs::read_to_string(root.join("edit.txt")).unwrap(),
        "after"
    );
    assert!(root.join("keep.txt").exists());
    assert!(!root.join("added.txt").exists());

    let output = invoke(&[&tx, &"rollback".into(), &workspace]).unwrap();
    assert!(output.contains("rolled back"));
    assert_eq!(
        std::fs::read_to_string(root.join("edit.txt")).unwrap(),
        "before"
    );
    assert!(invoke(&[&tx, &"commit".into(), &workspace]).is_err());
    let _ = std::fs::remove_dir_all(base);
}
