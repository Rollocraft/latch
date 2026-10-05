use super::*;

#[test]
fn streaming_is_independent_of_chunking() {
    let data: Vec<u8> = (0..1000u32).map(|i| (i % 251) as u8).collect();
    let expected = sha256(&data);
    for size in [1, 7, 55, 56, 63, 64, 65, 128, 999] {
        let mut hasher = Sha256::new();
        for part in data.chunks(size) {
            hasher.update(part);
        }
        assert_eq!(hasher.finish(), expected, "chunk size {size}");
    }
}
