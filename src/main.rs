use clap::Parser;
use grep_rs::{print_all_matches, search_in_file};

#[derive(Parser, Debug)]
#[command(name = "grep-rs")]
#[command(version, about, long_about = None)]
struct Args {
    pattern: String,

    file: String,
}

fn main() {
    let args = Args::parse();

    let result = search_in_file(&args.pattern, &args.file);

    match result {
        Ok(m) => print_all_matches(&m),
        Err(e) => panic!("{}", e),
    };
}