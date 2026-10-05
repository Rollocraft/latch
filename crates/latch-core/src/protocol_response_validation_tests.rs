use super::*;
use serde_json::{Value, json};

#[test]
fn response_validation_is_strict() {
    let value = json!({"version": 1, "request_id": "r1", "outcome": {"kind": "decision", "decision": "denied", "reasons": ["policy_denied"]}});
    for (path, replacement) in [
        ("/version", json!(2)),
        ("/request_id", Value::Null),
        ("/outcome/decision", json!("maybe")),
        ("/outcome/reasons", json!([])),
        (
            "/outcome/reasons",
            json!(vec!["policy_denied"; MAX_REASONS + 1]),
        ),
    ] {
        let mut invalid = value.clone();
        *invalid.pointer_mut(path).unwrap() = replacement;
        assert!(decode_response(&serde_json::to_vec(&invalid).unwrap()).is_err());
    }
    for path in ["", "/outcome"] {
        let mut invalid = value.clone();
        invalid
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("capability".into(), json!("secret"));
        assert_eq!(
            decode_response(&serde_json::to_vec(&invalid).unwrap()),
            Err(ProtocolError::InvalidJson)
        );
    }
    let response = Response {
        version: VERSION,
        request_id: None,
        outcome: Outcome::Error {
            code: ErrorCode::InvalidRequest,
            reasons: vec![DecisionReason::InvalidRequest],
        },
    };
    assert!(encode_response(&response).is_ok());
}
