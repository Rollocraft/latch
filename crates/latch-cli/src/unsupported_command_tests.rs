use crate::scenarios;
use crate::workflow_test_fixtures::*;
use std::path::Path;

#[test]
fn unsupported_run_never_executes_and_capabilities_are_truthful() {
    let files = Files::new();
    let marker = files.0.join("must-not-exist").into_os_string();
    for args in [
        vec!["run".as_ref()],
        vec![
            "run".as_ref(),
            "--".as_ref(),
            "touch".as_ref(),
            marker.as_os_str(),
        ],
    ] {
        let (result, output) = call(&args);
        assert!(
            result
                .unwrap_err()
                .contains("no confined execution backend")
        );
        assert!(output.is_empty());
        assert!(!Path::new(&marker).exists());
    }
    let (result, output) = call(&["capabilities".as_ref()]);
    assert!(result.is_ok());
    assert!(output.contains("Unsupported: confined run"));
    assert!(output.contains("not authenticated identities"));
    let (_, help) = call(&["--help".as_ref()]);
    assert!(help.contains("latch tx begin <root> <workspace>"));
    assert!(help.contains("unsupported until a confined backend exists"));
    assert_eq!(
        call(&["policy".as_ref(), "test".as_ref(), "--help".as_ref()]).1,
        scenarios::HELP
    );
}

#[test]
fn workflow_argument_counts_are_strict() {
    for args in [
        vec!["policy"],
        vec!["policy", "validate"],
        vec!["policy", "validate", "a", "b"],
        vec!["policy", "test"],
        vec!["policy", "test", "a"],
        vec!["policy", "test", "a", "b", "c"],
        vec!["manifest"],
        vec!["manifest", "check"],
        vec!["manifest", "check", "a", "b"],
        vec!["manifest", "diff", "a"],
        vec!["manifest", "diff", "a", "b", "c"],
        vec!["diff"],
        vec!["diff", "a", "b"],
        vec!["commit"],
        vec!["rollback"],
        vec!["rollback", "a", "b"],
        vec!["tx", "begin", "a"],
        vec!["tx", "begin", "a", "b", "c"],
        vec!["capabilities", "extra"],
    ] {
        let (result, output) = call(
            &args
                .iter()
                .map(|arg| std::ffi::OsStr::new(*arg))
                .collect::<Vec<_>>(),
        );
        assert!(result.is_err(), "{args:?}");
        assert!(output.is_empty(), "{args:?}");
    }
}
