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
        if self.t < 0 {
            self.t = 6;
            return true;
        }
        false
    }
}

fn main() -> Result<(),Box<dyn Error>>{

    let args: Vec<String> = std::env::args().collect();

    let mut days = 80;
    if args.len()>1 {
        days = args[1].parse().unwrap();
    }


    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x|x.unwrap());

    let size = 2usize.pow(days/16);
    let mut fishes = Vec::with_capacity(size);

    for line in lines {
        for p in  line.split(",") {
            let fish = p.parse::<Lanternfish>().unwrap();
            fishes.push(fish);
        }
    }

    
    for day in 0..days {
        let mut count_new = 0;

        for fish in fishes.iter_mut() {
            if fish.tick() {
                count_new += 1;
            }
        }
        for i in 0..count_new {
            fishes.push(Lanternfish{t:8});
        }
        println!("days:{}, count:{}", day+1, fishes.len());
    }

    println!("count {:?}", fishes.len());
    Ok(())
}


