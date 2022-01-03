use std::error::Error;
use std::fs::File;
use std::io::{BufRead,self};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Lanternfish {
    t: i32,
}

impl FromStr for Lanternfish {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        Ok(
            Lanternfish{
                t: s.parse()?,
            }
        )
    }
}

impl Lanternfish {
    fn tick(&mut self) -> bool {
        self.t -= 1;
        if self.t <0 {
            self.t = 6;
            return true;
        }
        false
    }
}

fn main() -> Result<(),Box<dyn Error>>{

    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x|x.unwrap());

    let mut fishes = Vec::new();

    for line in lines {
        for p in  line.split(",") {
            let fish = p.parse::<Lanternfish>().unwrap();
            fishes.push(fish);
        }
    }

    
    for _day in 0..80 {
        let mut fishes_new = Vec::new();

        for fish in fishes.iter_mut() {
            if fish.tick() {
                fishes_new.push(Lanternfish{t:8});
            }
        }
        fishes.append( &mut fishes_new);
    }

    println!("count {:?}", fishes.len());
    Ok(())
}


