use crate::merge_rule_effects::merge_rule_effects;
use crate::{Decision, Outcome, PolicyEngine, Reason};
use latch_core::{Action, Session};

impl PolicyEngine {
    /// The trusted caller supplies every applicable policy to the engine.
    /// All supplied policies apply; session references cannot remove a company
    /// rule, and an unavailable referenced policy causes denial.
    pub fn evaluate(&self, session: &Session, action: &Action, now: u64) -> Decision {
        let mut decision = Decision {
            outcome: Outcome::Deny,
            reason: Reason::DefaultDeny,
            matches: vec![],
            limits: Default::default(),
        };
        if let Err(error) = session.validate_action(action, now) {
            decision.reason = Reason::InvalidAction(error);
            return decision;
        }
        for id in session.policies() {
            if !self.policies.iter().any(|p| &p.id == id) {
                decision.reason = Reason::MissingPolicy(id.clone());
                return decision;
            }
        }
        merge_rule_effects(&self.policies, action, &mut decision);
        decision
    }
}
