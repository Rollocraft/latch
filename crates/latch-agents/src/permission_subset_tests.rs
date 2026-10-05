use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn no_review_implies_no_new_requests_for_finite_scope_combinations() {
    let capabilities = [
        capability("file.read", "file:///project", false),
        capability("file.read", "file:///project", true),
        capability("file.read", "file:///project/src", false),
        capability("file.write", "file:///project", true),
    ];
    let manifests: Vec<_> = (0..16)
        .map(|mask| {
            manifest(
                capabilities
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| mask & (1 << index) != 0)
                    .map(|(_, capability)| capability.clone())
                    .collect(),
            )
        })
        .collect();
    for old in &manifests {
        for new in &manifests {
            let diff = PermissionDiff::between(old, new).unwrap();
            if diff.requires_review() {
                continue;
            }
            for action in ["file.read", "file.write", "file.delete"] {
                let action = ActionName::new(action).unwrap();
                for resource in [
                    "file:///project",
                    "file:///project/src",
                    "file:///project/src/a",
                    "file:///project2",
                    "file:///other",
                ] {
                    let resource = ResourceId::new(resource).unwrap();
                    assert!(
                        !new.requests(&action, &resource, "development")
                            || old.requests(&action, &resource, "development")
                    );
                }
            }
        }
    }
}
