use clap::Parser;
use grep_rs::{Args, Grep};


fn main() {
    let args = Args::parse();
    let grep = Grep::new(args);

    match grep.run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
