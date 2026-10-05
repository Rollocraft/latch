use crate::{command_test_support::invoke, run};
use std::ffi::OsString;

#[test]
fn version_flags_report_package_version() {
    for flag in ["--version", "-V"] {
        assert_eq!(
            invoke(&[&flag.into()]).unwrap(),
            format!("latch {}\n", env!("CARGO_PKG_VERSION"))
        );
        assert!(run(vec![flag.into(), "extra".into()], &mut Vec::new()).is_err());
    }
}

#[test]
fn help_and_invalid_commands() {
    let mut output = Vec::new();
    run(vec!["--help".into()], &mut output).unwrap();
    assert!(String::from_utf8(output).unwrap().contains("latch audit"));
    for args in [
        vec!["unknown".into()],
        vec!["audit".into()],
        vec![OsString::from("tx")],
        vec!["tx".into(), "begin".into()],
        vec!["tx".into(), "changes".into(), "a".into(), "b".into()],
    ] {
        assert!(run(args.clone(), &mut Vec::new()).is_err(), "{args:?}");
    }
}
