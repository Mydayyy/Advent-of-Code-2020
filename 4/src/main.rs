use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::fmt;
use itertools::{join, Itertools};

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::{Regex, Captures};

lazy_static! {
        static ref RE_BYR: Regex = Regex::new(r"byr:(\d+)").unwrap();
        static ref RE_IYR: Regex = Regex::new(r"iyr:(\d+)").unwrap();
        static ref RE_EYR: Regex = Regex::new(r"eyr:(\d+)").unwrap();
        static ref RE_HGT: Regex = Regex::new(r"hgt:(\w+)").unwrap();
        static ref RE_HCL: Regex = Regex::new(r"hcl:(#?\w+)").unwrap();
        static ref RE_ECL: Regex = Regex::new(r"ecl:(#?\w+)").unwrap();
        static ref RE_PID: Regex = Regex::new(r"pid:(#?\w+)").unwrap();
        static ref RE_CID: Regex = Regex::new(r"cid:(\d+)").unwrap();
    }

struct Passport {
    data: String,

    //(Birth Year)
    byr: Option<u32>,

    // (Issue Year)
    iyr: Option<u32>,

    // (Expiration Year)
    eyr: Option<u32>,

    // (Height)
    hgt: Option<String>,

    //(Hair Color)
    hcl: Option<String>,

    //(Eye Color)
    ecl: Option<String>,

    //(Passport ID)
    pid: Option<String>,

    //(Country ID)
    cid: Option<u32>,

}

fn get_int(cap: &Option<Captures>) -> Option<u32> {
    match cap {
        None => None,
        Some(x) => { x.get(1).unwrap().as_str().parse().ok() }
    }
}

fn get_string(cap: &Option<Captures>) -> Option<String> {
    match cap {
        None => None,
        Some(x) => { Some(x.get(1).unwrap().as_str().to_string()) }
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        return self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some();
    }

    fn is_valid_constraints(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        lazy_static! {
            static ref RE_NUMBER: Regex = Regex::new(r"(\d+)").unwrap();
            static ref RE_CHECK_HCL: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
            static ref RE_CHECK_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref RE_CHECK_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        if !self.byr.map_or(false, |x| (1920..=2002).contains(&x)) {
            return false;
        }

        if !self.iyr.map_or(false, |x| (2010..=2020).contains(&x)) {
            return false;
        }

        if !self.eyr.map_or(false, |x| (2020..=2030).contains(&x)) {
            return false;
        }

        if !self.hgt.as_ref().map_or(false, |x| {
            let number: u32 = RE_NUMBER.captures(&x).unwrap().get(1).unwrap().as_str().parse().unwrap();

            if !x.ends_with("cm") && !x.ends_with("in") {
                return false;
            }

            return if x.ends_with("cm") {
                (150..=193).contains(&number)
            } else {
                (59..=76).contains(&number)
            };
        }) {
            return false;
        }

        if !self.hcl.as_ref().map_or(true, |x| {
            return RE_CHECK_HCL.is_match(&x);
        }) {
            return false;
        }

        if !self.ecl.as_ref().map_or(false, |x| {
            return RE_CHECK_ECL.is_match(&x);
        }) {
            return false;
        }

        if !self.pid.as_ref().map_or(false, |x| {
            return RE_CHECK_PID.is_match(&x);
        }) {
            return false;
        }
        return true;
    }

    fn new(data: String) -> Option<Passport> {
        let byr = RE_BYR.captures(&data);
        let iyr = RE_IYR.captures(&data);
        let eyr = RE_EYR.captures(&data);
        let hgt = RE_HGT.captures(&data);
        let hcl = RE_HCL.captures(&data);
        let ecl = RE_ECL.captures(&data);
        let pid = RE_PID.captures(&data);
        let cid = RE_CID.captures(&data);
        Some(Passport {
            data: data.clone(),
            byr: get_int(&byr),
            iyr: get_int(&iyr),
            eyr: get_int(&eyr),
            hgt: get_string(&hgt),
            hcl: get_string(&hcl),
            ecl: get_string(&ecl),
            pid: get_string(&pid),
            cid: get_int(&cid),
        })
    }
}

impl fmt::Debug for Passport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let byr = match self.byr {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };

        let iyr = match self.iyr {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };

        let eyr = match self.eyr {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };

        let hgt = match &self.hgt {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };

        let hcl = match &self.hcl {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };


        let ecl = match &self.ecl {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };


        let pid = match &self.pid {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };


        let cid = match &self.cid {
            None => "_".to_string(),
            Some(x) => format!("{}", x),
        };

        write!(f, "byr>{}|iyr>{}|eyr>{}|hgt>{}|hcl>{}|ecl>{}|pid>{}|cid>{}", byr, iyr, eyr, hgt, hcl, ecl, pid, cid)
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();


    let input: Vec<Passport> = input.iter()
        .group_by(|&l| l.is_empty())
        .into_iter()
        .map(|(_, g)| join(g, " "))
        .filter(|l| !l.is_empty()).map(|x| Passport::new(x).unwrap()).collect();

    // println!("{:?}", input);

    let valid1 = input.iter().filter(|x| x.is_valid()).count();
    let valid2 = input.iter().filter(|x| x.is_valid_constraints()).count();

    println!("Part 1:  {}", valid1);
    println!("Part 2:  {}", valid2);
    Ok(())
}
