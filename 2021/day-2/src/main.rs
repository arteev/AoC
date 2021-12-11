use std::error::Error;
use std::fs::File;
use std::io::{BufRead, self};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Movement {
    FORWARD,
    DOWN,
    UP,
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Movement::FORWARD),
            "down" => Ok(Movement::DOWN),
            "up" => Ok(Movement::UP),
            _ => Err("invalid movement")
        }
    }
}

#[derive(Copy, Clone)]
struct Position {
    distance: i32,
    depth: i32,
}

trait Strategy {
    fn mov(&mut self, m: Movement, distance: i32);
    fn get_position(&self) -> Position;
}

struct Part1Strategy {
    p: Position,
}

impl Part1Strategy {
    fn new() -> Part1Strategy {
        Part1Strategy {
            p: Position { depth: 0, distance: 0 },
        }
    }
}

impl Strategy for Part1Strategy {
    fn mov(&mut self, m: Movement, distance: i32) {
        match m {
            Movement::FORWARD => self.p.distance += distance,
            _ => self.p.depth += match m {
                Movement::DOWN => distance,
                Movement::UP => -1 * distance,
                _ => panic!("invalid movements")
            }
        };
    }

    fn get_position(&self) -> Position {
        self.p
    }
}


struct Part2Strategy {
    aim: i32,
    p: Position,
}

impl Part2Strategy {
    fn new() -> Part2Strategy {
        Part2Strategy {
            aim: 0,
            p: Position { depth: 0, distance: 0 },
        }
    }
}

impl Strategy for Part2Strategy {
    fn mov(&mut self, m: Movement, distance: i32) {
        match m {
            Movement::FORWARD => {
                self.p.distance += distance;
                if self.aim > 0 {
                    self.p.depth += distance * self.aim;
                }
            }
            _ => self.aim += match m {
                Movement::DOWN => distance,
                Movement::UP => -1 * distance,
                _ => panic!("invalid movements")
            }
        };
    }

    fn get_position(&self) -> Position {
        self.p
    }
}


fn printStrategy(s: &impl Strategy) {
    let pos = s.get_position();
    println!("distance: {}", pos.distance);
    println!("depth: {}", pos.depth);
    println!("multiply: {}", pos.distance * pos.depth);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut part1 = Part1Strategy::new();
    let mut part2 = Part2Strategy::new();
    for line in lines {
        let v = line?;
        let values = v.splitn(2, " ").collect::<Vec<&str>>();
        let movement: Movement = values[0].parse().unwrap();
        let steps: i32 = values[1].parse().unwrap();
        part1.mov(movement, steps);
        part2.mov(movement, steps);
    }
    printStrategy(&part1);
    printStrategy(&part2);

    Ok(())
}
