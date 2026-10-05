use latch_ipc::*;
#[path = "support/frame_fixtures.rs"]
mod frame_fixtures;
use frame_fixtures::*;

#[test]
fn rejects_invalid_json_and_unknown_fields_without_echoing_input() {
    for payload in [
        b"{\"text\":\"secret\",\"unknown\":true}".as_slice(),
        b"{",
        b"\xff",
        b"{} {}",
        b"null",
    ] {
        let bytes = frame(payload);
        let error = read_json::<_, Message>(&mut bytes.as_slice(), MAX_FRAME_BYTES).unwrap_err();
        assert_eq!(error, CodecError::InvalidJson);
        assert!(!format!("{error:?} {error}").contains("secret"));
    }
}
