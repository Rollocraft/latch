use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn diff_is_sorted_order_independent_and_serializable() {
    let a = capability("file.read", "file:///project", true);
    let b = capability("git.push", "repo://acme/backend", false);
    let c = capability("file.write", "file:///project/src", true);
    let first = manifest(vec![b.clone(), a.clone()]);
    let reordered = manifest(vec![a.clone(), b.clone()]);
    assert_eq!(first.to_json().unwrap(), reordered.to_json().unwrap());
    assert!(
        PermissionDiff::between(&first, &reordered)
            .unwrap()
            .is_empty()
    );
    let next = manifest(vec![c.clone(), a.clone()]);
    let diff = PermissionDiff::between(&first, &next).unwrap();
    assert_eq!(diff.added(), &[c]);
    assert_eq!(diff.removed(), &[b]);
    assert_eq!(diff.expanded(), diff.added());
    assert!(diff.requires_review());
    let initial = PermissionDiff::initial(&first);
    assert_eq!(
        initial.added(),
        &[a, capability("git.push", "repo://acme/backend", false)]
    );
    assert_eq!(
        serde_json::to_string(&diff).unwrap(),
        serde_json::to_string(&PermissionDiff::between(&reordered, &next).unwrap()).unwrap()
    );
}
