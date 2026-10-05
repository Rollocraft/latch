use crate::{DomainName, NetworkError};
use std::net::IpAddr;

/// One resolution: the chain of names followed, and what it produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    /// The queried name first, then each CNAME target in order.
    pub chain: Vec<DomainName>,
    pub addresses: Vec<IpAddr>,
    pub resolved_at: u64,
    pub expires_at: u64,
}

/// What the runtime's resolver has produced for this session, and for how long
/// it counts. Binding decisions to this ledger is what stops an agent from
/// connecting to an address nobody looked up, and what keeps a name that has
/// since been re-pointed from carrying its old address along.
#[derive(Debug, Default, Clone)]
pub struct Ledger {
    resolutions: Vec<Resolution>,
}

/// The longest a resolution may be honoured regardless of the TTL a server
/// claims, so that a hostile server cannot pin an address indefinitely.
pub const MAXIMUM_TTL: u64 = 600;

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record what the resolver returned. `ttl` is clamped to [`MAXIMUM_TTL`].
    pub fn record(
        &mut self,
        chain: Vec<DomainName>,
        addresses: Vec<IpAddr>,
        now: u64,
        ttl: u64,
    ) -> Result<(), NetworkError> {
        if chain.is_empty() || addresses.is_empty() {
            return Err(NetworkError::EmptyResolution);
        }
        if ttl == 0 {
            return Err(NetworkError::InvalidTtl);
        }
        let expires_at = now
            .checked_add(ttl.min(MAXIMUM_TTL))
            .ok_or(NetworkError::InvalidTtl)?;
        // A fresh answer replaces the old one for that name: re-pointing a name
        // must not leave the previous address usable.
        let queried = chain[0].clone();
        self.resolutions
            .retain(|resolution| resolution.chain.first() != Some(&queried));
        self.resolutions.push(Resolution {
            chain,
            addresses,
            resolved_at: now,
            expires_at,
        });
        Ok(())
    }

    pub fn resolutions(&self) -> &[Resolution] {
        &self.resolutions
    }

    fn live(&self, now: u64) -> impl Iterator<Item = &Resolution> {
        self.resolutions
            .iter()
            .filter(move |resolution| resolution.resolved_at <= now && now < resolution.expires_at)
    }

    pub(crate) fn resolutions_for_name(&self, name: &DomainName, now: u64) -> Vec<&Resolution> {
        self.live(now)
            .filter(|resolution| resolution.chain.contains(name))
            .collect()
    }

    pub(crate) fn resolutions_for_address(&self, address: &IpAddr, now: u64) -> Vec<&Resolution> {
        self.live(now)
            .filter(|resolution| resolution.addresses.contains(address))
            .collect()
    }

    pub fn forget_expired(&mut self, now: u64) {
        self.resolutions
            .retain(|resolution| now < resolution.expires_at);
    }
}
