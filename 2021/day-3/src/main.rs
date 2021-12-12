use std::error::Error;
use std::fs::File;
use std::io::{BufRead, self};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut max_bit = 0;
    let mut input = Vec::new();
    for line in lines {
        let v = line?;
        if v.len() > max_bit {
            max_bit = v.len();
        }
        let b = i32::from_str_radix(&v, 2)?;
        input.push(b);
    }

    let mut stat: Vec<(i32, i32)> = vec![(0, 0); max_bit as usize];

    for b in input {
        for num in 0..max_bit {
            let bit = b & (1 << num) != 0;
            match bit {
                false => stat[num].0 += 1,
                true => stat[num].1 += 1,
            }
        }
    }
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    for num in 0..max_bit {
        if stat[num].1 >= stat[num].0 {
            gamma = gamma | 1 << num;
            epsilon = epsilon & !(1 << num);
        } else {
            gamma = gamma & !(1 << num);
            epsilon = epsilon | 1 << num;
        }
    }
    println!("gamma:{} epsilon:{} power:{}", gamma, epsilon, gamma * epsilon);
    Ok(())
}
