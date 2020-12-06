use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::Range;
use std::fmt::{Formatter};
use std::fmt;
use std::collections::HashMap;
use std::iter::FromIterator;

fn get_row_column(desc: &str, row: Option<Range<u32>>, column: Option<Range<u32>>) -> (u32, u32) {
    let row: Range<u32> = match row {
        None => { 0..128 }
        Some(x) => { x }
    };

    let column: Range<u32> = match column {
        None => { 0..8 }
        Some(x) => { x }
    };

    if desc.is_empty() {
        return (row.start, column.start);
    }

    match desc.as_bytes()[0] {
        b'F' => {
            let size = row.end - row.start;
            let row = row.start..(row.end - size / 2);
            // println!("row {:?}", row);
            return get_row_column(&desc[1..], Some(row), Some(column));
        }
        b'B' => {
            let size = row.end - row.start;
            let row = (row.start + size / 2)..row.end;
            // println!("row {:?}", row);
            return get_row_column(&desc[1..], Some(row), Some(column));
        }
        b'R' => {
            let size = column.end - column.start;
            let column = (column.start + size / 2)..column.end;
            // println!("column {:?}", column);
            return get_row_column(&desc[1..], Some(row), Some(column));
        }
        b'L' => {
            let size = column.end - column.start;
            let column = column.start..(column.end - size / 2);
            // println!("row {:?}", column);
            return get_row_column(&desc[1..], Some(row), Some(column));
        }
        _ => {
            panic!("Invalid Seat String")
        }
    };
}

struct Seat {
    row: Option<u32>,
    column: Option<u32>,
    seat_id: Option<u32>,
}

impl Seat {
    fn new(data: String) -> Seat {
        let seating = get_row_column(&data, None, None);
        Seat {
            row: Some(seating.0),
            column: Some(seating.1),
            seat_id: Some(seating.0 * 8 + seating.1),
        }
    }
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Seat({:?}|{:?}|{:?})", self.row, self.column, self.seat_id)
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<Seat> = reader.lines().filter_map(io::Result::ok).map(|x| Seat::new(x)).collect();
    // println!("{:?}", input);

    let highest = input.iter().fold(0, |a: u32, v| if v.seat_id.unwrap() > a { v.seat_id.unwrap() } else { a });
    let hm: HashMap<u32, &Seat> = HashMap::from_iter(input.iter().map(|x| (x.seat_id.unwrap(), x)));


    println!("Part 1: {}", highest);

    for i in 0..highest {
        if i > 0 && !hm.contains_key(&i) && hm.contains_key(&(i - 1)) && hm.contains_key(&(i + 1)) {
            println!("Part 2: {}", i)
        }
    }

    Ok(())
}
