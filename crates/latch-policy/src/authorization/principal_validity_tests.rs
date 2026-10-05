use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn validity_is_half_open_for_every_role() {
    for role in ROLES {
        let principal = principal([role]);
        let request = request(Operation::ReadPolicy);
        let profile = PermissionProfile::default();
        for now in [0, 9] {
            assert_eq!(
                authorize(&principal, &request, &profile, now),
                AuthorizationDecision::Denied(AuthorizationDenial::PrincipalNotYetValid)
            );
        }
        for now in [10, 19] {
            assert!(authorize(&principal, &request, &profile, now).is_allowed());
        }
        for now in [20, 21, u64::MAX] {
            assert_eq!(
                authorize(&principal, &request, &profile, now),
                AuthorizationDecision::Denied(AuthorizationDenial::PrincipalExpired)
            );
        }
    }
}
