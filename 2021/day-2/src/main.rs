use std::error::Error;
use std::fs::File;
use std::io::{BufRead, self};
use std::str::FromStr;

enum Movement {
    FORWARD,
    DOWN,
    UP,
}

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Movement::FORWARD),
            "down" => Ok(Movement::DOWN),
            "up" => Ok(Movement::UP),
            _ => Err("invalid movement")
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut depth = 0i32;
    let mut distanse = 0i32;
    for line in lines {
        let v = line?;
        let values = v.splitn(2, " ").collect::<Vec<&str>>();
        let movement: Movement = values[0].parse().unwrap();
        let steps: i32 = values[1].parse().unwrap();
        match movement {
            Movement::FORWARD => distanse += steps,
            _ => depth += match movement {
                Movement::DOWN => steps,
                Movement::UP => -1 * steps,
                _ => panic!("invalid movements")
            }
        };
    }

    println!("distance: {}", distanse);
    println!("depth: {}", depth);
    println!("multiply: {}", depth * distanse);

    Ok(())
}
