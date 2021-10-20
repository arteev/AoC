use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;

use std::io::{self, BufRead};
use std::fs::File;

use std::error::Error;

#[derive(Debug)]
enum Operation {
    TOGGLE,
    ON,
    OFF,
}

impl FromStr for Operation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle" => Ok(Operation::TOGGLE),
            "turn on" => Ok(Operation::ON),
            "turn off" => Ok(Operation::OFF),
            _ => Err("invalid operation"),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"^(toggle|turn on|turn off)\s(\d{1,3},\d{1,3})\sthrough\s(\d{1,3},\d{1,3})").unwrap();
    let mut grid = [[false; 1000]; 1000];
    let mut grid_brightness = [[0i32; 1000]; 1000];
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    let mut brightness = 0;

    for line in lines {
        let v = line?;
        let caps = re.captures(&v).unwrap();
        let oper = Operation::from_str(caps.get(1).unwrap().as_str())?;
        let point_start = Point::from_str(caps.get(2).unwrap().as_str())?;
        let point_end = Point::from_str(caps.get(3).unwrap().as_str())?;

        for i in point_start.x..=point_end.x {
            for j in point_start.y..=point_end.y {
                let i_idx = i as usize;
                let j_idx = j as usize;
                let old_state = grid[i_idx][j_idx];
                let old_brightness = grid_brightness[i_idx][j_idx];

                match &oper {
                    Operation::TOGGLE => {
                        grid[i_idx][j_idx] = !grid[i_idx][j_idx];
                        grid_brightness[i_idx][j_idx] += 2;
                    }

                    Operation::ON => {
                        grid[i_idx][j_idx] = true;
                        grid_brightness[i_idx][j_idx] += 1;
                    }

                    Operation::OFF => {
                        grid[i_idx][j_idx] = false;

                        grid_brightness[i_idx][j_idx] -= 1;
                        if grid_brightness[i_idx][j_idx] < 0 {
                            grid_brightness[i_idx][j_idx] = 0;
                        }
                    }
                };
                if old_state != grid[i_idx][j_idx] {
                    if grid[i_idx][j_idx] { count += 1 } else { count -= 1 }
                };
                if old_brightness != grid_brightness[i_idx][j_idx] {
                    brightness += grid_brightness[i_idx][j_idx] - old_brightness;
                }
            }
        }
    }

    println!("count {}", count);
    println!("brightness {}", brightness);

    Ok(())
}
