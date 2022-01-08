use std::error::Error;
use std::fs::File;
use std::io::{BufRead,self};

fn calculate(pos: u32, v: &Vec<u32>) -> u32 {
    v.iter().fold(0, |s, &x| {
        s+(pos as i32 - x as i32).abs() as u32
    })
}

fn calculate_part2(pos: u32, v: &Vec<u32>) -> u32 {
    v.iter().fold(0, |s, &x| {
        s+{
            let steps=(pos as i32 - x as i32).abs() as u32;
            if steps>1 {
                (1+steps)*steps /2 
            } else {
                steps
            }
        }
    })

}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x|x.unwrap());
    
    let mut positions:Vec<u32> = Vec::new(); 
    for line in lines {
        for p in line.split(",") {
            let c: u32 = p.parse().unwrap();
            positions.push(c);
        }
    }
    let max = positions.iter().max().unwrap();

    let mut m_pos : Option<u32> = None;
    let mut fuel: Option<u32> = None;

    for i in 0..=*max {
        let c = calculate_part2(i, &positions);
        if fuel.is_none() || fuel.unwrap() > c {
            m_pos = Some(i);
            fuel=Some(c);
        }
    }
    println!("fuel: {}, pos: {}", fuel.unwrap(), m_pos.unwrap());

    Ok(())
}
