use super::command_test_support::*;
use super::*;

#[test]
fn arguments_decide_what_a_command_means() {
    // The same program means different things depending on its arguments.
    let harmless = command("git", &["status"]).classify();
    assert_eq!(harmless.action, "git.inspect");
    assert_eq!(harmless.risk.value(), 2);

    for force in [
        vec!["push", "--force", "origin", "main"],
        vec!["push", "-f"],
        vec!["push", "--force-with-lease"],
    ] {
        let dangerous = command("git", &force).classify();
        assert_eq!(dangerous.action, "git.force_push", "{force:?}");
        assert_eq!(dangerous.reversibility, Reversibility::Irreversible);
    }
    let ordinary = command("git", &["push", "origin", "main"]).classify();
    assert_eq!(ordinary.action, "git.push");
    assert!(ordinary.risk < command("git", &["push", "-f"]).classify().risk);

    assert_eq!(
        command("rm", &["-rf", "/"]).classify().action,
        "file.recursive_delete"
    );
    assert_eq!(
        command("rm", &["file.txt"]).classify().action,
        "file.delete"
    );
    assert_eq!(
        command("terraform", &["destroy", "-auto-approve"])
            .classify()
            .action,
        "infrastructure.destroy"
    );
}
