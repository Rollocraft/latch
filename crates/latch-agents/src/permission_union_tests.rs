use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn redundant_addition_is_not_expansion_but_union_of_exacts_is_not_a_subtree() {
    let root = capability("file.read", "file:///project", true);
    let child = capability("file.read", "file:///project/src", false);
    let old = manifest(vec![root.clone()]);
    let next = manifest(vec![root.clone(), child.clone()]);
    let diff = PermissionDiff::between(&old, &next).unwrap();
    assert_eq!(diff.added(), std::slice::from_ref(&child));
    assert!(!diff.requires_review());
    let exacts = manifest(vec![
        child,
        capability("file.read", "file:///project", false),
    ]);
    assert!(
        PermissionDiff::between(&exacts, &manifest(vec![root]))
            .unwrap()
            .requires_review()
    );
}
