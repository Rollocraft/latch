use super::protocol_test_support::*;
use super::*;

#[test]
fn request_roundtrip_preserves_caller_claims_and_arguments() {
    let request = request();
    let bytes = encode_request(&request).unwrap();
    assert_eq!(decode_request(&bytes).unwrap(), request);
    assert_eq!(serde_json::from_slice::<Request>(&bytes).unwrap(), request);
    assert!(!format!("{request:?}").contains("tenant"));
}
