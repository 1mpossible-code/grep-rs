use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "grep-rs")]
#[command(version, about, long_about = None)]
struct Args {
    pattern: String,

    file: String,
}

fn main() {
    let args = Args::parse();
}