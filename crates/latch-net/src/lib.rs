//! Egress policy: which destinations a session may reach, and how much may
//! leave through them.
//!
//! The rule that shapes this whole crate is that a name is the unit a policy
//! talks about, but an address is what a socket connects to. An agent that can
//! choose its own address can otherwise walk around every name-based rule, so
//! nothing here decides on a destination that the runtime's own resolver did
//! not produce: an unresolved name is denied, a bare address is denied, and a
//! resolution stops counting when it expires.
//!
//! This crate decides. It opens no sockets and enforces nothing by itself: an
//! agent that can reach the network directly is outside what it can promise,
//! and that confinement belongs to the sandbox.

mod address_scope;
mod connection_policy;
mod connection_totals;
mod destination;
mod domain_name;
mod domain_pattern;
mod egress_budget;
mod egress_decision;
mod egress_rule;
mod exfiltration;
mod name_policy;
mod network_error;
mod resolution_ledger;

pub use address_scope::{is_loopback, is_private};
pub use connection_totals::{Connection, totals};
pub use destination::{Destination, Host, Protocol};
pub use domain_name::DomainName;
pub use domain_pattern::Pattern;
pub use egress_budget::{Budget, BudgetError, Limits};
pub use egress_decision::{Decision, Outcome, Reason};
pub use egress_rule::{Effect, Rule};
pub use exfiltration::{ExfiltrationMonitor, Response, SensitiveRead};
pub use name_policy::Policy;
pub use network_error::NetworkError;
pub use resolution_ledger::{Ledger, MAXIMUM_TTL, Resolution};
