use crate::command_test_support::{invoke, temporary};

#[test]
fn populated_audit_output_escapes_terminal_controls() {
    use latch_audit::{AuditEvent, AuditSink, EventResult, FileAudit};
    let path = temporary("event").with_extension("bin");
    let mut sink = FileAudit::create(&path).unwrap();
    sink.append(&AuditEvent {
        timestamp: 42,
        organization: "acme".into(),
        owner: "david".into(),
        agent: "agent://acme/coder".into(),
        session: "s1".into(),
        action_id: "a1".into(),
        action: "file.read".into(),
        resource: "file\nforged event\u{1b}[2J".into(),
        environment: "dev".into(),
        risk: latch_core::RiskScore::new(2).unwrap(),
        result: EventResult::Succeeded,
        policies: vec!["default".into()],
    })
    .unwrap();
    let head = sink.head();
    drop(sink);
    let output = invoke(&[&"audit".into(), &path.as_os_str().into()]).unwrap();
    assert_eq!(output.lines().count(), 2);
    assert!(!output.contains('\u{1b}'));
    assert!(output.contains("Succeeded"));
    assert!(output.contains("david"));
    assert!(output.contains("risk=2"));
    assert!(output.contains(&format!("chain {}", head.to_hex())));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn reads_empty_segment_without_events() {
    let path = temporary("empty").with_extension("bin");
    drop(latch_audit::FileAudit::create(&path).unwrap());
    let output = invoke(&[&"audit".into(), &path.as_os_str().into()]).unwrap();
    assert_eq!(output.lines().count(), 1, "only the chain head: {output:?}");
    std::fs::remove_file(path).unwrap();
}
