#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    EmptyName,
    NameTooLong,
    InvalidLabel,
    /// An address written where a name belongs. Addresses are never patterns:
    /// they are what a name must resolve to.
    AddressAsName,
    InvalidPattern,
    DuplicateRule,
    EmptyResolution,
    InvalidTtl,
}
