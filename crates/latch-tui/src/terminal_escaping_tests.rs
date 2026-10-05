use crate::presentation_test_fixtures::*;
use crate::terminal_text::CLIPPED;
use crate::*;
use latch_policy::Effect;

#[test]
fn terminal_controls_and_unicode_spoofing_are_escaped() {
    let attack = "\x1b[2J\x1b]52;c;data\x07\r\n\t\x08\x7f\u{009b}\u{009d}\u{202e}\u{2066}\u{2069}\u{2028}\u{2029}\u{200b}";
    let escaped = terminal_text(attack);
    assert_safe(&escaped.text);
    assert!(!escaped.text.contains('\n'));
    assert!(escaped.text.contains("\\u{1b}"));
    assert!(escaped.text.contains("\\n"));
    let mut action = action();
    action.resource = attack.into();
    action.id = attack.into();
    action.name = attack.into();
    let mut decision = decision();
    decision.matches[0].policy = attack.into();
    decision.matches[0].rule = attack.into();
    decision.matches[0].description = attack.into();
    decision.limits.insert(attack.into(), 2);
    decision.matches[0].effect = Effect::Limit {
        budget: attack.into(),
        maximum: 2,
    };
    assert_safe(
        &render_approval(&session(), &action, &decision)
            .rendered()
            .text,
    );
}

#[test]
fn utf8_clipping_is_bounded_and_disables_allow() {
    for unit in ["a", "é", "界", "\u{1f680}", "\x1b", "e\u{301}"] {
        let value = unit.repeat(20_000);
        let rendered = terminal_text(&value);
        assert!(rendered.truncated);
        assert!(rendered.text.len() <= MAX_FIELD_BYTES);
        assert!(rendered.text.ends_with(CLIPPED));
        let mut action = action();
        action.resource = value;
        let view = render_approval(&session(), &action, &decision());
        assert!(view.rendered().truncated);
        assert_eq!(
            view.parse_response("allow once"),
            Err(ResponseError::AllowUnavailable)
        );
        assert_safe(&view.rendered().text);
    }
    assert_eq!(terminal_text("café/文件").text, "café/文件");
    assert!(!terminal_text("").truncated);
}
