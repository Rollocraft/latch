use crate::{Decision, Destination, Host, Ledger, Outcome, Policy, Reason, Resolution, is_private};

impl Policy {
    /// Decide about a connection. The destination must correspond to a live
    /// resolution performed by the runtime: a bare address, or a name nobody
    /// resolved, is denied rather than guessed at.
    pub fn evaluate(&self, destination: &Destination, ledger: &Ledger, now: u64) -> Decision {
        let chains: Vec<&Resolution> = match &destination.host {
            Host::Name(name) => ledger.resolutions_for_name(name, now),
            Host::Address(address) => ledger.resolutions_for_address(address, now),
        };
        if chains.is_empty() {
            return Decision {
                outcome: Outcome::Deny,
                reason: match &destination.host {
                    Host::Name(_) => Reason::UnresolvedName,
                    Host::Address(_) => Reason::UnresolvedAddress,
                },
                names: match &destination.host {
                    Host::Name(name) => vec![name.clone()],
                    Host::Address(_) => Vec::new(),
                },
                matched: Vec::new(),
                decided_by: None,
            };
        }

        let mut names = Vec::new();
        let mut matched = Vec::new();
        let mut worst = Outcome::Allow;
        let mut reason = Reason::ExplicitAllow;
        let mut decided_by = None;
        // Every name in every chain that could have produced this connection is
        // judged, and the least permissive answer wins. A CNAME into a denied
        // domain denies, however the connection was addressed.
        for chain in chains {
            for address in &chain.addresses {
                if !self.allow_private_addresses
                    && is_private(address)
                    && matches!(&destination.host, Host::Address(target) if target == address)
                {
                    return Decision {
                        outcome: Outcome::Deny,
                        reason: Reason::PrivateAddress(*address),
                        names: chain.chain.clone(),
                        matched,
                        decided_by: None,
                    };
                }
            }
            if let Host::Name(_) = &destination.host
                && !self.allow_private_addresses
                && let Some(address) = chain.addresses.iter().find(|address| is_private(address))
            {
                return Decision {
                    outcome: Outcome::Deny,
                    reason: Reason::PrivateAddress(*address),
                    names: chain.chain.clone(),
                    matched,
                    decided_by: None,
                };
            }
            for name in &chain.chain {
                let decision = self.evaluate_name(name, destination);
                matched.extend(decision.matched);
                if !names.contains(name) {
                    names.push(name.clone());
                }
                let rank = |outcome: Outcome| match outcome {
                    Outcome::Allow => 0,
                    Outcome::Ask => 1,
                    Outcome::Deny => 2,
                };
                if rank(decision.outcome) > rank(worst) || decided_by.is_none() {
                    worst = decision.outcome;
                    reason = decision.reason;
                    decided_by = decision.decided_by;
                }
            }
        }
        Decision {
            outcome: worst,
            reason,
            names,
            matched,
            decided_by,
        }
    }
}
