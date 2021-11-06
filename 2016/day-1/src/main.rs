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

#[derive(Debug)]
struct Point(i32, i32);

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
    Ok(())
}
