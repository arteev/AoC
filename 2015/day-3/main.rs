use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(s: &u8) -> Option<Direction> {
        match *s as char {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => panic!("unknown direction: {}", s)
        }
    }
}

impl Position {
    pub fn mov(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.y = self.y - 1,
            Direction::Down => self.y = self.y + 1,
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
        }
    }
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn coordinates(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}


fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut position = Position::new();

    let mut visits: HashMap<(i32, i32), ()> = HashMap::new();
    let mut visits_next_year: HashMap<(i32, i32), ()> = HashMap::new();

    let mut positions_next = HashMap::new();
    positions_next.insert(true, Position::new());
    positions_next.insert(false, Position::new());

    let mut who_moved = false;
    for ch in buffer {
        let dir = Direction::parse(&ch).unwrap();
        // now
        position.mov(&dir);
        visits.insert(position.coordinates(), ());

        // next year
        let p = positions_next.get_mut(&who_moved).unwrap();
        p.mov(&dir);
        visits_next_year.insert(p.coordinates(), ());
        who_moved = !who_moved;
    }

    println!("now count: {:?}", visits.len());
    println!("next year count: {:?}", visits_next_year.len());

    Ok(())
}