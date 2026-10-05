use super::{DestinationTrust, Environment, Sensitivity};
use crate::Reversibility;

pub(super) fn environment_points(value: Environment) -> u8 {
    match value {
        Environment::Development => 0,
        Environment::Staging => 15,
        Environment::Production | Environment::Unknown => 45,
    }
}

pub(super) fn sensitivity_points(value: Sensitivity) -> u8 {
    match value {
        Sensitivity::Public => 0,
        Sensitivity::Internal => 10,
        Sensitivity::Confidential => 25,
        Sensitivity::Restricted | Sensitivity::Unknown => 40,
    }
}

pub(super) fn reversibility_points(value: Option<Reversibility>) -> u8 {
    match value {
        Some(Reversibility::FullyReversible) => 0,
        Some(Reversibility::PartiallyReversible) => 20,
        Some(Reversibility::Irreversible) | None => 35,
    }
}

pub(super) fn destination_points(value: DestinationTrust) -> u8 {
    match value {
        DestinationTrust::Trusted => 0,
        DestinationTrust::Known => 5,
        DestinationTrust::Unverified => 20,
        DestinationTrust::Untrusted | DestinationTrust::Unknown => 35,
    }
}
