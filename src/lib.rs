use clap::Parser;
use colored::{ColoredString, Colorize};
use regex::Regex;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub type Match = Vec<ColoredString>;

#[derive(Parser, Debug)]
#[command(name = "grep-rs", version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub expression: bool,

    #[clap(name = "PATTERN")]
    pub pattern: String,

    pub file: Vec<PathBuf>,
}

pub struct Grep {
    files: Vec<PathBuf>,
    pattern: String,
    is_expression: bool,
}

impl Grep {
    pub fn new(args: Args) -> Self {
        Self {
            files: args.file,
            pattern: args.pattern,
            is_expression: args.expression,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        if self.files.is_empty() {
            return Ok(self.process_stdin()?);
        }
        for file_path in &self.files {
            let matches = self.process_file(file_path)?;
            self.display_matches(matches, file_path);
        }
        Ok(())
    }

    fn process_stdin(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let matches = if self.is_expression {
            find_regex_matches(&self.pattern, reader)?
        } else {
            find_exact_matches(&self.pattern, reader)?
        };
        self.display_matches(matches, &PathBuf::from("stdin"));
        Ok(())
    }

    fn process_file(&self, file_path: &PathBuf) -> io::Result<Vec<Match>> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        if self.is_expression {
            find_regex_matches(&self.pattern, reader)
        } else {
            find_exact_matches(&self.pattern, reader)
        }
    }

    fn display_matches(&self, matches: Vec<Match>, file_path: &PathBuf) {
        for match_line in matches {
            if self.files.len() > 1 {
                print!("{}:", file_path.display());
            }
            print_match(&match_line);
        }
    }
}

fn find_exact_matches<R: BufRead>(pattern: &str, buf_reader: R) -> io::Result<Vec<Match>> {
    let mut results = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            results.push(colorize_match(&line, pattern));
        }
    }
    Ok(results)
}

fn find_regex_matches<R: BufRead>(pattern: &str, buf_reader: R) -> io::Result<Vec<Match>> {
    let re = Regex::new(pattern).unwrap();
    let mut results = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        for mat in re.find_iter(&line) {
            results.push(colorize_match(&line, mat.as_str()));
        }
    }
    Ok(results)
}

fn colorize_match(text: &str, pattern: &str) -> Match {
    let colored_match = pattern.red().bold();
    let mut result = Vec::new();
    let parts = text.split(pattern);
    for part in parts {
        result.push(part.into());
        result.push(colored_match.clone());
    }
    result.pop(); // Remove the last redundant colored match
    result
}

fn print_match(match_line: &Match) {
    for part in match_line {
        print!("{}", part);
    }
    println!();
}
