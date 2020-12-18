use std::io::{BufReader, BufRead};
use std::fs::File;
use itertools::{Itertools, join};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::num::ParseIntError;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fmt::{Debug, Formatter};
use core::fmt;

lazy_static! {
    static ref RE_RANGE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
}


struct Zone {
    valid: Vec<RangeInclusive<usize>>,
    name: String,
}

impl Zone {
    fn new(name: &str) -> Zone {
        Zone {
            valid: vec![],
            name: name.to_string(),
        }
    }
    fn add_range(&mut self, r: RangeInclusive<usize>) {
        self.valid.push(r);
    }
    fn is_valid(&self, number: usize) -> bool {
        for v in &self.valid {
            if v.contains(&number) {
                return true
            }
        }
        return false;
    }
}

struct Zones {
    zones: Vec<Zone>
}


impl Zones {
    fn new() -> Zones {
        Zones { zones: vec![] }
    }

    fn add_zone(&mut self, name: &str, ranges: Vec<RangeInclusive<usize>>) {
        let mut zone = Zone::new(name);
        for r in ranges {
            zone.add_range(r);
        }
        self.zones.push(zone);
    }

    fn is_valid(&self, number: usize) -> bool{
        for z in &self.zones {
            if z.is_valid(number) {
                return true;
            }
        }
        false
    }
}


fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    ;
    let input: Vec<Vec<String>> = reader.lines().filter_map(Result::ok).group_by(|x| x.is_empty()).into_iter().map(|(x, g)| {
        g.collect_vec()
    }).filter(|x| !(x.len() == 1 && x[0].is_empty())).collect();

    let z: Vec<String> = input.get(0).unwrap().clone();
    let your_ticket: Vec<String> = input.get(1).unwrap().clone().into_iter().skip(1).collect();
    let nearby_tickets: Vec<Vec<usize>> = input.get(2).unwrap().clone().into_iter().skip(1).map(|x| x.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>()).collect();

    let mut zones = Zones::new();
    for zone in z {
        let name = zone.split(":").take(1).collect::<String>();

        let caps = RE_RANGE.captures_iter(&zone);

        zones.add_zone(&name, caps.map(|x| {
            let from = x.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let to = x.get(2).unwrap().as_str().parse::<usize>().unwrap();
            from..=to
        }).collect());
    }

    println!("{:?}", zones);
    println!("{:?}", your_ticket);
    println!("{:?}", nearby_tickets);

    // Part 1
    let mut error_rate = 0;
    for  ticket in nearby_tickets {
        for number in ticket {
            if !zones.is_valid(number) {
                error_rate  += number;
            }
        }
    }
    println!("Part 1: {}", error_rate);

    Ok(())
}

impl fmt::Debug for Zones {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.zones)
    }
}

impl fmt::Debug for Zone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.valid)
    }
}