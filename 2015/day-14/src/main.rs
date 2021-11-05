use regex::Regex;
use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;
use std::str::FromStr;

static RE: &str = r#"^(\w+)\scan\sfly\s(\d{1,3})\skm/s\sfor\s(\d{1,3})\sseconds,\sbut\sthen\smust\srest\sfor\s(\d{1,3})\sseconds.$"#;

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: (u32, u32),
    rest_sec: u32,
}

impl FromStr for Reindeer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE)?;
        let caps = re.captures(&s).unwrap();
        Ok(Reindeer {
            name: caps.get(1).unwrap().as_str().to_string(),
            speed: (caps.get(2).unwrap().as_str().parse()?, caps.get(3).unwrap().as_str().parse()?),
            rest_sec: caps.get(4).unwrap().as_str().parse()?,
        })
    }
}

fn distance(time: u32, reindeer: &Reindeer) -> u32 {
    let mut d = 0;
    let mut x = 0;
    while x <= time {
        if x + reindeer.speed.1 <= time {
            x += reindeer.speed.1;
            d += reindeer.speed.0*reindeer.speed.1;
        } else {
            d += reindeer.speed.0 / reindeer.speed.1 * (time - x);
            x += time;
        }

        if x + reindeer.rest_sec <= time {
            x += reindeer.rest_sec;
        } else {
            x = time + 1;
        }
    }
    d
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut reindeer: Vec<Reindeer> = Vec::new();
    for line in lines {
        let v = line?;
        if v.len() == 0 {
            continue;
        }
        let r: Reindeer = v.parse().unwrap();
        reindeer.push(r);
    }

    const TIME: u32 = 2503;
    let max = reindeer.iter().map(|x| {
        distance(TIME, &x)
    }).max();

    println!("distance: {:?}", max.unwrap());
    Ok(())
}
