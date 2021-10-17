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
    pub fn mov(&mut self, d: Direction) {
        match d {
            Direction::Up => self.y = self.y - 1,
            Direction::Down => self.y = self.y + 1,
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
        }
    }
}


fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut position = Position { x: 0, y: 0 };

    let mut visits: HashMap<(i32, i32), ()> = HashMap::new();

    for ch in buffer {
        let dir = Direction::parse(&ch).unwrap();
        position.mov(dir);
        visits.insert((position.x, position.y), ());
    }

    println!("position: {:?}", position);
    println!("count: {:?}", visits.len());

    Ok(())
}