use super::Digest;
use super::hash_compression::compress;

#[derive(Debug, Clone)]
pub struct Sha256 {
    state: [u32; 8],
    block: [u8; 64],
    buffered: usize,
    bytes: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            block: [0; 64],
            buffered: 0,
            bytes: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        self.bytes = self
            .bytes
            .checked_add(data.len() as u64)
            .expect("hashed length exceeds 2^64 bytes");
        if self.buffered > 0 {
            let take = data.len().min(64 - self.buffered);
            self.block[self.buffered..self.buffered + take].copy_from_slice(&data[..take]);
            self.buffered += take;
            data = &data[take..];
            if self.buffered == 64 {
                compress(&mut self.state, &self.block);
                self.buffered = 0;
            }
        }
        while data.len() >= 64 {
            let (block, rest) = data.split_at(64);
            compress(&mut self.state, block.try_into().expect("64 bytes"));
            data = rest;
        }
        self.block[self.buffered..self.buffered + data.len()].copy_from_slice(data);
        self.buffered += data.len();
    }

    pub fn finish(mut self) -> Digest {
        let bits = self.bytes.wrapping_mul(8);
        self.update(&[0x80]);
        while self.buffered != 56 {
            self.update(&[0]);
        }
        self.update(&bits.to_be_bytes());
        debug_assert_eq!(self.buffered, 0);
        let mut digest = [0u8; 32];
        for (chunk, word) in digest.as_chunks_mut::<4>().0.iter_mut().zip(self.state) {
            *chunk = word.to_be_bytes();
        }
        Digest::from_bytes(digest)
    }
}

pub fn sha256(data: &[u8]) -> Digest {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finish()
}
