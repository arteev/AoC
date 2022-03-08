use std::error::Error;
use std::fs::File;
use std::io::{BufRead, self};

fn is_basin(x: usize, y: usize, v: &mut Vec<Vec<u8>>) -> u32 {
    let value = v[y][x];
    if value == 9 {
        return 0;
    }
    let mut result = 1;
    v[y][x] = 9;

    if let Some(_) = v[y].get(x + 1) {
        result = result + is_basin(x + 1, y, v);
    }
    if let Some(_) = v[y].get(x - 1) {
        result = result + is_basin(x - 1, y, v);
    }
    if let Some(_) = v.get(y + 1) {
        result = result + is_basin(x, y + 1, v);
    }
    if let Some(_) = v.get(y - 1) {
        result = result + is_basin(x, y - 1, v);
    }
    return result;
}

fn is_low(x: usize, y: usize, v: &Vec<Vec<u8>>) -> bool {
    let value = v[y][x];
    if let Some(&vx) = v[y].get(x + 1) {
        if vx <= value {
            return false;
        }
    }
    if let Some(&vx) = v[y].get(x - 1) {
        if vx <= value {
            return false;
        }
    }
    if let Some(vy) = v.get(y + 1) {
        if vy[x] <= value {
            return false;
        }
    }
    if let Some(vy) = v.get(y - 1) {
        if vy[x] <= value {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut x_max = 0;
    for line in lines {
        let x = line.chars().map(|c| {
            c.to_string().parse::<u8>().unwrap()
        }).collect::<Vec<u8>>();

        if x.len() > x_max {
            x_max = x.len();
        }
        v.push(x);
    }

    let mut points: u32 = 0;
    for x in 0..x_max {
        for y in 0..v.len() {
            let low = is_low(x, y, &v);
            if low {
                points += v[y][x] as u32 + 1;
            }
        }
    }
    println!("points: {}", points);

    let mut basins: Vec<u32> = Vec::new();
    for x in 0..x_max {
        for y in 0..v.len() {
            let size = is_basin(x, y, &mut v);
            if size > 0 {
                basins.push(size);
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));
    let top_basins: u32 = basins.iter().take(3).product();
    println!("size basins: {}", top_basins);

    Ok(())
}
