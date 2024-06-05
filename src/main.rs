use std::{
    fs::File,
    io::{stdin, BufReader, Read},
    path::PathBuf,
};

use clap::Parser;
use grep_rs::{find_exact_matches, find_regex_matches, print_all_matches, Match};

#[derive(Parser, Debug)]
#[command(name = "grep-rs")]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    expression: bool,

    #[clap(name = "PATTERN")]
    pattern: String,

    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let result: Result<Vec<Match>, std::io::Error>;

    let input: Box<dyn Read> = match args.file {
        Some(file_path) => Box::new(File::open(file_path).expect("Failed to open file")),
        None => Box::new(stdin().lock()),
    };

    let reader = BufReader::new(input);

    if args.expression {
        result = find_regex_matches(&args.pattern, reader);
    } else {
        result = find_exact_matches(&args.pattern, reader);
    }

    match result {
        Ok(m) => print_all_matches(&m),
        Err(e) => panic!("{}", e),
    };
}
