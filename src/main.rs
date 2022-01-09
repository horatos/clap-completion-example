use std::path::PathBuf;

use clap::{ArgEnum, Parser, Subcommand};

/// Greet command (example for clap_complete command).
#[derive(Parser,Debug)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand,Debug)]
enum Action {
    /// Greet some message.
    Greet {
        /// Language in which messages are shown.
        #[clap(long,short,arg_enum)]
        language: Option<Language>,
        /// File whose content is printed.
        ///
        /// The trailing whitespaces of the content are trimmed.
        #[clap(long,short,conflicts_with("language"))]
        file: Option<PathBuf>,
    },
}

#[derive(ArgEnum,Clone,Debug)]
enum Language {
    En,
    Ja,
}

impl Action {
    fn handle(self) {
        use Action::Greet;

        match self {
            Greet { language: None, file: None } => {
                println!("Hello");
            },
            Greet { language: Some(Language::En), .. } => {
                println!("Hello");
            },
            Greet { language: Some(Language::Ja), .. } => {
                println!("こんにちは");
            },
            Greet { file: Some(file), .. } => {
                let s = std::fs::read_to_string(&file).unwrap();
                println!("{}", s.trim_end());
            },
        }
    }
}

fn main() {
    Cli::parse().action.handle();
}
