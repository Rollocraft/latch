use super::identity_test_support::*;
use super::*;

#[test]
fn sessions_require_policy_and_actions_require_semantic_names() {
    assert!(Session::new("s1".into(), identity(), vec![], 10).is_err());
    let s = session();
    for name in ["", "git", "git..push", "git.Push", "git.push\n"] {
        let mut a = action();
        a.name = name.into();
        assert_eq!(
            s.validate_action(&a, 20),
            Err(ValidationError::InvalidAction)
        );
    }
}
