use std::io::{BufReader, BufRead};
use std::fs::File;

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let depart_time: usize = input.get(0).unwrap().parse().unwrap();
    let times: Vec<usize> = input.get(1).unwrap().split(",").filter(|x| x != &"x").map(|x| x.parse().unwrap()).collect();
    let mut depart: Vec<(usize, usize)> = times.iter().map(|x| (*x, depart_time + x - (depart_time % x))).collect();
    depart.sort_by(|x,y| x.1.cmp(&y.1));
    println!("Part 1: {:?}",(depart[0].1 - depart_time) * depart[0].0);

    let mut residues = vec![];
    let mut modulii = vec![];

    for (i, bus) in input.get(1).unwrap().split(",").enumerate() {
        if bus == "x" {
            continue;
        }

        let bus_id = bus.parse::<i64>().unwrap();

        residues.push(bus_id - i as i64);
        modulii.push(bus_id);
    }

    println!("Part 2: {:?}", chinese_remainder(&residues, &valid_buses).unwrap());

    Ok(())
}
