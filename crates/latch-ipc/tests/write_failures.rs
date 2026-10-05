use latch_ipc::*;
use std::io::{self, Write};

struct FailingWriter {
    remaining: usize,
}

impl Write for FailingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let written = self.remaining.min(bytes.len());
        self.remaining -= written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        panic!("codec must not imply flushing or durability")
    }
}

#[test]
fn propagates_header_and_payload_write_failures() {
    for remaining in [0, 2, 4, 5] {
        assert_eq!(
            write_json(&mut FailingWriter { remaining }, &"abc", 64),
            Err(CodecError::Io(io::ErrorKind::WriteZero))
        );
    }
}
