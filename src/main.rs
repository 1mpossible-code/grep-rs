use std::{fs::File, io::{stdin, BufReader, IsTerminal, Read}, path::PathBuf};

use clap::{CommandFactory, Parser};
use grep_rs::{find_exact_matches, find_regex_matches, print_all_matches, Match};

#[derive(Parser, Debug)]
#[command(name = "grep-rs")]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    expression: bool,

    #[clap(name = "PATTERN")]
    pattern: String,

    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = args.file;
    let result: Result<Vec<Match>, std::io::Error>;

    let reader: Box<dyn Read> = if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Args::command().print_help().unwrap();
            std::process::exit(2);
        }

        Box::new(stdin().lock())
    } else {
        Box::new(File::open(&file).unwrap())
    };

    let reader = BufReader::new(reader);


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