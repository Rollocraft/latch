use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CapabilityToken([u8; 32]);

impl fmt::Debug for CapabilityToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CapabilityToken([REDACTED])")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidCapabilityTokenLength;

impl CapabilityToken {
    pub fn import_secret(bytes: &[u8]) -> Result<Self, InvalidCapabilityTokenLength> {
        bytes
            .try_into()
            .map(Self)
            .map_err(|_| InvalidCapabilityTokenLength)
    }

    pub fn export_secret(&self) -> [u8; 32] {
        self.0
    }

    pub(super) fn generate() -> Result<Self, getrandom::Error> {
        Self::generate_with(getrandom::fill)
    }

    fn generate_with(
        fill: impl FnOnce(&mut [u8]) -> Result<(), getrandom::Error>,
    ) -> Result<Self, getrandom::Error> {
        let mut bytes = [0; 32];
        fill(&mut bytes)?;
        Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_import_bounds_and_random_failure() {
        for length in [0, 1, 31, 33, 64, 1024] {
            assert_eq!(
                CapabilityToken::import_secret(&vec![7; length]),
                Err(InvalidCapabilityTokenLength)
            );
        }
        let token = CapabilityToken::import_secret(&[7; 32]).unwrap();
        assert_eq!(token.export_secret(), [7; 32]);
        assert_eq!(format!("{token:#?}"), "CapabilityToken([REDACTED])");
        assert!(
            CapabilityToken::generate_with(|bytes| {
                bytes.fill(7);
                Err(getrandom::Error::UNSUPPORTED)
            })
            .is_err()
        );
    }
}
