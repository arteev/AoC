use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};


fn solve(v: &Vec<i32>) -> i32 {
    let mut prev: Option<i32> = None;
    let mut larger = 0;
    for measure in v {
        if let Some(p) = prev {
            if p < *measure {
                larger += 1
            }
        };
        prev = Some(*measure);
    }
    larger
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut measurements = Vec::new();
    for line in lines {
        if let Ok(v) = line {
            let measure = v.parse::<i32>()?;
            measurements.push(measure);
        }
    }
    let larger = solve(&measurements);
    println!("measurements that are larger: {}", larger);

    let mut groups = Vec::new();
    for i in 0..measurements.len() - 2 {
        let mut sum = 0;
        for j in 0..3 {
            sum += measurements[i + j];
        }
        groups.push(sum);
    }
    let larger = solve(&groups);
    println!("measurements (sliding window) that are larger: {}", larger);
    Ok(())
}
