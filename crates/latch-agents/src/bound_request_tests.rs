use crate::manifest_test_fixtures::*;
use crate::*;
use latch_core::{Session, SessionState};

#[test]
fn bound_requests_revalidate_session_action_and_expiration() {
    let bound = sample().bind(&identity(), 20).unwrap();
    for field in 0..5 {
        let mut request = action();
        match field {
            0 => request.actor = "agent://acme/other".into(),
            1 => request.session_id = "other".into(),
            2 => request.environment = "production".into(),
            3 => request.name = "file.*".into(),
            _ => request.resource = "file:///project/../other".into(),
        }
        assert!(bound.requests_action(&session(), &request, 20).is_err());
    }
    for now in [9, 100] {
        assert!(bound.requests_action(&session(), &action(), now).is_err());
    }
    let mut frozen = session();
    frozen.transition(SessionState::Frozen, 20).unwrap();
    assert!(bound.requests_action(&frozen, &action(), 20).is_err());
    let mut runtime = identity();
    runtime.expires_at = 200;
    let other = Session::new("session-1".into(), runtime, vec!["default".into()], 10).unwrap();
    assert_eq!(
        bound.requests_action(&other, &action(), 20),
        Err(ManifestError::IdentityMismatch)
    );
}
