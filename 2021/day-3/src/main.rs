use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, self};

fn bit_enabled(i: i32, num: usize) -> bool {
    i & (1 << num) != 0
}

fn calc_bits(selected: Option<&HashMap<usize, bool>>, v: &Vec<i32>, num: usize) -> (i32, i32) {
    let mut stat: (i32, i32) = (0, 0);
    for (i, &b) in v.iter().enumerate() {
        if let Some(s) = selected {
            if !*s.get(&i).unwrap() {
                continue;
            }
        }
        let bit = b & (1 << num) != 0;
        match bit {
            false => stat.0 += 1,
            true => stat.1 += 1,
        }
    }
    stat
}

fn select<F>(max_bit: usize, input: &Vec<i32>, selector: F) -> i32 where
    F: Fn(i32, i32, bool) -> bool {
    let mut selected: HashMap<usize, bool> = input.iter().enumerate().map(|(x, _)| {
        (x, true)
    }).into_iter().collect();

    let mut count_selected = input.len();
    let mut selected_item = 0;
    for num in (0..max_bit).rev() {
        let s = calc_bits(Some(&selected), &input, num);
        for i in 0..input.len() {
            if !*selected.get(&i).unwrap() {
                continue;
            }
            let is_enabled = bit_enabled(input[i], num);
            if selector(s.0, s.1, is_enabled) {
                selected_item = input[i];
                continue;
            }
            *selected.get_mut(&i).unwrap() = false;
            count_selected -= 1;
            if count_selected <= 1 {
                return selected_item;
            }
        }
    }
    selected_item
}

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

    for num in 0..max_bit {
        let s = calc_bits(None, &input, num);
        stat[num].1 += s.1;
        stat[num].0 += s.0;
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

    let oxygen = select(max_bit, &input, |s0, s1, is_set_bit| {
        (s0 > s1 && !is_set_bit) || (s1 >= s0 && is_set_bit)
    });
    let co2 = select(max_bit, &input, |s0, s1, is_set_bit| {
        (s0 <= s1 && !is_set_bit) || (s1 < s0 && is_set_bit)
    });
    println!("oxygen {:?} CO2 scrubber: {:?} life rating: {:?}", oxygen, co2, oxygen * co2);

    Ok(())
}
