use std::io::{BufReader, BufRead};
use std::{io, fmt};
use std::fs::File;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Copy, Clone)]
enum SeatState {
    FLOOR,
    OCCUPIED,
    EMPTY,
}

fn sum_alive_neighbors(i: i32, j: i32, input: &Vec<Vec<SeatState>>) -> usize {
    let row_limit: i32 = input.len() as i32 - 1;
    let column_limit: i32 = input.get(0).unwrap().len() as i32 - 1;

    let mut sum = 0;
    for x in std::cmp::max(0i32, i - 1)..=std::cmp::min(i + 1, row_limit) {
        for y in std::cmp::max(0, j - 1)..=std::cmp::min(j + 1, column_limit) {
            if (x != i || y != j) && input.get(x as usize).unwrap().get(y as usize).unwrap() == &SeatState::OCCUPIED {
                sum += 1;
            }
        }
    }

    sum
}

fn get_seatstate(x: i32, y: i32, input: &Vec<Vec<SeatState>>) -> &SeatState {
    return input.get(y as usize).unwrap().get(x as usize).unwrap();
}

fn raytrace(x: i32, y: i32, xDir: i32, yDir: i32, input: &Vec<Vec<SeatState>>) -> SeatState {
    let mut x = x;
    let mut y = y;

    loop {
        x += xDir;
        y += yDir;

        if x < 0 || y < 0 || y >= input.len() as i32 || x >= input.get(0).unwrap().len() as i32 {
            return SeatState::FLOOR;
        }

        let e = get_seatstate(x, y, &input);

        if e == &SeatState::EMPTY {
            return SeatState::EMPTY;
        }
        if e == &SeatState::OCCUPIED {
            return SeatState::OCCUPIED;
        }
    }
}

fn sum_alive_neighbors_raytrace(i: i32, j: i32, input: &Vec<Vec<SeatState>>) -> usize {
    let row_limit: i32 = input.len() as i32 - 1;
    let column_limit: i32 = input.get(0).unwrap().len() as i32 - 1;


    // let mut sum = 0;
    // for x in std::cmp::max(0i32, i - 1)..=std::cmp::min(i + 1, row_limit) {
    //     for y in std::cmp::max(0, j - 1)..=std::cmp::min(j + 1, column_limit) {
    //         if x != i || y != j {
    //             if raytrace(i,j,y-j , x-i, &input) == SeatState::OCCUPIED {
    //                 sum += 1;
    //             }
    //         }
    //     }
    // }
    let mut sum = 0;
    for (x_dir, y_dir) in &[(1,0),(-1,0),(0,1),(0,-1),   (-1, -1), (-1, 1), (1,-1),(1,1)] {
        if raytrace(j, i, *x_dir, *y_dir, &input) == SeatState::OCCUPIED {
            sum += 1;
        }
    }
sum
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let input: Vec<Vec<SeatState>> = input.iter()
        .map(|x| x.chars()
            .map(|c| match c {
                '.' => { SeatState::FLOOR }
                '#' => { SeatState::OCCUPIED }
                'L' => { SeatState::EMPTY }
                _ => { panic!("Unknown Seat Type") }
            }).collect()).collect();

    // let field = [[SeatState::FREE, input.get(0).unwrap().len()]; input.len()];

    let mut changes = 1;
    let mut transformed = input.clone();
    while changes > 0 {
        changes = 0;
        transformed = transformed.iter().enumerate()
            .map(|(x, v)| v.iter().enumerate()
                .map(|(y, vv)| match (vv, sum_alive_neighbors(x as i32, y as i32, &transformed)) {
                    (SeatState::EMPTY, 0) => {
                        changes += 1;
                        SeatState::OCCUPIED
                    }
                    (SeatState::OCCUPIED, x) if x >= 4 => {
                        changes += 1;
                        SeatState::EMPTY
                    }
                    _ => { *vv }
                }).collect())
            .collect();
    }


    let mut occupied = 0;
    for row in &transformed {
        for seat in row {
            if *seat == SeatState::OCCUPIED {
                occupied += 1;
            }
        }
    }

    println!("Part 1: {:?}", occupied);


    let mut changes = 1;
    let mut transformed = input.clone();
    while changes > 0 {
        changes = 0;
        transformed = transformed.iter().enumerate()
            .map(|(x, v)| v.iter().enumerate()
                .map(|(y, vv)| match (vv, sum_alive_neighbors_raytrace(x as i32, y as i32, &transformed)) {
                    (SeatState::EMPTY, 0) => {
                        changes += 1;
                        SeatState::OCCUPIED
                    }
                    (SeatState::OCCUPIED, x) if x >= 5 => {
                        changes += 1;
                        SeatState::EMPTY
                    }
                    _ => { *vv }
                }).collect())
            .collect();
    }


    let mut occupied = 0;
    for row in &transformed {
        for seat in row {
            if *seat == SeatState::OCCUPIED {
                occupied += 1;
            }
        }
    }


    println!("Part 2: {:?}", occupied);

    Ok(())
}
