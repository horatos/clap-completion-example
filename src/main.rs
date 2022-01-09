use std::path::PathBuf;

use clap::{ArgEnum, Parser};

#[derive(Parser,Debug)]
struct Cli {
    #[clap(long,short,arg_enum)]
    language: Option<Language>,
    #[clap(long,short)]
    file: Option<PathBuf>,
}

#[derive(ArgEnum,Clone,Debug)]
enum Language {
    En,
    Ja,
}

fn main() {
    match Cli::parse() {
        Cli { language: None, file: None } => {
            println!("Hello");
        },
        Cli { language: Some(Language::En), .. } => {
            println!("Hello");
        },
        Cli { language: Some(Language::Ja), .. } => {
            println!("こんにちは");
        },
        Cli { file: Some(file), .. } => {
            let s = std::fs::read_to_string(&file).unwrap();
            println!("{}", s.trim_end());
        },
    }
}
