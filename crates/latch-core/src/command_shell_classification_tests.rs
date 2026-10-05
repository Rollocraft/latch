use super::command_test_support::*;
use super::*;

#[test]
fn anything_that_opens_a_shell_is_treated_as_what_it_is() {
    for (program, arguments) in [
        ("bash", vec!["-c", "git push --force"]),
        ("sh", vec!["-c", "rm -rf /"]),
        ("/bin/zsh", vec![]),
        ("cmd.exe", vec!["/c", "del /s"]),
        ("powershell", vec!["-Command", "Remove-Item -Recurse"]),
    ] {
        let classification = command(program, &arguments).classify();
        assert_eq!(classification.action, "command.shell", "{program}");
        assert_eq!(classification.risk.value(), 90);
    }
}

#[test]
fn an_unknown_program_is_assumed_to_be_irreversible() {
    let classification = command("some-internal-tool", &["--wipe"]).classify();
    assert_eq!(classification.action, "command.run");
    assert_eq!(classification.reversibility, Reversibility::Irreversible);
}
