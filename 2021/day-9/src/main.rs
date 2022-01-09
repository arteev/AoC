use std::error::Error;
use std::fs::File;
use std::io::{BufRead,self};

fn is_low(x: usize, y:usize, v:&Vec<Vec<u8>>) -> bool {
    let value = v[y][x];
    if let Some(&vx) = v[y].get(x+1) {
        if vx <= value {
            return false
        }
    }
    if let Some(&vx) = v[y].get(x-1) {
        if vx <= value {
            return false
        }
    }
    if let Some(vy) = v.get(y+1) {
        if vy[x] <= value {
            return false
        }
    }
    if let Some(vy) = v.get(y-1) {
        if vy[x] <= value {
            return false
        }
    }
    true
}

fn main() -> Result<(),Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x|x.unwrap());
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut x_max = 0;
    for line in lines {
        let x = line.chars().map(|c| {
            c.to_string().parse::<u8>().unwrap()
        }).collect::<Vec<u8>>();

        if x.len() > x_max  {
            x_max = x.len();
        }
        v.push(x);
    }
   
    let mut points:u32 = 0;
    for x in 0..x_max {
        for y in 0..v.len() {
            let low = is_low(x,y,&v);
            if low {
                points += v[y][x] as u32 +1;
            }
        }
    }
    println!("points: {}", points);

    Ok(())    
}
