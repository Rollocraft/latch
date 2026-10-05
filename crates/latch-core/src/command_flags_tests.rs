use super::*;

#[test]
fn flags_are_recognized_in_the_forms_a_command_line_actually_uses() {
    let flags = |args: &[&str]| -> Vec<String> { args.iter().map(|a| (*a).into()).collect() };

    assert!(has_flag(&flags(&["--force"]), "--force"));
    assert!(has_flag(&flags(&["--force=true"]), "--force"));
    assert!(has_flag(&flags(&["-f"]), "-f"));
    assert!(has_flag(&flags(&["-rf"]), "-f"));
    assert!(has_flag(&flags(&["-rf"]), "-r"));

    assert!(!has_flag(&flags(&["--forced"]), "--force"));
    assert!(!has_flag(&flags(&["--no-force"]), "--force"));
    assert!(!has_flag(&flags(&["-o=force"]), "-f"));
    assert!(!has_flag(&flags(&["force"]), "-f"));
    // Nothing after a bare -- is an option.
    assert!(!has_flag(&flags(&["--", "-f"]), "-f"));
    assert!(!has_flag(&flags(&["--", "--force"]), "--force"));
    // A flag must be written as a flag.
    assert!(!has_flag(&flags(&["-f"]), "f"));
}
