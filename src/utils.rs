use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn readfile(path: &str) -> Vec<String> {
    let path: &Path = Path::new(path);
    let file: File = File::open(&path).expect("Could not open file");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}
