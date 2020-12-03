use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn slope_solver(input: &Vec<String>, right: usize, down: usize) -> u32 {
    let mut col = right;
    let mut row = down;

    let mut trees = 0;
    while row < input.len() {
        if input[row].as_bytes()[col % (input[row].len())] == b'#'
        {
            trees += 1;
        }


        col += right;
        row += down;
    }

    return trees;
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let trees = slope_solver(&input, 3, 1);
    println!("Part 1: {}", trees);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees = slopes.iter().map(|e| { slope_solver(&input, e.0, e.1) }).fold(1, |a, v| a * v);
    println!("Part 2: {}", trees);

    Ok(())
}
