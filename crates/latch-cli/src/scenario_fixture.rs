use crate::scenario_input::{ReversibilityInput, Scenario};
use latch_core::{Action, Reversibility, RiskScore, Session};
use latch_policy::Outcome;

pub(crate) struct Fixture {
    pub(crate) id: String,
    pub(crate) now: u64,
    pub(crate) session: Session,
    pub(crate) action: Action,
    pub(crate) expected: Outcome,
}

impl TryFrom<Scenario> for Fixture {
    type Error = String;

    fn try_from(input: Scenario) -> Result<Self, Self::Error> {
        let session = Session::new(
            input.session.id,
            input.session.identity.into(),
            input.session.policies,
            input.now,
        )
        .map_err(|e| format!("invalid session in scenario {:?}: {e:?}", input.id))?;
        let action = Action {
            id: input.action.id,
            actor: session.identity().id.clone(),
            session_id: session.id().into(),
            name: input.action.name,
            resource: input.action.resource,
            environment: session.identity().environment.clone(),
            arguments: input.action.arguments,
            risk: RiskScore::new(input.action.risk).ok_or("risk must be between 0 and 100")?,
            reversibility: match input.action.reversibility {
                ReversibilityInput::FullyReversible => Reversibility::FullyReversible,
                ReversibilityInput::PartiallyReversible => Reversibility::PartiallyReversible,
                ReversibilityInput::Irreversible => Reversibility::Irreversible,
            },
        };
        session
            .validate_action(&action, input.now)
            .map_err(|e| format!("invalid action in scenario {:?}: {e:?}", input.id))?;
        Ok(Self {
            id: input.id,
            now: input.now,
            session,
            action,
            expected: input.expected.into(),
        })
    }
}
