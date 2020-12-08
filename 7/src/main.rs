use std::io::{BufReader, BufRead};
use std::io;
use std::fs::File;
use itertools::{Itertools, join};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::{Regex, Captures, RegexBuilder};

lazy_static! {
        static ref RE_BAGS: Regex = RegexBuilder::new(r"(\d+) (\w+ \w+) bag").multi_line(true).build().unwrap();
    }

fn contains_shiny_gold(bags: &HashMap<String, HashMap<String, u32>>, name: &str) -> bool {
    if name == "shiny gold" {
        return true;
    }

    if !bags.contains_key(name) {
        return false;
    }

    let mut found = false;
    for (idx, val) in bags.get(name).unwrap() {


        found = found || contains_shiny_gold(bags, idx);
    }

    return found;
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let mut Bags: HashMap<String, HashMap<String, u32>> = HashMap::new();


    for rule in input {
        if rule.contains("bags contain no other bags") {
            continue;
        }

        let mut Bag: HashMap<String, u32> = HashMap::new();


        let name: String = rule.split(" ").take(2).join(" ");

        let caps = RE_BAGS.captures_iter(&rule);

        for cap in caps {
            let cap_name = cap.get(2).unwrap().as_str();
            let cap_count: u32 = cap.get(1).unwrap().as_str().parse().unwrap();

            Bag.insert(cap_name.to_string(), cap_count);
        }
        Bags.insert(name, Bag);


        // println!("{:?}", cap_count);
    }

    println!("{:?}", contains_shiny_gold(&Bags, "shiny gold"));



    let mut count = 0;
    for (idx, val) in &Bags {
        if idx == "shiny gold" {
            continue
        }
        if contains_shiny_gold(&Bags, &idx) {
            count += 1;
        }
    }

    println!("{:?}", count);

    Ok(())
}
