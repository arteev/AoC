use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};
use std::fs::File;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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


struct ButtonPin {
    b: String,
    edges: Vec<Move>,
    pos: Point,
}

impl ButtonPin {
    pub fn new(b: &str, pos: Point, edges: Vec<Move>) -> ButtonPin {
        ButtonPin {
            b: b.to_string(),
            edges,
            pos,
        }
    }
}

struct PinPad {
    buttons: HashMap<Point, ButtonPin>,
    position: Point,
}

impl PinPad {
    pub fn new(buttons: Vec<ButtonPin>, position: Point) -> PinPad {
        let mut p = PinPad {
            buttons: HashMap::new(),
            position,
        };
        for b in buttons {
            p.buttons.insert(b.pos, b);
        }
        p
    }

    pub fn current(&self) -> Point {
        self.position
    }

    pub fn mov(&mut self, m: Move) {
        let b = self.buttons.get(&self.current()).unwrap();
        let f = b.edges.iter().find(|&x| *x == m);
        if f.is_none() {
            match m {
                Move::Up => self.position.1 -= 1,
                Move::Down => self.position.1 += 1,
                Move::Left => self.position.0 -= 1,
                Move::Right => self.position.0 += 1,
            }
        }
    }

    pub fn current_digit(&self) -> String {
        self.buttons.get(&self.current()).unwrap().b.to_owned()
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32);


fn main() -> Result<(), Box<dyn Error>> {
    let buttons: Vec<ButtonPin> = vec![
        ButtonPin::new("1", Point(-1, -1), vec![Move::Up, Move::Left]),
        ButtonPin::new("2", Point(0, -1), vec![Move::Up]),
        ButtonPin::new("3", Point(1, -1), vec![Move::Up, Move::Right]),
        ButtonPin::new("4", Point(-1, 0), vec![Move::Left]),
        ButtonPin::new("5", Point(0, 0), vec![]),
        ButtonPin::new("6", Point(1, 0), vec![Move::Right]),
        ButtonPin::new("7", Point(-1, 1), vec![Move::Left, Move::Down]),
        ButtonPin::new("8", Point(0, 1), vec![Move::Down]),
        ButtonPin::new("9", Point(1, 1), vec![Move::Right, Move::Down]),
    ];
    let mut pinpad1 = PinPad::new(buttons, Point(0, 0));

    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut digits = String::new();

    for line in lines {
        let l = line?;
        if l.len() == 0 {
            continue;
        }
        for ch in l.chars() {
            let m = Move::from_char(&ch);
            pinpad1.mov(m?);
        }
        digits.push_str(pinpad1.current_digit().as_str())
    }
    println!("{:?}", digits);
    Ok(())
}

