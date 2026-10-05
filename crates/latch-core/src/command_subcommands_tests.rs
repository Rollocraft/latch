use super::command_test_support::*;

#[test]
fn a_leading_option_cannot_hide_the_dangerous_verb() {
    assert_eq!(
        command("git", &["-C", "/repo", "push"]).subcommand(),
        Some("push"),
        "the value of -C is not the subcommand"
    );
    assert_eq!(
        command("git", &["-C", "/repo", "push", "--force"])
            .classify()
            .action,
        "git.force_push"
    );
    assert_eq!(
        command("kubectl", &["-n", "prod", "delete", "pod", "api"])
            .classify()
            .action,
        "kubernetes.delete"
    );
    assert_eq!(
        command("git", &["--git-dir=/repo/.git", "status"]).subcommand(),
        Some("status"),
        "an option carrying its own value consumes nothing after it"
    );
    assert_eq!(
        command("git", &["--bare", "status"]).subcommand(),
        Some("status")
    );
    assert_eq!(command("git", &[]).subcommand(), None);
    // A subcommand argument that happens to repeat a verb is not a verb.
    assert_eq!(
        command("git", &["commit", "-m", "push"]).classify().action,
        "git.commit"
    );
}
