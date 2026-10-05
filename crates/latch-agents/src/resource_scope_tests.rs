use crate::*;

#[test]
fn scope_membership_respects_path_authority_scheme_and_case() {
    let scope = ResourceScope::subtree(ResourceId::new("repo://acme/project").unwrap());
    for resource in [
        "repo://acme/project",
        "repo://acme/project/src",
        "repo://acme/project/src/lib.rs",
    ] {
        assert!(scope.contains(&ResourceId::new(resource).unwrap()));
    }
    for resource in [
        "repo://acme/project2",
        "repo://acme/Project",
        "repo://other/project",
        "file://acme/project",
        "repo://acme",
    ] {
        assert!(
            !scope.contains(&ResourceId::new(resource).unwrap()),
            "{resource}"
        );
    }
    let exact = ResourceScope::exact(scope.resource().clone());
    assert!(scope.covers(&exact));
    assert!(!exact.covers(&scope));
    assert!(!exact.contains(&ResourceId::new("repo://acme/project/src").unwrap()));
    assert_eq!(exact.kind(), ScopeKind::Exact);
}
