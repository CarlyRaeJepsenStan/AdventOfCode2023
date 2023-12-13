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

fn parse_lines(a_line: String) -> String {
    let re = Regex::new(r"\d{1}").unwrap();
    let matches: Vec<_> = re.find_iter(&a_line).map(|x| x.as_str()).collect();
    let l: usize = matches.len();
    if l > 1 {
        return matches[0].to_owned() + matches[l - 1];
    } else {
        return matches[0].to_owned() + matches[0];
    }
}

fn main() {
    let mut values: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(a_line) = line {
                values.push(parse_lines(a_line).parse().unwrap());
            }
        }
    }
    let sum: i32 = values.into_iter().sum();
    println!("{}", sum);
}
