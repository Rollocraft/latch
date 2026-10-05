use latch_ipc::*;
#[path = "support/frame_fixtures.rs"]
mod frame_fixtures;
use frame_fixtures::*;
use std::io::Cursor;

#[test]
fn eof_is_clean_only_at_frame_boundary() {
    let bytes = frame(b"12345");
    assert_eq!(
        read_json::<_, u64>(&mut &bytes[..0], MAX_FRAME_BYTES),
        Ok(None)
    );
    for length in 1..bytes.len() {
        let expected = if length < 4 {
            CodecError::TruncatedHeader
        } else {
            CodecError::TruncatedPayload
        };
        assert_eq!(
            read_json::<_, u64>(&mut &bytes[..length], MAX_FRAME_BYTES),
            Err(expected)
        );
    }
}

#[test]
fn rejects_lengths_before_reading_payload() {
    for (length, error) in [
        (0, CodecError::EmptyFrame),
        (65, CodecError::FrameTooLarge),
        (u32::MAX, CodecError::FrameTooLarge),
    ] {
        let mut bytes = length.to_be_bytes().to_vec();
        bytes.extend_from_slice(b"unread");
        let mut reader = Cursor::new(bytes);
        assert_eq!(read_json::<_, Message>(&mut reader, 64), Err(error));
        assert_eq!(reader.position(), 4);
    }
    for maximum in [0, MAX_FRAME_BYTES + 1, usize::MAX] {
        let mut reader = Cursor::new(frame(b"1"));
        assert_eq!(
            read_json::<_, u64>(&mut reader, maximum),
            Err(CodecError::InvalidLimit)
        );
        assert_eq!(reader.position(), 0);
        let mut output = Vec::new();
        assert_eq!(
            write_json(&mut output, &1, maximum),
            Err(CodecError::InvalidLimit)
        );
        assert!(output.is_empty());
    }
}
