use assert_cmd::Command;
use rstest::{fixture, rstest};

#[fixture]
fn command_bin() -> Command {
    Command::cargo_bin("clap-completion-example").unwrap()
}

#[rstest]
fn exec_without_arguments(command_bin: Command) {
    let mut cmd = command_bin;
    cmd.assert().stdout("Hello\n");
}
