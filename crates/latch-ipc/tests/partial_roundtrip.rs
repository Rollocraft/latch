use latch_ipc::*;
#[path = "support/frame_fixtures.rs"]
mod frame_fixtures;
use frame_fixtures::*;
use std::io::Cursor;

#[test]
fn roundtrip_partial_reads_writes_interruptions_and_multiple_frames() {
    let message = Message {
        text: "é and spaces".into(),
    };
    let mut writer = Chunked {
        inner: Vec::new(),
        interrupt: false,
    };
    write_json(&mut writer, &message, MAX_FRAME_BYTES).unwrap();
    assert_eq!(writer.inner, frame(&serde_json::to_vec(&message).unwrap()));
    write_json(&mut writer, &message, MAX_FRAME_BYTES).unwrap();
    let mut reader = Chunked {
        inner: Cursor::new(writer.inner),
        interrupt: false,
    };
    assert_eq!(
        read_json::<_, Message>(&mut reader, MAX_FRAME_BYTES).unwrap(),
        Some(Message {
            text: message.text.clone()
        })
    );
    assert_eq!(
        read_json::<_, Message>(&mut reader, MAX_FRAME_BYTES).unwrap(),
        Some(message)
    );
    assert_eq!(
        read_json::<_, Message>(&mut reader, MAX_FRAME_BYTES).unwrap(),
        None
    );
}
