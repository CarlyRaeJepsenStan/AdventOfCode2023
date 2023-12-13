use pcre2::bytes::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn turn_string_to_int(input: String, dict: HashMap<String, String>) -> String {
    match dict.get(&input) {
        Some(output) => return output.to_string(),
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
fn parse_lines(a_line: String, dict: HashMap<String, String>, re: Regex) -> String {
    println!("{}", a_line);
    let matches: Vec<String> = re
        .captures_iter(a_line.as_bytes())
        .map(|x| {
            turn_string_to_int(
                String::from_utf8(x.unwrap().get(1).unwrap().as_bytes().to_vec()).unwrap(),
                dict.clone(),
            )
        })
        .collect();
    let l: usize = matches.len();
    if l > 1 {
        println!("{}", matches[0].to_owned() + &matches[l - 1]);
        return matches[0].to_owned() + &matches[l - 1];
    } else {
        println!("{}", matches[0].to_owned() + &matches[0]);
        return matches[0].to_owned() + &matches[0];
    }
}

fn main() {
    let dict: HashMap<String, String> = HashMap::from([
        ("one".to_string(), "1".to_string()),
        ("two".to_string(), "2".to_string()),
        ("three".to_string(), "3".to_string()),
        ("four".to_string(), "4".to_string()),
        ("five".to_string(), "5".to_string()),
        ("six".to_string(), "6".to_string()),
        ("seven".to_string(), "7".to_string()),
        ("eight".to_string(), "8".to_string()),
        ("nine".to_string(), "9".to_string()),
    ]);
    let re: Regex =
        Regex::new(r"(?=(\d{1}|one|two|three|four|five|six|seven|eight|nine|ten))").unwrap();
    let mut values: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(a_line) = line {
                values.push(
                    parse_lines(a_line, dict.clone(), re.clone())
                        .parse()
                        .unwrap(),
                );
            }
        }
    }
    let sum: i32 = values.into_iter().sum();
    println!("{}", sum);
}
