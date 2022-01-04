use std::error::Error;
use std::fs::File;
use std::io::{BufRead,self};
use std::collections::HashMap;

fn main() -> Result<(),Box<dyn Error>>{

    let args: Vec<String> = std::env::args().collect();

    let mut days = 80;
    if args.len()>1 {
        days = args[1].parse().unwrap();
    }

    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x|x.unwrap());


    let mut fish_map:HashMap<i32,u64> = HashMap::new(); 

    for line in lines {
        for p in  line.split(",") {
            let f: i32 = p.parse().unwrap();
            let fish = fish_map.entry(f).or_insert(0);
            *fish += 1;
        }
    }

    
    for _day in 0..days {
        let mut updated:HashMap<i32,u64> = HashMap::new();
        for (fish,count) in &fish_map {
            if *fish ==0 {
                *updated.entry(6).or_default() += count;
                *updated.entry(8).or_default() += count;
            } else {
                *updated.entry(fish-1).or_default() += count;
            }
        }
        fish_map = updated;
    }

    let count = fish_map.values().cloned().collect::<Vec<u64>>().iter().sum::<u64>();
    println!("count {:?}",count);
    Ok(())
}


