use super::*;

#[test]
fn responses_roundtrip_all_outcomes() {
    for outcome in [
        Outcome::Decision {
            decision: Decision::Allowed,
            reasons: vec![DecisionReason::PolicyAllowed],
        },
        Outcome::Decision {
            decision: Decision::Denied,
            reasons: vec![DecisionReason::PolicyDenied],
        },
        Outcome::Decision {
            decision: Decision::ApprovalRequired,
            reasons: vec![DecisionReason::ApprovalRequired],
        },
        Outcome::Error {
            code: ErrorCode::UnsupportedVersion,
            reasons: vec![DecisionReason::UnsupportedVersion],
        },
    ] {
        let response = Response {
            version: VERSION,
            request_id: Some("r1".into()),
            outcome,
        };
        let bytes = encode_response(&response).unwrap();
        assert_eq!(decode_response(&bytes).unwrap(), response);
        assert_eq!(
            serde_json::from_slice::<Response>(&bytes).unwrap(),
            response
        );
    }
}
