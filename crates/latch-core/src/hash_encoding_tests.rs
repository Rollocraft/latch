use super::*;

#[test]
fn hex_round_trips_and_rejects_ambiguous_encodings() {
    let digest = sha256(b"latch");
    assert_eq!(Digest::parse_hex(&digest.to_hex()), Some(digest));
    assert_eq!(Digest::parse_hex(&digest.to_hex().to_uppercase()), None);
    for text in ["", "00", &"0".repeat(63), &"0".repeat(65), &"g".repeat(64)] {
        assert_eq!(Digest::parse_hex(text), None);
    }
    assert_eq!(
        Digest::parse_hex(&"0".repeat(64)),
        Some(Digest::from_bytes([0; 32]))
    );
}
