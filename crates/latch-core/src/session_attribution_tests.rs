use super::identity_test_support::*;
use super::*;

#[test]
fn expired_identity_cannot_resume_but_can_be_cleaned_up() {
    let mut s = session();
    s.transition(SessionState::Frozen, 20).unwrap();
    assert_eq!(
        s.transition(SessionState::Active, 100),
        Err(ValidationError::ExpiredIdentity)
    );
    s.transition(SessionState::RolledBack, 100).unwrap();
}

#[test]
fn action_is_bound_to_actor_session_and_environment() {
    let s = session();
    assert!(s.validate_action(&action(), 20).is_ok());
    for field in 0..3 {
        let mut a = action();
        match field {
            0 => a.actor = "agent://other/coder".into(),
            1 => a.session_id = "other".into(),
            _ => a.environment = "production".into(),
        }
        assert_eq!(
            s.validate_action(&a, 20),
            Err(ValidationError::AttributionMismatch)
        );
    }
    let mut s = s;
    s.transition(SessionState::Frozen, 20).unwrap();
    assert_eq!(
        s.validate_action(&action(), 20),
        Err(ValidationError::InactiveSession)
    );
}
