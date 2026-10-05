use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn decisions_explain_the_grant_or_failed_constraint_without_attribute_values() {
    let principal = principal([Role::Auditor]);
    let request = request(Operation::ReadAudit);
    let allowed = authorize(&principal, &request, &PermissionProfile::default(), 10);
    assert!(allowed.to_string().contains("Auditor"));
    let denied = authorize(
        &principal,
        &request,
        &profile(
            Operation::ReadAudit,
            AttributePredicate::Equals {
                attribute: ResourceAttribute::Team,
                value: "private-team".into(),
            },
        ),
        10,
    );
    assert_eq!(denied.to_string(), "denied: constraint 0 rejected Team");
}
