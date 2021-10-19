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

fn is_nice_part_2(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let mut frequency_pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..s.len() - 1 {
        let key: (char, char) = (chars[i], chars[i + 1]);
        let v = frequency_pairs.entry(key).or_insert(Vec::new());
        v.push(i);
    }
    let mut nice = false;
    for (_, pos) in &frequency_pairs {
        for k in pos {
            nice = pos.iter().any(|x| (*x as i32 - *k as i32) >= 2);
            if nice {
                break;
            }
        }
        if nice {
            break;
        }
    }
    if !nice {
        return false;
    }

    for i in 0..s.len() - 2 {
        let key: (char, char) = (chars[i], chars[i + 2]);
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    let mut count_part2 = 0;
    for line in lines {
        let value = line.unwrap();
        if is_nice(&value) {
            count += 1;
        }
        if is_nice_part_2(&value) {
            count_part2 += 1;
        }
    }
    println!("nice count(part 1 rules): {}", count);
    println!("nice count(part 2 rules): {}", count_part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::is_nice_part_2;

    #[test]
    fn is_nice_new_tests() {
        assert_eq!(is_nice_part_2("xyxy"), true);
        assert_eq!(is_nice_part_2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_part_2("xxyxx"), true);
        assert_eq!(is_nice_part_2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part_2("ieodomkazucvgmuy"), false);
    }
}