use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"mask = (\w+)").unwrap();
    static ref RE_MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

struct CPU {
    memory: HashMap<u64, u64>,
    mask: String,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memory: HashMap::new(),
            mask: "".repeat(36),
        }
    }

    fn apply_mask(&self, value: u64) -> u64 {
        let or = u64::from_str_radix(&self.mask.replace("X", "0"), 2).unwrap();
        let and = u64::from_str_radix(&self.mask.replace("X", "1"), 2).unwrap();

        let mut result = value;
        result &= and;
        result |= or;
        result
    }


    fn get_locations(&self, location: u64) -> Vec<u64> {
        let mut locations: Vec<u64> = vec![];
        let or_mask = u64::from_str_radix(&self.mask.replace("X", "0"), 2).unwrap();
        let location = location | or_mask;
        let location = self.mask.chars().enumerate().map(|(idx, c)| {
            if c == 'X' {
                c
            } else {
                if ((location & (1 << (35 - idx))) >> (35 - idx)) == 1 {
                    '1'
                } else {
                   '0'
                }
            }
        }).collect::<String>();

        let count: u32 = location.matches("X").count() as u32;
        for i in 0..2u64.pow(count) {
            let mut n = count;
            let adr = location.chars().enumerate().map(|(idx, c)| {
                if c == 'X' {
                    n -= 1;
                    if ((i & (1 << (n))) >> (n)) == 1 {
                        '1'
                    } else {
                        '0'
                    }
                } else {
                   c
                }
            }).collect::<String>();

            locations.push(u64::from_str_radix(&adr, 2).unwrap());
        }
        locations
    }

    fn sum_memory(&self) -> u64 {
        let mut sum = 0;
        for (_, value) in self.memory.iter() {
            sum += value;
        }
        sum
    }

    fn set_mask(&mut self, mask: String) {
        self.mask = mask;
    }

    fn write_to_mem(&mut self, value: u64, pos: u64) {
        self.memory.insert(pos, self.apply_mask(value));
    }

    fn write_to_mem_translate(&mut self, value: u64, pos: u64) {
        for location in self.get_locations(pos) {
            self.memory.insert(location, value);
        }
    }
}

enum Instruction {
    MASK(String),
    MEM(u64, u64),
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut cpu = CPU::new();

    let input: Vec<Instruction> = input.iter().map(|x| {
        if x.starts_with("mask") {
            let cap = RE_MASK.captures(x).unwrap();
            let mask = cap.get(1).unwrap().as_str();
            return Instruction::MASK(mask.to_string());
        } else {
            let cap = RE_MEM.captures(x).unwrap();
            let idx = cap.get(1).unwrap().as_str().parse().unwrap();
            let val = cap.get(2).unwrap().as_str().parse().unwrap();
            return Instruction::MEM(idx, val);
        }
    }).collect();

    for instruction in &input {
        match instruction {
            Instruction::MASK(mask) => { cpu.set_mask(mask.clone()) }
            Instruction::MEM(idx, val) => { cpu.write_to_mem(*val, *idx) }
        }
    }

    println!("Part 1: {:?}", cpu.sum_memory());

    let mut cpu = CPU::new();


    for instruction in &input {
        match instruction {
            Instruction::MASK(mask) => { cpu.set_mask(mask.clone()) }
            Instruction::MEM(idx, val) => { cpu.write_to_mem_translate(*val, *idx) }
        }
    }

    println!("Part 2: {:?}", cpu.sum_memory());

    Ok(())
}
