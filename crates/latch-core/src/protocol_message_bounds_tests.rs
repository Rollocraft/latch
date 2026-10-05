use super::protocol_test_support::*;
use super::*;

#[test]
fn versions_and_message_limits_are_typed() {
    for version in [0, 2, u32::MAX] {
        let mut request = request();
        request.version = version;
        let bytes = serde_json::to_vec(&request).unwrap();
        assert_eq!(
            decode_request(&bytes),
            Err(ProtocolError::UnsupportedVersion { received: version })
        );
        assert_eq!(
            encode_request(&request),
            Err(ProtocolError::UnsupportedVersion { received: version })
        );
    }
    assert_eq!(
        decode_request(&vec![b' '; MAX_MESSAGE_BYTES + 1]),
        Err(ProtocolError::MessageTooLarge)
    );
    let mut bytes = encode_request(&request()).unwrap();
    bytes.resize(MAX_MESSAGE_BYTES, b' ');
    assert!(decode_request(&bytes).is_ok());
}
