use std::error::Error;
use std::io::{self, BufRead};
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(ch: &char) -> Result<Self, &'static str> {
        match ch {
            'U' => Ok(Move::Up),
            'D' => Ok(Move::Down),
            'L' => Ok(Move::Left),
            'R' => Ok(Move::Right),
            _ => Err("invalid move")
        }
    }
}

struct Point(i32, i32);

impl Point {
    fn digit(&self) -> i32 {
        let d = 5 + self.0 + (3 * self.1);
        if d < 1 {
            return 1;
        } else if d > 9 { return 9; }
        d
    }
}

fn mov(position: &mut Point, d: Move) {
    match d {
        Move::Up => position.1 -= 1,
        Move::Down => position.1 += 1,
        Move::Left => position.0 -= 1,
        Move::Right => position.0 += 1,
    }

    if position.0 > 1 {
        position.0 = 1
    }

    if position.0 < -1 {
        position.0 = -1
    }

    if position.1 > 1 {
        position.1 = 1
    }

    if position.1 < -1 {
        position.1 = -1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut position = Point(0, 0);
    let mut didits = String::new();

    for line in lines {
        let l = line?;
        if l.len() == 0 {
            continue;
        }
        for ch in l.chars() {
            let m = Move::from_char(&ch);
            mov(&mut position, m?)
        }
        didits.push_str(position.digit().to_string().as_str())
    }
    println!("{:?}", didits);
    Ok(())
}

