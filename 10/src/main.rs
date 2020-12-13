use std::io::{BufReader, BufRead};
use std::{io, fmt};
use std::fs::File;
use std::collections::{HashMap, HashSet};


fn solve(input: &Vec<usize>, current_index: usize, memo: &mut HashMap<usize, usize>) -> usize {
    let mut sum: usize = 0;
    for i in 1..=3 {
        if current_index + i < input.len() && (input.get(current_index + i).unwrap() - input.get(current_index).unwrap()) <= 3 {
            if memo.contains_key(&(current_index + i)) {
                sum += memo.get(&(current_index + i)).unwrap();
            } else {
                let result = solve(&input, current_index + i, memo);
                sum += result;
                memo.insert(current_index+i, result);
            }
        }
        if current_index + i == input.len() - 2 {
            sum += 1;
        }
    }
    sum
}


fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut input: Vec<usize> = reader.lines().map(|x| x.unwrap().parse().unwrap()).collect();

    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort();


    let mut diffs: HashMap<usize, usize> = HashMap::new();

    for (idx, value) in input.iter().enumerate().skip(1) {
        let diff = value - input.get(idx - 1).unwrap();
        *diffs.entry(diff).or_insert(0) += 1;
    }

    println!("Part 1: {}", diffs[&1] * diffs[&3]);

    let vec = vec![0, 1, 2, 3, 4];

    let mut memo: HashMap<usize, usize> = HashMap::new();

    println!("Part 2: {:?}", solve(&input, 0, &mut memo));

    Ok(())
}
