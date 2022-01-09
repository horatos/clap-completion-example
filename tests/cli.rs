use std::path::PathBuf;

use assert_cmd::Command;
use rstest::{fixture, rstest};

#[fixture]
fn cmd() -> Command {
    Command::cargo_bin("clap-completion-example").unwrap()
}

#[fixture]
fn greet(mut cmd: Command) -> Command {
    cmd.arg("greet");
    cmd
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
fn greet_without_arguments(mut greet: Command) {
    greet.assert().stdout("Hello\n");
}

#[rstest]
fn greet_with_option_lang_en(mut greet: Command) {
    greet.args(["-l", "en"]).assert().stdout("Hello\n");
}

#[rstest]
fn greet_with_option_lang_ja(mut greet: Command) {
    greet.args(["-l", "ja"]).assert().stdout("こんにちは\n");
}

#[rstest]
fn greet_with_option_file_see_you(mut greet: Command, see_you_txt: PathBuf) {
    greet.arg("-f").arg(&see_you_txt).assert().stdout("See you!\n");
}

#[rstest]
fn greet_with_language_and_file(mut greet: Command) {
    greet.args(["-l", "ja", "-f", "foo.txt"]).assert().failure();
}
