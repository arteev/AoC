use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut prev: Option<i32> = None;
    let mut larger = 0;
    for line in lines {
        if let Ok(v) = line {
            let measure = v.parse::<i32>()?;
            if let Some(p) = prev {
                if p < measure {
                    larger += 1
                }
            };
            prev = Some(measure)
        }
    }
    println!("measurements that are larger: {}", larger);
    Ok(())
}
