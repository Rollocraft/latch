use super::query_test_fixtures::*;
use crate::*;

#[test]
fn exports_minimize_resources_and_never_mutate_original_events() {
    let resources = [
        "https://user:password@example.test/private?token=secret#fragment",
        "C:\\Users\\private\\secret.txt",
        "../../secret",
        "秘密/secret",
        "",
        "\n\"secret\"\u{0}",
    ];
    for resource in resources {
        let mut e = event("acme", 0, EventResult::Succeeded);
        e.resource = resource.into();
        let events = [e.clone()];
        let reader = AuditReader::new(&events);
        for policy in [
            ResourceExportPolicy::default(),
            ResourceExportPolicy::Redact,
            ResourceExportPolicy::Include,
        ] {
            let values = export(&reader, &query("acme"), policy);
            let value = &values[0];
            assert!(value.get("policies").is_none());
            assert!(value.get("arguments").is_none());
            match policy {
                ResourceExportPolicy::Omit => assert!(value.get("resource").is_none()),
                ResourceExportPolicy::Redact => assert_eq!(value["resource"], "[REDACTED]"),
                ResourceExportPolicy::Include => assert_eq!(value["resource"], resource),
            }
        }
        assert_eq!(events[0], e);
        assert_eq!(
            reader.timeline(&query("acme"), page(0, 1)).events[0].resource,
            resource
        );
    }
}
