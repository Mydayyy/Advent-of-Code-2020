use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use fnv::{FnvHasher, FnvBuildHasher};
use std::hash::BuildHasherDefault;

fn update_highest(number: i64, new_value: i64, hm: &mut HashMap<i64, (i64, i64), BuildHasherDefault<FnvHasher>>) {
    let val = match hm.get(&number) {
        None => {
            (new_value, -1)
        }
        Some((val0, val1)) => {
            if val1 > val0 {
                (new_value, *val1)
            } else {
                (*val0, new_value)
            }
        }
    };
    hm.insert(number, val);
}

fn solve(till_turn: i64, starting_numbers: &Vec<i64>) -> i64 {
    let mut history: HashMap<i64, (i64, i64), BuildHasherDefault<FnvHasher>> = HashMap::with_hasher(FnvBuildHasher::default());
    // let mut history:  = starting_numbers.iter().enumerate().map(|(idx, val)| (*val, (1 + idx as i64, -1))).collect();
    for (idx, val) in starting_numbers.iter().enumerate() {
        history.insert(*val, (1+idx as i64, -1));
    }
    let mut last_spoken = *starting_numbers.last().unwrap();
    for i in (starting_numbers.len() as i64 + 1)..(till_turn + 1) {
        let mut new_number = 0;

        let gkv = history.get_key_value(&last_spoken).unwrap();
        let first_time = gkv.1.1 == -1;
        if first_time {
            last_spoken = new_number;
            update_highest(last_spoken, i, &mut history);
        } else {
            new_number = (gkv.1.0.abs() - gkv.1.1.abs()).abs();
            last_spoken = new_number;
            update_highest(last_spoken, i, &mut history);
        }
    }
    last_spoken
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let starting_numbers: Vec<i64> = input.get(0).unwrap().split(",").map(|x| x.parse().unwrap()).collect();


    println!("Part 1: {:?}", solve(2020, &starting_numbers));
    println!("Part 2: {:?}", solve(30000000, &starting_numbers));


    Ok(())
}
