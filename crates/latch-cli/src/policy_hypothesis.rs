use latch_core::{Action, AgentIdentity, Reversibility, RiskScore, Session};
use latch_policy::{Decision, Outcome, PolicyDocument, Reason};

pub fn evaluate(document: PolicyDocument, name: &str, resource: &str) -> Result<Decision, String> {
    validate_name(name)?;
    // Session requires at least one policy reference. An empty document has no
    // matches and defaults to denial, rather than referencing an invented policy.
    if document.policies().is_empty() {
        return Ok(Decision {
            outcome: Outcome::Deny,
            reason: Reason::DefaultDeny,
            matches: Vec::new(),
            limits: Default::default(),
        });
    }
    let policies = document.policies().iter().map(|p| p.id.clone()).collect();
    let session = Session::new("offline-explain".into(), identity(), policies, 0)
        .map_err(|e| format!("cannot construct offline evaluation session: {e:?}"))?;
    let action = Action {
        id: "hypothetical".into(),
        actor: session.identity().id.clone(),
        session_id: session.id().into(),
        name: name.into(),
        resource: resource.into(),
        environment: "offline".into(),
        arguments: Vec::new(),
        reversibility: Reversibility::Irreversible,
        risk: RiskScore::new(0).unwrap(),
    };
    Ok(document.into_engine().evaluate(&session, &action, 0))
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.split('.').count() < 2
        || name.split('.').any(|part| {
            part.is_empty()
                || !part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        })
    {
        return Err(format!(
            "invalid action {name:?}: expected dot-separated lowercase names, e.g. file.read"
        ));
    }
    Ok(())
}

fn identity() -> AgentIdentity {
    AgentIdentity {
        id: "agent://offline/explain".into(),
        organization: "offline".into(),
        team: "offline".into(),
        owner: "offline".into(),
        purpose: "offline evaluation".into(),
        provider: "none".into(),
        model: "none".into(),
        model_version: "none".into(),
        runtime: "offline".into(),
        environment: "offline".into(),
        device: "offline".into(),
        created_at: 0,
        expires_at: 1,
        trust_level: 0,
    }
}
