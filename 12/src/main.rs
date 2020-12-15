use std::io::{BufReader, BufRead};
use std::{io, fmt};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::mem::swap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    NORTH(i32),
    SOUTH(i32),
    EAST(i32),
    WEST(i32),
    LEFT(i32),
    RIGHT(i32),
    FORWARD(i32),
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new(x: i32, y: i32) -> Waypoint {
        Waypoint { x, y }
    }
    fn north(&mut self, distance: i32) {
        self.y += distance;
    }
    fn south(&mut self, distance: i32) {
        self.y -= distance;
    }
    fn east(&mut self, distance: i32) {
        self.x += distance;
    }
    fn west(&mut self, distance: i32) {
        self.x -= distance;
    }
    fn right(&mut self, angle: i32) {
        let v = angle/90;

        for i in 1..=v {
            swap(&mut self.x, &mut self.y);
            self.y *= -1;
        }
    }

    fn left(&mut self, angle: i32) {
        let v = angle/90;

        for i in 1..=v {
            swap(&mut self.x, &mut self.y);
            self.x *= -1;
        }
    }
}

struct Ship {
    angle: i32,
    x: i32,
    y: i32,

}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            angle: 0,
        }
    }
    fn north(&mut self, distance: i32) {
        self.y += distance;
    }
    fn south(&mut self, distance: i32) {
        self.y -= distance;
    }
    fn east(&mut self, distance: i32) {
        self.x += distance;
    }
    fn west(&mut self, distance: i32) {
        self.x -= distance;
    }

    fn right(&mut self, angle: i32) {
        self.angle -= angle;
    }
    fn left(&mut self, angle: i32) {
        self.angle += angle;
    }

    fn forward(&mut self, distance: i32) {
        let radians = (self.angle as f32).to_radians();
        self.x = (radians.cos() * distance as f32 + self.x as f32).round() as i32;
        self.y = (radians.sin() * distance as f32 + self.y as f32).round() as i32;
    }

    fn forward_waypoint(&mut self, w: &Waypoint,  distance: i32) {
        self.x += distance * w.x;
        self.y += distance * w.y;
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let input: Vec<Action> = input.iter().map(|x| {
        let val: i32 = x[1..].parse().unwrap();

        match &x[..1] {
            "N" => { Action::NORTH(val) }
            "S" => { Action::SOUTH(val) }
            "E" => { Action::EAST(val) }
            "W" => { Action::WEST(val) }
            "L" => { Action::LEFT(val) }
            "R" => { Action::RIGHT(val) }
            "F" => { Action::FORWARD(val) }
            _ => {
                panic!("Invalid Instruction");
            }
        }
    }).collect();

    let mut ship = Ship::new();
    for action in &input {
        match action {
            Action::NORTH(distance) => { ship.north(*distance) }
            Action::SOUTH(distance) => { ship.south(*distance) }
            Action::EAST(distance) => { ship.east(*distance) }
            Action::WEST(distance) => { ship.west(*distance) }
            Action::LEFT(angle) => { ship.left(*angle) }
            Action::RIGHT(angle) => { ship.right(*angle) }
            Action::FORWARD(distance) => { ship.forward(*distance) }
        }
    }
    println!("Part 1: {}", ship.x.abs() + ship.y.abs());

    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new(10, 1);
    for action in &input {
        match action {
            Action::NORTH(distance) => { waypoint.north(*distance) }
            Action::SOUTH(distance) => { waypoint.south(*distance) }
            Action::EAST(distance) => { waypoint.east(*distance) }
            Action::WEST(distance) => { waypoint.west(*distance) }
            Action::LEFT(angle) => { waypoint.left(*angle) }
            Action::RIGHT(angle) => { waypoint.right(*angle) }
            Action::FORWARD(distance) => { ship.forward_waypoint(&waypoint, *distance) }
        }
    }
    println!("Part 2: {}", ship.x.abs() + ship.y.abs());

    Ok(())
}
