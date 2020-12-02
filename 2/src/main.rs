use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let re = Regex::new(r"(\d+)-(\d+)\s([a-zA-Z]):\s([a-zA-Z]+)").unwrap();
    let mut valid = 0;
    for rule in &input {
        let cap = re.captures(&rule).unwrap();

        let lower_bound: usize = cap[1].parse().unwrap();
        let upper_bound: usize = cap[2].parse().unwrap();
        let char = &cap[3];
        let string = &cap[4];

        let contains = string.matches(char).count();

        if (lower_bound <= contains && contains <= upper_bound) {
            valid += 1;
        }
    }
    println!("Part 1: {}", valid);


    valid = 0;
    for rule in &input {
        let cap = re.captures(&rule).unwrap();

        let lower_bound = cap[1].parse::<usize>().unwrap() - 1;
        let upper_bound = cap[2].parse::<usize>().unwrap() - 1;
        let char = &cap[3];
        let string = &cap[4];

        if (string.chars().nth(lower_bound).unwrap() != string.chars().nth(upper_bound).unwrap()) && (string.chars().nth(lower_bound).unwrap().to_string() == char || string.chars().nth(upper_bound).unwrap().to_string() == char) {
            valid += 1;
        }
    }
    println!("Part 2: {}", valid);

    Ok(())
}
