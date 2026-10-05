use crate::presentation_test_fixtures::*;
use crate::*;

#[test]
fn context_is_complete_and_payloads_are_absent() {
    let view = render_approval(&session(), &action(), &decision());
    for expected in [
        "session-1",
        "acme",
        "human-owner",
        "agent://acme/coder",
        "production",
        "action-1",
        "git.push",
        "repo/main",
        "[IRREVERSIBLE]",
        "85/100",
        "ApprovalRequired",
        "production-policy",
        "Company",
        "review-push",
        "Production branch requires human review",
        "default: deny",
        "presentation only",
    ] {
        assert!(
            view.rendered().text.contains(expected),
            "missing {expected}"
        );
    }
    assert!(!view.rendered().text.contains("SECRET_TOKEN"));
    assert!(!view.rendered().text.contains("payload-contents"));
    assert!(view.allow_once_available());
    assert_eq!(
        view.parse_response("allow once"),
        Ok(ApprovalResponse::AllowOnce)
    );
    assert_safe(&view.rendered().text);
}
