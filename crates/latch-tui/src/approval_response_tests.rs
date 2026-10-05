use crate::*;

#[test]
fn parser_defaults_to_deny_and_rejects_ambiguous_input() {
    assert_eq!(ApprovalResponse::default(), ApprovalResponse::Deny);
    for input in ["", " ", "\n", "\r\n", "deny", " DENY \r\n"] {
        assert_eq!(input.parse(), Ok(ApprovalResponse::Deny));
    }
    for input in ["allow once", " ALLOW ONCE ", "allow once\r\n"] {
        assert_eq!(input.parse(), Ok(ApprovalResponse::AllowOnce));
    }
    for input in [
        "yes",
        "y",
        "allow",
        "allow session",
        "allow once deny",
        "allow once\n\n",
        "allow once\r",
        "\tallow once",
        "allow\tonce",
        "allow once\0",
        "allow once\x1b",
        "allow once\u{200b}",
        "allow\u{a0}once",
        "deny\nallow once",
    ] {
        assert_eq!(
            input.parse::<ApprovalResponse>(),
            Err(ResponseError::InvalidInput),
            "{input:?}"
        );
    }
    assert_eq!(
        " ".repeat(100_000).parse::<ApprovalResponse>(),
        Err(ResponseError::TooLong)
    );
    assert!(!ResponseError::InvalidInput.to_string().contains("SECRET"));
}
