use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn search_in_file(pattern: &str, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}: {}", index + 1, line)
        }
    }

    Ok(())
}
