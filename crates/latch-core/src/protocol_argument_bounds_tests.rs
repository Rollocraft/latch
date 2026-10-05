use super::protocol_test_support::*;
use super::*;
use serde_json::json;

#[test]
fn string_and_argument_bounds_apply_to_decode_and_encode() {
    for (path, value) in [
        ("/request_id", json!("x".repeat(MAX_ID_BYTES + 1))),
        ("/idempotency_id", json!("")),
        ("/caller/tenant", json!(" ")),
        ("/caller/actor", json!("x\n")),
        ("/caller/session", json!("é".repeat(MAX_ID_BYTES / 2 + 1))),
        ("/action/name", json!("file..read")),
        (
            "/action/resource",
            json!("x".repeat(MAX_RESOURCE_BYTES + 1)),
        ),
        ("/action/environment", json!("")),
        ("/action/arguments", json!(vec![""; MAX_ARGUMENTS + 1])),
        (
            "/action/arguments",
            json!(["x".repeat(MAX_ARGUMENT_BYTES + 1)]),
        ),
        ("/action/arguments", json!(["\u{0}"])),
        (
            "/action/arguments",
            json!(vec![
                "x".repeat(MAX_ARGUMENT_BYTES);
                MAX_TOTAL_ARGUMENT_BYTES / MAX_ARGUMENT_BYTES + 1
            ]),
        ),
    ] {
        let mut object = serde_json::to_value(request()).unwrap();
        *object.pointer_mut(path).unwrap() = value;
        let bytes = serde_json::to_vec(&object).unwrap();
        assert!(decode_request(&bytes).is_err(), "{path}");
        assert!(serde_json::from_slice::<Request>(&bytes).is_err(), "{path}");
    }
    let mut request = request();
    request.request_id = "x".repeat(MAX_ID_BYTES);
    request.action.resource = "x".repeat(MAX_RESOURCE_BYTES);
    request.action.arguments =
        vec!["x".repeat(MAX_ARGUMENT_BYTES); MAX_TOTAL_ARGUMENT_BYTES / MAX_ARGUMENT_BYTES];
    assert!(encode_request(&request).is_ok());
    request.action.arguments =
        vec!["\n".repeat(MAX_ARGUMENT_BYTES); MAX_TOTAL_ARGUMENT_BYTES / MAX_ARGUMENT_BYTES];
    assert!(encode_request(&request).is_ok());
    request.action.arguments =
        vec!["\u{1}".repeat(MAX_ARGUMENT_BYTES); MAX_TOTAL_ARGUMENT_BYTES / MAX_ARGUMENT_BYTES];
    assert_eq!(
        encode_request(&request),
        Err(ProtocolError::MessageTooLarge)
    );
}
