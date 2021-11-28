use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;
use std::str::FromStr;
use std::vec;

#[derive(Debug)]
struct Triangle(i32, i32, i32);

impl Triangle {
    pub fn is_valid(&self) -> bool {
        self.0 < self.1 + self.2 &&
            self.1 < self.0 + self.2 &&
            self.2 < self.0 + self.1
    }
    pub fn field(&self, i: i32) -> i32 {
        match i {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("invalid index")
        }
    }
}

impl FromStr for Triangle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x: Vec<i32> = Vec::new();
        for s in s.split(" ").into_iter() {
            if let Ok(t) = s.parse::<i32>() {
                x.push(t);
            }
        };
        if x.len() != 3 {
            return Err("invalid length");
        }
        Ok(Triangle(
            x[0], x[1], x[2],
        ))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut count: i32 = 0;
    let mut count_by_column: i32 = 0;
    let mut triangles = Vec::new();
    for line in lines {
        if let Ok(v) = line {
            let t: Triangle = v.parse()?;
            if t.is_valid() {
                count += 1;
            }
            triangles.push(t);
        }
    }

    for i in (0..triangles.len()).step_by(3) {
        for idx in 0..3 {
            if Triangle(
                triangles[i].field(idx),
                triangles[i + 1].field(idx),
                triangles[i + 2].field(idx),
            ).is_valid() {
                count_by_column += 1;
            }
        }
    }
    println!("count {}", count);
    println!("count_by_column {}", count_by_column);
    Ok(())
}
