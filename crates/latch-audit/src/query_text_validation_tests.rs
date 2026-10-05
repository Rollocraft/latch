use super::query_test_fixtures::*;
use crate::*;

#[test]
fn organization_is_required_and_text_is_validated_without_normalization() {
    for text in [
        "".into(),
        " \t".into(),
        "acme\n".into(),
        "acme\0".into(),
        "x".repeat(MAX_QUERY_TEXT_BYTES + 1),
    ] {
        assert_eq!(
            AuditQuery::new(text, AuditFilters::default()),
            Err(QueryError::InvalidText("organization"))
        );
    }
    let name = "é".repeat(MAX_QUERY_TEXT_BYTES / 2);
    assert_eq!(query(&name).organization(), name);
    for field in ["session", "agent", "owner", "environment"] {
        for text in ["", " ", "bad\rvalue", &"x".repeat(MAX_QUERY_TEXT_BYTES + 1)] {
            let mut filters = AuditFilters::default();
            match field {
                "session" => filters.session = Some(text.into()),
                "agent" => filters.agent = Some(text.into()),
                "owner" => filters.owner = Some(text.into()),
                _ => filters.environment = Some(text.into()),
            }
            assert_eq!(
                AuditQuery::new("acme", filters),
                Err(QueryError::InvalidText(field))
            );
        }
    }
    let events = [event("acme", 0, EventResult::Succeeded)];
    let reader = AuditReader::new(&events);
    for organization in ["ACME", " acme", "acme ", "*", "ac"] {
        assert_eq!(query(organization).organization(), organization);
        assert_eq!(
            reader
                .timeline(&query(organization), page(0, 10))
                .total_matches,
            0
        );
    }
}
