use std::collections::HashMap;
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
            d += reindeer.speed.0 * reindeer.speed.1;
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

#[derive(Debug, PartialEq)]
struct Distance {
    distance: u32,
    points: u32,
}

impl Distance {
    fn new() -> Distance {
        Distance {
            distance: 0,
            points: 0,
        }
    }
}

fn race(time: u32, reindeer: &Vec<Reindeer>) -> HashMap<String, Distance> {
    let mut result = HashMap::new();


    let mut distances: HashMap<String, (Distance, (u32, u32))> = HashMap::new();

    for sec in 1..=time {
        // calc distances
        for r in reindeer {
            let m = distances.get_mut(&r.name);
            let dist = match m {
                Some(t) => t,
                None => {
                    distances.insert(
                        r.name.to_string(), (Distance::new(), (0, 0)),
                    );
                    distances.get_mut(&r.name).unwrap()
                }
            };

            if dist.1.1 > 0 {
                dist.1.1 -= 1;
                continue;
            } else if dist.1.0 >= r.speed.1 {
                dist.1.1 = r.rest_sec - 1;
                dist.1.0 = 0;
                continue;
            }
            dist.1.0 += 1;

            dist.0.distance += r.speed.0;
        }

        let max = distances.values().max_by(|x, y| {
            x.0.distance.cmp(&y.0.distance)
        }).unwrap().0.distance;

        for (_, item) in distances.iter_mut() {
            if item.0.distance == max {
                item.0.points += 1
            }
        }
    }

    for (name, d) in distances {
        result.insert(name, d.0);
    }
    result
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

    let points = race(TIME, &reindeer);

    println!("distance: {:?}", max.unwrap());
    let winner_by_points = points.iter().max_by(|x, y| {
        x.1.points.cmp(&y.1.points)
    }).unwrap();
    println!("winner by points: {:#?}", &winner_by_points);
    Ok(())
}
