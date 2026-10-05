use crate::presentation_test_fixtures::*;
use crate::*;
use latch_core::SessionState;
use latch_policy::{Outcome, Reason};

#[test]
fn denial_and_invalid_attribution_cannot_offer_allow() {
    let mut denied = decision();
    denied.outcome = Outcome::Deny;
    denied.reason = Reason::DefaultDeny;
    let view = render_approval(&session(), &action(), &denied);
    assert!(view.rendered().text.contains("default deny"));
    assert_eq!(
        view.parse_response("allow once"),
        Err(ResponseError::AllowUnavailable)
    );
    assert_eq!(view.parse_response(""), Ok(ApprovalResponse::Deny));
    for field in 0..3 {
        let mut action = action();
        match field {
            0 => action.actor = "agent://attacker/forged".into(),
            1 => action.session_id = "forged-session".into(),
            _ => action.environment = "forged-environment".into(),
        }
        let view = render_approval(&session(), &action, &decision());
        assert!(!view.allow_once_available());
        assert!(view.rendered().text.contains("INVALID CONTEXT"));
        assert!(view.rendered().text.contains("human-owner"));
        assert!(!view.rendered().text.contains("forged"));
    }
    let mut frozen = session();
    frozen.transition(SessionState::Frozen, 20).unwrap();
    assert!(!render_approval(&frozen, &action(), &decision()).allow_once_available());
}
