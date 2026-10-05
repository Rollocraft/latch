use super::*;

pub(super) fn command(program: &str, arguments: &[&str]) -> CommandLine {
    CommandLine::new(program, arguments.iter().map(|a| (*a).into()).collect()).unwrap()
}
