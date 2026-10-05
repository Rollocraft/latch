use crate::{ArgumentMatcher, MAXIMUM_DEPTH};

fn arguments(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).into()).collect()
}

#[test]
fn a_rule_can_tell_a_push_from_a_force_push() {
    let push = arguments(&["push", "origin", "main"]);
    let force = arguments(&["push", "--force", "origin", "main"]);
    let short = arguments(&["push", "-f"]);

    let forced = ArgumentMatcher::AnyFlag(arguments(&["-f", "--force", "--force-with-lease"]));
    assert!(!forced.matches(&push));
    assert!(forced.matches(&force));
    assert!(forced.matches(&short));

    let a_push = ArgumentMatcher::Positional {
        index: 0,
        value: "push".into(),
    };
    assert!(a_push.matches(&push) && a_push.matches(&force));

    // The combination a real rule would carry: a forced push, specifically.
    let forced_push = ArgumentMatcher::All(vec![a_push, forced]);
    assert!(forced_push.matches(&force));
    assert!(!forced_push.matches(&arguments(&["pull", "--force"])));
}

#[test]
fn exactly_permits_one_invocation_and_nothing_added_to_it() {
    let matcher = ArgumentMatcher::Exactly(arguments(&["push", "origin", "main"]));
    assert!(matcher.matches(&arguments(&["push", "origin", "main"])));
    for other in [
        vec!["push", "origin", "main", "--force"],
        vec!["push", "origin"],
        vec!["origin", "push", "main"],
        vec![],
    ] {
        assert!(!matcher.matches(&arguments(&other)), "{other:?}");
    }
    assert!(ArgumentMatcher::Empty.matches(&[]));
    assert!(!ArgumentMatcher::Empty.matches(&arguments(&["x"])));
    assert!(ArgumentMatcher::Any.matches(&[]));
}

#[test]
fn matchers_that_cannot_mean_anything_are_refused() {
    assert!(ArgumentMatcher::Flag("--force".into()).valid());
    assert!(ArgumentMatcher::Flag("-f".into()).valid());
    for invalid in [
        ArgumentMatcher::Flag("force".into()),
        ArgumentMatcher::Flag("-".into()),
        ArgumentMatcher::Flag("".into()),
        ArgumentMatcher::AnyFlag(vec![]),
        ArgumentMatcher::AnyFlag(vec!["force".into()]),
        ArgumentMatcher::Contains("".into()),
        ArgumentMatcher::Contains("bad\nvalue".into()),
        ArgumentMatcher::All(vec![]),
        ArgumentMatcher::Exactly(vec!["\u{1b}[2J".into()]),
    ] {
        assert!(!invalid.valid(), "{invalid:?} was accepted");
    }
}

#[test]
fn nesting_is_bounded_before_anything_is_evaluated() {
    let mut matcher = ArgumentMatcher::Flag("--force".into());
    for _ in 0..MAXIMUM_DEPTH {
        matcher = ArgumentMatcher::All(vec![matcher]);
    }
    assert!(matcher.valid());
    matcher = ArgumentMatcher::All(vec![matcher]);
    assert!(!matcher.valid(), "depth past the bound must be refused");
}
