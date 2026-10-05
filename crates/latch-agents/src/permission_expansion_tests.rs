use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn expansion_requires_review_including_scope_changes_and_replacements() {
    let exact = manifest(vec![capability("file.read", "file:///project/src", false)]);
    let subtree = sample();
    let wider = manifest(vec![capability("file.read", "file:///project", true)]);
    let sibling = manifest(vec![capability("file.read", "file:///project/src2", true)]);
    let action_change = manifest(vec![capability("file.write", "file:///project/src", false)]);
    for (old, new) in [
        (&exact, &subtree),
        (&subtree, &wider),
        (&subtree, &sibling),
        (&exact, &action_change),
    ] {
        let diff = PermissionDiff::between(old, new).unwrap();
        assert!(diff.requires_review());
        assert_eq!(diff.added().len(), 1);
        assert_eq!(diff.removed().len(), 1);
    }
    for (old, new) in [(&wider, &subtree), (&subtree, &exact)] {
        let diff = PermissionDiff::between(old, new).unwrap();
        assert!(!diff.requires_review());
        assert!(!diff.is_empty());
    }
}
