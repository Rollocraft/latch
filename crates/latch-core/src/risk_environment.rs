#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Environment {
    Development,
    Staging,
    Production,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Sensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DestinationTrust {
    Trusted,
    Known,
    Unverified,
    Untrusted,
    #[default]
    Unknown,
}
