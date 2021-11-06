use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::fs::File;

#[derive(Debug, Copy, Clone)]
enum CardinalPoints {
    North,
    South,
    East,
    West,
}

impl CardinalPoints {
    fn turn(&self, d: &DirectionMove) -> CardinalPoints {
        match self {
            CardinalPoints::North => match d {
                DirectionMove::Left => CardinalPoints::West,
                DirectionMove::Right => CardinalPoints::East,
            },
            CardinalPoints::South => match d {
                DirectionMove::Left => CardinalPoints::East,
                DirectionMove::Right => CardinalPoints::West,
            },
            CardinalPoints::East => match d {
                DirectionMove::Left => CardinalPoints::North,
                DirectionMove::Right => CardinalPoints::South,
            },
            CardinalPoints::West => match d {
                DirectionMove::Left => CardinalPoints::South,
                DirectionMove::Right => CardinalPoints::North,
            },
        }
    }
}

#[derive(Debug)]
enum DirectionMove {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruct {
    direction: DirectionMove,
    steps: i32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn points_visits(&self, to: &Point) -> Vec<Point> {
        let mut v = Vec::new();
        if *&self.0 != to.0 {
            let fp = self.0;
            for x in fp.min(to.0)..=fp.max(to.0) {
                v.push(Point(x, *&self.1));
            }
        }

        if *&self.1 != to.1 {
            let fp = self.1;
            for x in fp.min(to.1)..=fp.max(to.1) {
                v.push(Point(*&self.0, x));
            }
        }
        v
    }
}

impl FromStr for Instruct {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(ch) = s.chars().nth(0) {
            Ok(Instruct {
                direction: match ch {
                    'L' => DirectionMove::Left,
                    'R' => DirectionMove::Right,
                    _ => return Err("invalid direction"),
                },
                steps: s[1..].parse().unwrap(),
            })
        } else {
            Err("invalid direction")
        }
    }
}

fn run_easter_bunny(from: &Point, d: CardinalPoints, ins: &Vec<Instruct>) -> Point {
    let mut dir = d;
    let mut pos = Point(from.0, from.1);

    for instruction in ins {
        dir = dir.turn(&instruction.direction);
        match dir {
            CardinalPoints::North => { pos.1 += instruction.steps }
            CardinalPoints::South => { pos.1 -= instruction.steps }
            CardinalPoints::East => { pos.0 += instruction.steps }
            CardinalPoints::West => { pos.0 -= instruction.steps }
        }
    }
    pos
}

fn run_easter_bunny_check_twice(from: &Point, d: CardinalPoints, ins: &Vec<Instruct>) -> Point {
    let mut dir = d;
    let mut pos = Point(from.0, from.1);
    let mut visit: HashMap<Point, bool> = HashMap::new();

    for instruction in ins {
        dir = dir.turn(&instruction.direction);
        let old = Point(pos.0, pos.1);
        match dir {
            CardinalPoints::North => { pos.1 += instruction.steps; }
            CardinalPoints::South => { pos.1 -= instruction.steps; }
            CardinalPoints::East => { pos.0 += instruction.steps; }
            CardinalPoints::West => { pos.0 -= instruction.steps; }
        }
        let visits_arr = old.points_visits(&pos);
        for v in visits_arr.iter() {
            if *v == old {
                continue;
            }
            let is_visit = visit.get(&v);
            if let Some(vv) = is_visit {
                return Point(v.0, v.1);
            }
            visit.insert(Point(v.0, v.1), true);
        }
    }
    pos
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut ins: Vec<Instruct> = Vec::new();
    for line in lines {
        let v = line?;
        if v.len() == 0 {
            continue;
        }
        for item in v.split(",").into_iter() {
            let r: Instruct = item.trim().parse().unwrap();
            ins.push(r);
        }
    }
    let pos = run_easter_bunny(&Point(0, 0), CardinalPoints::North, &ins);
    println!("position: {:?}", pos);
    println!("blocks: {}", pos.0.abs() + pos.1.abs());

    let pos = run_easter_bunny_check_twice(&Point(0, 0), CardinalPoints::North, &ins);
    println!("position (check twice): {:?}", pos);
    println!("blocks (check twice): {}", pos.0.abs() + pos.1.abs());

    Ok(())
}
