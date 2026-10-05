use super::command_test_support::*;
use super::*;

#[test]
fn the_same_program_classifies_the_same_however_it_is_spelled() {
    for spelling in [
        "sudo",
        "/usr/bin/sudo",
        "SUDO",
        "C:\\Windows\\System32\\sudo.exe",
        "./sudo",
    ] {
        assert_eq!(command(spelling, &[]).program(), "sudo", "{spelling}");
    }
    assert_eq!(
        CommandLine::new("", vec![]),
        Err(CommandError::EmptyProgram)
    );
    assert_eq!(
        CommandLine::new("git", vec!["a\0b".into()]),
        Err(CommandError::IllegalCharacter)
    );
    assert_eq!(
        CommandLine::new("git", vec!["x".into(); MAXIMUM_ARGUMENTS + 1]),
        Err(CommandError::TooManyArguments)
    );
}
