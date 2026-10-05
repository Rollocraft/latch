use crate::bounded_output::Output;
use crate::presentation_test_fixtures::*;
use crate::terminal_text::CLIPPED;
use crate::*;
use latch_policy::{Effect, PolicyLevel, RuleMatch};

#[test]
fn policy_lists_and_total_output_are_bounded() {
    let mut decision = decision();
    decision.matches = vec![
        RuleMatch {
            policy: "界".repeat(1000),
            level: PolicyLevel::Company,
            rule: "r".repeat(1000),
            description: "d".repeat(1000),
            effect: Effect::Ask,
        };
        10_000
    ];
    let view = render_approval(&session(), &action(), &decision);
    assert!(view.rendered().truncated);
    assert!(
        view.rendered()
            .text
            .contains("9984 additional entries omitted")
    );
    assert!(!view.allow_once_available());
    assert_safe(&view.rendered().text);
    let mut out = Output::default();
    for _ in 0..1000 {
        out.field("Field", &"x".repeat(1000));
    }
    let rendered = out.finish();
    assert!(rendered.truncated);
    assert!(rendered.text.ends_with(&format!("{CLIPPED}\n")));
    assert_safe(&rendered.text);
}
