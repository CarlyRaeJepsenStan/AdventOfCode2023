use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn m(input: &String, nums: HashMap<String, String>) -> String {
    match nums.get(input) {
        Some(m) => return m.to_string(),
        None => return input.to_string(),
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(a_line: String, nums: HashMap<String, String>, re: Regex) -> String {
    let matches: Vec<_> = re.find_iter(&a_line).map(|x| x.as_str()).collect();
    let l: usize = matches.len();
    if l > 1 {
        return matches[0].to_owned() + matches[l - 1];
    } else {
        return matches[0].to_owned() + matches[0];
    }
}

fn main() {
    let nums = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let re: Regex =
        Regex::new(r"\d{1}|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(ten)")
            .unwrap();
    if let Ok(lines) = read_lines("test.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(a_line) = line {
                let matches: Vec<_> = re.find_iter(&a_line).map(|x| x.as_str()).collect();
                println!("{:?}", matches);
            }
        }
    }
}
