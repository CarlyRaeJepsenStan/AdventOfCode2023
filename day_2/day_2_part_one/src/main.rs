use fancy_regex::Regex;
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

fn parse_lines(a_line: String, red: Regex, blue: Regex, green: Regex) -> bool {
    let mut red_values: Vec<i32> = red
        .find_iter(&a_line)
        .map(|x| x.unwrap().as_str().to_owned().parse().unwrap())
        .collect();
    let mut green_values: Vec<i32> = green
        .find_iter(&a_line)
        .map(|x| x.unwrap().as_str().to_owned().parse().unwrap())
        .collect();
    let mut blue_values: Vec<i32> = blue
        .find_iter(&a_line)
        .map(|x| x.unwrap().as_str().to_owned().parse().unwrap())
        .collect();

    red_values.sort();
    green_values.sort();
    blue_values.sort();
    /*
        println!(
            "red: {:#?}\nblue: {:#?}\ngreen: {:#?}",
            red_values, blue_values, green_values
        );
    */
    let red_max: i32 = red_values.pop().unwrap();
    let blue_max: i32 = blue_values.pop().unwrap();
    let green_max: i32 = green_values.pop().unwrap();

    //hardcoded values

    if red_max > 12 || green_max > 13 || blue_max > 14 {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let red_regex = Regex::new(r"\d+(?= red)").unwrap();
    let green_regex = Regex::new(r"\d+(?= green)").unwrap();
    let blue_regex = Regex::new(r"\d+(?= blue)").unwrap();
    let mut i: i32 = 1;
    let mut result: i32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(a_line) = line {
                if parse_lines(
                    a_line,
                    red_regex.clone(),
                    blue_regex.clone(),
                    green_regex.clone(),
                ) {
                    result += i;
                }

                i += 1;
                /*
                let v: bool = parse_lines(
                    a_line,
                    red_regex.clone(),
                    blue_regex.clone(),
                    green_regex.clone(),
                );
                println!("{}: {}", i, v);
                i += 1;
                */
            }
        }
    }
    println!("result: {}", result);
}
