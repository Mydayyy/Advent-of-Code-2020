use std::io::{BufReader, BufRead};
use std::io;
use std::fs::File;
use itertools::{Itertools, join};
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let input: Vec<String> = input.iter().group_by(|l| l.is_empty()).into_iter().map(|(_, g)| join(g, " ")).filter(|x| !x.is_empty()).collect();
    let part1: usize = input.iter().map(|x| x.split("").filter(|x| !x.is_empty() && x != &" ").unique().count()).sum();
    let part2: Vec<Vec<_>> = input.iter().map(|x| x.split(" ").collect()).collect();

    println!("Part 1: {}", part1);

    let mut sum = 0;
    for group in part2 {
        let mut hm: HashMap<u8, usize> = HashMap::new();

        let person_count = group.len();

        for person in group {
            for answer in person.split("").filter(|x| !x.is_empty()) {
                *hm.entry(answer.as_bytes()[0]).or_insert(0) += 1;
            }
        }

        sum += hm.into_iter().filter(|&(_, count)| count == person_count).count();
    }

    println!("Part 2: {}", sum);

    Ok(())
}
