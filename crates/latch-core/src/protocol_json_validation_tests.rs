use super::protocol_test_support::*;
use super::*;
use serde_json::json;

#[test]
fn rejects_unknown_duplicate_missing_and_malformed_fields() {
    for path in ["", "/caller", "/action"] {
        let mut value = serde_json::to_value(request()).unwrap();
        value
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("secret".into(), json!("not echoed"));
        let bytes = serde_json::to_vec(&value).unwrap();
        assert_eq!(decode_request(&bytes), Err(ProtocolError::InvalidJson));
        assert!(serde_json::from_slice::<Request>(&bytes).is_err());
    }
    let bytes = encode_request(&request()).unwrap();
    let duplicate = String::from_utf8(bytes.clone())
        .unwrap()
        .replacen('{', "{\"version\":1,", 1);
    assert_eq!(
        decode_request(duplicate.as_bytes()),
        Err(ProtocolError::InvalidJson)
    );
    for bytes in [b"{}".as_slice(), b"null", b"[]", b"{", b"\xff"] {
        assert_eq!(decode_request(bytes), Err(ProtocolError::InvalidJson));
    }
    let mut trailing = bytes;
    trailing.extend_from_slice(b" {}");
    assert_eq!(decode_request(&trailing), Err(ProtocolError::InvalidJson));
}
