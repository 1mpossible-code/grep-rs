use std::{
    io::{self, BufRead},
    vec,
};

use regex::Regex;

use colored::{ColoredString, Colorize};

pub type Match = Vec<ColoredString>;

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

pub fn print_all_matches(matches: &Vec<Match>) {
    for m in matches {
        print_match(&m);
    }
}
