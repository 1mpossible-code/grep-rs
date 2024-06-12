use clap::Parser;
use grep_rs::Grep;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "grep-rs", version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    pub expression: bool,

    #[clap(short, long)]
    pub ignore_case: bool,

    #[clap(short = 'n', long = "line-number")]
    pub line_number: bool,

    #[clap(name = "PATTERN")]
    pub pattern: String,

    pub file: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let grep = Grep::new(
        args.file,
        args.pattern,
        args.expression,
        args.ignore_case,
        args.line_number,
    );

    match grep.run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
