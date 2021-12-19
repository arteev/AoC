use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, self};

#[derive(Debug)]
struct Board {
    v: Vec<[i32; 5]>,
    s: Vec<[bool; 5]>,
}

impl Board {
    fn new() -> Board {
        Board {
            v: Vec::new(),
            s: Vec::new(),
        }
    }

    fn is_full(&self) -> bool {
        self.v.len() == 5
    }

    fn append_row(&mut self, row: [i32; 5]) {
        self.v.push(row);
        self.s.push([false; 5]);
    }

    fn bingo(&mut self, number: i32) -> bool {
        for (i, row) in self.v.iter().enumerate() {
            let mut is_bingo = true;
            for (j, n) in row.iter().enumerate() {
                if *n == number {
                    self.s[i][j] = true
                }
                is_bingo = is_bingo && self.s[i][j];
            }
            if is_bingo {
                return true;
            }
        }

        for i in 0..self.v[0].len() {
            let mut is_bingo = true;
            for (j, _) in self.v.iter().enumerate() {
                is_bingo = is_bingo && self.s[j][i];
            }
            if is_bingo {
                return true;
            }
        }
        false
    }

    fn score(&self) -> i32 {
        let mut s = 0;
        for (i, row) in self.v.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if !self.s[i][j] {
                    s += n;
                }
            }
        }
        s
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut numbers = Vec::new();
    let mut current = Board::new();
    let mut boards = Vec::new();
    for line in lines {
        let v = line?;
        if numbers.is_empty() {
            let mut s = v.split(",").map(|s| {
                s.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>();
            numbers.append(&mut s);
            continue;
        }
        if v.is_empty() {
            continue;
        }


        let row = v.split(" ").filter(|&s| !s.trim().is_empty()).map(|s| {
            s.parse::<i32>().unwrap()
        }).take(5).collect::<Vec<i32>>();

        current.append_row(row.try_into().unwrap());

        if current.is_full() {
            boards.push(current);
            current = Board::new();
        }
    }

    for n in numbers {
        for b in boards.iter_mut() {
            if b.bingo(n) {
                let sum = b.score();
                println!("score: {} (n={})", sum * n, n);
                return Ok(());
            }
        }
    }
    Ok(())
}
