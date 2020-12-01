use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    for i in &input {
        for j in &input {
            let a: u32 = i.parse().unwrap();
            let b: u32 = j.parse().unwrap();

            let res = a + b;

            if res == 2020 {
                println!("Result A: {}", a * b);
            }

            for n in &input {
                let c: u32 = n.parse().unwrap();

                let res = a + b + c;
                if res == 2020 {
                    println!("Result B: {}", a * b * c);
                }
            }
        }
    }

    Ok(())
}
