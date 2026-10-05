use super::query_test_fixtures::*;
use crate::*;
use serde_json::Value;

#[test]
fn jsonl_escapes_all_exported_strings_without_record_injection() {
    let dangerous = "quote\" slash\\ newline\nreturn\rtab\tzero\0雪\u{2028}";
    let organization = "tenant\"\\雪";
    let mut e = event(organization, u64::MAX, EventResult::Succeeded);
    for text in [
        &mut e.owner,
        &mut e.agent,
        &mut e.session,
        &mut e.action_id,
        &mut e.action,
        &mut e.resource,
        &mut e.environment,
    ] {
        *text = dangerous.into();
    }
    e.risk = risk(100);
    let events = [e.clone(), e];
    let mut bytes = Vec::new();
    let result = AuditReader::new(&events)
        .export_jsonl(
            &query(organization),
            page(0, 2),
            ResourceExportPolicy::Include,
            &mut bytes,
        )
        .unwrap();
    assert_eq!(result.written, 2);
    assert_eq!(bytes.iter().filter(|b| **b == b'\n').count(), 2);
    assert!(bytes.ends_with(b"\n"));
    for line in String::from_utf8(bytes).unwrap().lines() {
        let value: Value = serde_json::from_str(line).unwrap();
        assert_eq!(value["organization"], organization);
        assert_eq!(value["timestamp"].as_u64(), Some(u64::MAX));
        assert_eq!(value["risk"], 100);
        for key in [
            "owner",
            "agent",
            "session",
            "action_id",
            "action",
            "resource",
            "environment",
        ] {
            assert_eq!(value[key], dangerous);
        }
    }
}
