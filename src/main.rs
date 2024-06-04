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

    file: String,
}

fn main() {
    let args = Args::parse();

    let result: Result<Vec<Match>, std::io::Error>;

    if args.expression {
        println!("expression mode");
        print!("pattern: {}, file: {}", args.pattern, args.file);
        result = find_regex_matches(&args.pattern, &args.file);
    } else {
        result = find_exact_matches(&args.pattern, &args.file);
    }


    match result {
        Ok(m) => print_all_matches(&m),
        Err(e) => panic!("{}", e),
    };
}