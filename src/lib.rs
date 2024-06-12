use colored::{ColoredString, Colorize};
use regex::Regex;
use std::io::{self, BufRead};
use std::path::PathBuf;

struct Match {
    string_vec: Vec<ColoredString>,
    file_path: String,
    line_number: usize,
}

impl Match {
    fn new(string_vec: Vec<ColoredString>, file_path: &str, line_number: usize) -> Self {
        Self {
            string_vec,
            file_path: file_path.to_string(),
            line_number,
        }
    }

    fn display(&self) {
        for part in &self.string_vec {
            print!("{}", part);
        }
        println!();
    }

    fn add_file_name(&mut self) {
        self.string_vec
            .insert(0, ColoredString::from(format!("{}:", self.file_path)));
    }

    fn add_line_number(&mut self) {
        self.string_vec
            .insert(0, ColoredString::from(format!("{}:", self.line_number)));
    }
}

pub struct Grep {
    files: Vec<PathBuf>,
    pattern: String,
    is_expression: bool,
    ignore_case: bool,
    is_line_number: bool,
}

impl Grep {
    pub fn new(
        files: Vec<PathBuf>,
        pattern: String,
        is_expression: bool,
        ignore_case: bool,
        is_line_number: bool,
    ) -> Self {
        Self {
            files,
            pattern,
            is_expression,
            ignore_case,
            is_line_number,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        if self.files.is_empty() {
            let matches = self.process_stdin()?;
            self.display(matches)
        }
        for file_path in &self.files {
            let matches = self.process_file(file_path)?;
            self.display(matches);
        }
        Ok(())
    }

    fn process_stdin(&self) -> io::Result<Vec<Match>> {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let matches = if self.is_expression {
            self.find_regex_matches(reader, "")?
        } else {
            self.find_exact_matches(reader, "")?
        };
        Ok(matches)
    }

    fn process_file(&self, file_path: &PathBuf) -> io::Result<Vec<Match>> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        if self.is_expression {
            self.find_regex_matches(reader, file_path.to_str().unwrap())
        } else {
            self.find_exact_matches(reader, file_path.to_str().unwrap())
        }
    }

    fn find_exact_matches<R: BufRead>(
        &self,
        buf_reader: R,
        file_name: &str,
    ) -> io::Result<Vec<Match>> {
        let pattern = if self.ignore_case {
            self.pattern.to_lowercase()
        } else {
            self.pattern.clone()
        };
        let mut results = Vec::new();
        let mut line_number = 0;
        for line in buf_reader.lines() {
            let line = line?;
            let line_to_check = if self.ignore_case {
                line.to_lowercase()
            } else {
                line.as_str().to_string()
            };
            if line_to_check.contains(pattern.as_str()) {
                results.push(Match::new(
                    colorize_match(&line, &line_to_check, self.pattern.as_str()),
                    file_name,
                    line_number,
                ));
            }
            line_number += 1;
        }
        Ok(results)
    }

    fn find_regex_matches<R: BufRead>(
        &self,
        buf_reader: R,
        file_name: &str,
    ) -> io::Result<Vec<Match>> {
        let re = Regex::new(self.pattern.as_str()).unwrap();
        let mut results = Vec::new();
        let mut counter: usize = 0;
        for line in buf_reader.lines() {
            let line = line?;
            for mat in re.find_iter(&line) {
                // results.push(colorize_match(&line, &line, mat.as_str()));
                results.push(Match::new(
                    colorize_match(&line, &line, mat.as_str()),
                    file_name,
                    counter,
                ));
            }
            counter += 1;
        }
        Ok(results)
    }

    fn display(&self, matches: Vec<Match>) {
        for mut match_ in matches {
            if self.is_line_number {
                match_.add_line_number();
            }
            if self.files.len() > 1 {
                match_.add_file_name();
            }
            match_.display();
        }
    }
}

fn colorize_match(original_text: &str, text_to_search: &str, pattern: &str) -> Vec<ColoredString> {
    let length = pattern.len();
    let mut result = Vec::new();
    let mut start_ind: usize = 0;
    let mut match_option = text_to_search[start_ind..].find(pattern);

    while match_option.is_some() {
        let match_ind = match_option.unwrap();
        let end_ind = start_ind + match_ind;
        result.push(ColoredString::from(
            original_text[start_ind..end_ind].to_string(),
        ));
        result.push(ColoredString::from(
            original_text[end_ind..end_ind + length]
                .to_string()
                .bold()
                .red(),
        ));
        start_ind = end_ind + length;
        match_option = text_to_search[start_ind..].find(pattern);
    }

    result.push(ColoredString::from(original_text[start_ind..].to_string()));

    result
}
