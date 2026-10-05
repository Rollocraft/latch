#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    pub text: String,
}

pub struct Chunked<T> {
    pub inner: T,
    pub interrupt: bool,
}

impl<T: Read> Read for Chunked<T> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.interrupt = !self.interrupt;
        if self.interrupt {
            return Err(io::ErrorKind::Interrupted.into());
        }
        let length = bytes.len().min(1);
        self.inner.read(&mut bytes[..length])
    }
}

impl<T: Write> Write for Chunked<T> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.interrupt = !self.interrupt;
        if self.interrupt {
            return Err(io::ErrorKind::Interrupted.into());
        }
        self.inner.write(&bytes[..bytes.len().min(1)])
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub fn frame(payload: &[u8]) -> Vec<u8> {
    let mut bytes = (payload.len() as u32).to_be_bytes().to_vec();
    bytes.extend_from_slice(payload);
    bytes
}
