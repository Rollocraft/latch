use super::*;

#[test]
fn terminal_sessions_cannot_resume() {
    for terminal in [
        SessionState::Committed,
        SessionState::RolledBack,
        SessionState::Failed,
    ] {
        let mut session = SessionLifecycle::new(10);
        session.transition(terminal, 20).unwrap();
        assert_eq!(session.ended_at(), Some(20));
        for next in [
            SessionState::Active,
            SessionState::Frozen,
            SessionState::Committed,
            SessionState::RolledBack,
            SessionState::Failed,
        ] {
            assert_eq!(
                session.transition(next, 30),
                Err(TransitionError::InvalidTransition)
            );
            assert_eq!(session.state(), terminal);
        }
    }
}

#[test]
fn freeze_blocks_commit_but_allows_rollback() {
    let mut session = SessionLifecycle::new(10);
    session.transition(SessionState::Frozen, 20).unwrap();
    assert_eq!(
        session.transition(SessionState::Committed, 21),
        Err(TransitionError::InvalidTransition)
    );
    session.transition(SessionState::RolledBack, 22).unwrap();
    assert_eq!(session.ended_at(), Some(22));
}

#[test]
fn rejected_time_reversal_preserves_state() {
    let mut session = SessionLifecycle::new(10);
    session.transition(SessionState::Frozen, 20).unwrap();
    let previous = session.clone();
    assert_eq!(
        session.transition(SessionState::Active, 19),
        Err(TransitionError::TimeReversed)
    );
    assert_eq!(session, previous);
    session.transition(SessionState::Active, 21).unwrap();
    assert_eq!(session.state(), SessionState::Active);
    assert_eq!(session.ended_at(), None);
}
