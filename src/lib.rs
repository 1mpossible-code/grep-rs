use clap::Parser;
use colored::{ColoredString, Colorize};
use regex::Regex;
use std::{
    io::{self, BufRead},
    path::PathBuf,
    vec,
};

pub type Match = Vec<ColoredString>;

#[derive(Parser, Debug)]
#[command(name = "grep-rs")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    expression: bool,

    #[clap(name = "PATTERN")]
    pattern: String,

    file: Option<Vec<PathBuf>>,
}

pub struct Grep {
    args: Args,
    files: Vec<PathBuf>,
}

impl Grep {
    pub fn new(args: Args) -> Self {
        let files = match &args.file {
            Some(files) => files.clone(),
            None => vec![],
        };

        Self { args, files }
    }

    pub fn run(&self) -> io::Result<()> {
        let is_multiple_files = self.files.len() != 1;

        for file_path in &self.files {
            let file = std::fs::File::open(file_path)?;
            let reader = std::io::BufReader::new(file);

            let result = if self.args.expression {
                find_regex_matches(&self.args.pattern, reader)
            } else {
                find_exact_matches(&self.args.pattern, reader)
            };

            match result {
                Ok(matches) => {
                    for m in matches {
                        if is_multiple_files {
                            print!("{}:", file_path.display());
                        }
                        print_match(&m);
                    }
                }
                Err(e) => panic!("{}", e),
            };
        }

        Ok(())
    }
}

// finds non regex matches
pub fn find_exact_matches<R: BufRead>(pattern: &str, buf_reader: R) -> io::Result<Vec<Match>> {
    let mut result: Vec<Match> = vec![];

    for (_, line) in buf_reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            let mut colored_line: Match = vec![];
            let colored_match = pattern.red().bold();
            let temp_split = line.split(pattern);
            for phrase in temp_split {
                colored_line.push(phrase.into());
                colored_line.push(colored_match.clone());
            }
            colored_line.pop();
            result.push(colored_line)
        }
    }

    Ok(result)
}

pub fn find_regex_matches<R: BufRead>(pattern: &str, buf_reader: R) -> io::Result<Vec<Match>> {
    let re = Regex::new(pattern).unwrap();
    let mut result: Vec<Match> = vec![];

    for (_, line) in buf_reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line) {
            let mut matches: Vec<_> = re.find_iter(&line).map(|m| m.as_str()).collect();
            matches.dedup();

            for m in matches {
                let mut colored_line: Match = vec![];
                let colored_match = m.red().bold();
                let temp_split = line.split(m);
                for phrase in temp_split {
                    colored_line.push(phrase.into());
                    colored_line.push(colored_match.clone());
                }
                colored_line.pop();
                result.push(colored_line)
            }
        }
    }

    Ok(result)
}

pub fn print_match(v: &Match) {
    for val in v {
        print!("{}", val);
    }
    println!();
}
