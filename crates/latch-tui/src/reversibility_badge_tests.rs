use crate::presentation_test_fixtures::*;
use crate::*;
use latch_core::Reversibility;

#[test]
fn all_reversibility_badges_are_distinct_and_honest() {
    for (value, expected) in [
        (Reversibility::FullyReversible, "[REVERSIBLE]"),
        (Reversibility::PartiallyReversible, "[PARTIALLY REVERSIBLE]"),
        (Reversibility::Irreversible, "[IRREVERSIBLE]"),
    ] {
        let mut action = action();
        action.reversibility = value;
        assert!(
            render_approval(&session(), &action, &decision())
                .rendered()
                .text
                .contains(expected)
        );
    }
}
