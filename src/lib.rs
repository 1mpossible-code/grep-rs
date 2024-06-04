use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

use regex::Regex;

use colored::{ColoredString, Colorize};

pub fn search_in_file(pattern: &str, file_path: &str) -> io::Result<Vec<Vec<ColoredString>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let re = Regex::new(pattern).unwrap();
    let mut result: Vec<Vec<ColoredString>> = vec![];

    for (_, line) in reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line) {
            let mut matches: Vec<_> = re.find_iter(&line).map(|m| m.as_str()).collect();
            matches.dedup();

            for m in matches {
                let mut colored_line: Vec<ColoredString> = vec![];
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

pub fn print_match(v: &Vec<ColoredString>) {
    for val in v {
        print!("{}", val);
    }
    println!();
}

pub fn print_all_matches(matches: &Vec<Vec<ColoredString>>) {
    for m in matches {
        print_match(&m);
    }
}