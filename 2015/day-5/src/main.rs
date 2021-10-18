use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn is_nice(s: &str) -> bool {
    if s.contains("ab") ||
        s.contains("cd") ||
        s.contains("pq") ||
        s.contains("xy") {
        return false;
    }
    let tmpl = "aeiou";

    if s.chars().into_iter().filter(|&x| {
        tmpl.contains(&x.to_string())
    }).count() < 3 {
        return false;
    }


    let mut prev: Option<char> = None;
    for ch in s.chars() {
        if let Some(c) = prev {
            if c == ch {
                return true;
            }
        }
        prev = Some(ch)
    };
    false
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let count =
        lines.map(|x| x.unwrap()).filter(|x| {
            is_nice(&x)
        }).count();
    println!("count: {}", count);
    Ok(())
}
