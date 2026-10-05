use latch_ipc::*;
#[path = "support/frame_fixtures.rs"]
mod frame_fixtures;
use frame_fixtures::*;

#[test]
fn enforces_exact_encoded_byte_limit_before_any_write() {
    let mut output = Vec::new();
    write_json(&mut output, &"abc", 5).unwrap();
    assert_eq!(output, frame(b"\"abc\""));
    assert_eq!(
        read_json::<_, String>(&mut output.as_slice(), 5).unwrap(),
        Some("abc".into())
    );
    output.clear();
    assert_eq!(
        write_json(&mut output, &"abc", 4),
        Err(CodecError::FrameTooLarge)
    );
    assert!(output.is_empty());
    assert_eq!(
        write_json(
            &mut output,
            &"\u{1}".repeat(MAX_FRAME_BYTES),
            MAX_FRAME_BYTES
        ),
        Err(CodecError::FrameTooLarge)
    );
    assert!(output.is_empty());
}
