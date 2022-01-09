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
    let _cli = Cli::parse();

    println!("Hello");
}
