use std::io::{BufReader, BufRead};
use std::{io, fmt};
use std::fs::File;
use std::collections::{HashMap, HashSet};

const PREAMBLE_LENGTH: usize = 25;

use itertools::iproduct;

fn calculate_sums(input: &[usize]) -> HashSet<usize> {
    iproduct!(input.iter(), input.iter()).filter(|x| x.0 != x.1)
        .map(|t| t.0 + t.1)
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<usize> = reader.lines().map(|x| x.unwrap().parse().unwrap()).collect();

    let mut result1 : usize = 0;

    for (idx, number) in input.iter().skip(PREAMBLE_LENGTH).enumerate() {
        let previous = &input[idx..idx + PREAMBLE_LENGTH];
        let sums = calculate_sums(previous);
        if !sums.contains(number) {
            result1 = *number;
            println!("Part 1: {}", number);
            break;
        }
    }

    for (start_idx, start_value) in input.iter().enumerate()  {
        for (end_idx, end_value) in input.iter().skip(start_idx+2).enumerate() {
            let range = &input[start_idx..start_idx+2+end_idx];
            if range.iter().sum::<usize>() == result1 {
                println!("Part 2: {}", range.iter().max().unwrap() + range.iter().min().unwrap());
                break;
            }
        }
    }

    Ok(())
}
