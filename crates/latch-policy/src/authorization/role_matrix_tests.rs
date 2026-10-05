use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn complete_role_matrix() {
    let expected = [
        [true, true, false, false, false, false, false],
        [true, true, true, true, true, false, true],
        [true, true, true, true, false, false, true],
        [true, true, true, true, true, true, true],
        [true, true, true, false, false, false, false],
        [true, true, false, false, true, false, false],
    ];
    for (role_index, role) in ROLES.into_iter().enumerate() {
        for (operation_index, operation) in OPERATIONS.into_iter().enumerate() {
            let decision = authorize(
                &principal([role]),
                &request(operation),
                &PermissionProfile::default(),
                10,
            );
            assert_eq!(
                decision.is_allowed(),
                expected[role_index][operation_index],
                "{role:?} {operation:?}: {decision}"
            );
        }
    }
}
