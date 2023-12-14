use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn main() {
    let special_regex = Regex::new(r"\@|\#|\$|\%|\^|\&|\*|\(|\)|\_|\+|\=").unwrap();
    let lines: Vec<String> = lines_from_file("test.txt");
    for line in lines {
        println!(
            "{:?}",
            special_regex
                .captures_iter(&line)
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
        );
    }
}
