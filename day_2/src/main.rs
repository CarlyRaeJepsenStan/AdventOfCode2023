use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(a_line: String, red: Regex, blue: Regex, green: Regex) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    let red_matches: Vec<_> = red.find_iter(&a_line).map(|x| x.as_str()).collect();
    let green_matches: Vec<_> = green.find_iter(&a_line).map(|x| x.as_str()).collect();
    let blue_matches: Vec<_> = blue.find_iter(&a_line).map(|x| x.as_str()).collect();

    return values;
}

fn main() {
    let redRegex = Regex::new(r"\d red");
    let greenRegex = Regex::new(r"\d green");
    let blueRegex = Regex::new(r"\d blue");

    let mut values: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(a_line) = line {}
        }
    }
}
