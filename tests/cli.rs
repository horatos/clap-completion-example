use assert_cmd::Command;
use rstest::{fixture, rstest};

#[fixture]
fn cmd() -> Command {
    Command::cargo_bin("clap-completion-example").unwrap()
}

#[rstest]
fn without_arguments(mut cmd: Command) {
    cmd.assert().stdout("Hello\n");
}

#[rstest]
fn with_option_lang_en(mut cmd: Command) {
    cmd.args(["-l", "en"]).assert().stdout("Hello\n");
}

#[rstest]
fn with_option_lang_ja(mut cmd: Command) {
    cmd.args(["-l", "ja"]).assert().stdout("こんにちは\n");
}
