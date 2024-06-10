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

    #[clap(short, long)]
    pub ignore_case: bool,

    #[clap(name = "PATTERN")]
    pub pattern: String,

    pub file: Vec<PathBuf>,
}

pub struct Grep {
    files: Vec<PathBuf>,
    pattern: String,
    is_expression: bool,
    ignore_case: bool,
}

impl Grep {
    pub fn new(args: Args) -> Self {
        Self {
            files: args.file,
            pattern: args.pattern,
            is_expression: args.expression,
            ignore_case: args.ignore_case,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        if self.files.is_empty() {
            let matches = self.process_stdin()?;
            display_matches(false, matches, &PathBuf::from("stdin"));
        }
        for file_path in &self.files {
            let matches = self.process_file(file_path)?;
            display_matches(self.files.len() > 1,matches, file_path);
        }
        Ok(())
    }

    fn process_stdin(&self) -> io::Result<Vec<Match>> {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let matches = if self.is_expression {
            self.find_regex_matches(reader)?
        } else {
            self.find_exact_matches(reader)?
        };
        Ok(matches)
    }

    fn process_file(&self, file_path: &PathBuf) -> io::Result<Vec<Match>> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        if self.is_expression {
            self.find_regex_matches(reader)
        } else {
            self.find_exact_matches(reader)
        }
    }

    fn find_exact_matches<R: BufRead>(&self, buf_reader: R) -> io::Result<Vec<Match>> {
        let pattern = if self.ignore_case {
            self.pattern.to_lowercase()
        } else {
            self.pattern.clone()
        };
        let mut results = Vec::new();
        for line in buf_reader.lines() {
            let line = line?;
            let line_to_check = if self.ignore_case {
                line.to_lowercase()
            } else {
                line.as_str().to_string()
            };
            if line_to_check.contains(pattern.as_str()) {
                // TODO: Fix the printing as lower case
                results.push(colorize_match(&line_to_check, self.pattern.as_str()));
            }
        }
        Ok(results)
    }

    fn find_regex_matches<R: BufRead>(&self, buf_reader: R) -> io::Result<Vec<Match>> {
        let re = Regex::new(self.pattern.as_str()).unwrap();
        let mut results = Vec::new();
        for line in buf_reader.lines() {
            let line = line?;
            for mat in re.find_iter(&line) {
                results.push(colorize_match(&line, mat.as_str()));
            }
        }
        Ok(results)
    }

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

fn display_matches(is_multiple_files: bool, matches: Vec<Match>, file_path: &PathBuf) {
    for match_line in matches {
        if is_multiple_files {
            print!("{}:", file_path.display());
        }
        print_match(&match_line);
    }
}