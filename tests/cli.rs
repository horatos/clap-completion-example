use std::path::PathBuf;

use assert_cmd::Command;
use rstest::{fixture, rstest};

#[fixture]
fn cmd() -> Command {
    Command::cargo_bin("clap-completion-example").unwrap()
}

#[fixture]
fn hello_world_txt() -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", "data", "hello_world.txt"].iter().collect()
}

#[fixture]
fn see_you_txt() -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "tests", "data", "see_you.txt"].iter().collect()
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

#[rstest]
fn with_option_file_see_you(mut cmd: Command, see_you_txt: PathBuf) {
    cmd.arg("-f").arg(&see_you_txt).assert().stdout("See you!\n");
}
