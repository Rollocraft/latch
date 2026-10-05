use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn approval_requires_a_valid_original_requester() {
    assert_eq!(
        AuthorizationRequest::new(Operation::ApproveAction, resource("tenant-a", None), None),
        Err(AuthorizationValidationError::MissingApprovalRequester)
    );
    assert_eq!(
        AuthorizationRequest::new(
            Operation::ReadAudit,
            resource("tenant-a", None),
            Some("requester".into())
        ),
        Err(AuthorizationValidationError::UnexpectedApprovalRequester)
    );
    assert_eq!(
        AuthorizationRequest::new(
            Operation::ApproveAction,
            resource("tenant-a", None),
            Some(" ".into())
        ),
        Err(AuthorizationValidationError::InvalidAttribute(
            AuthorizationField::ApprovalRequester
        ))
    );
}
