use std::collections::HashMap;
use regex::Regex;
use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;
use std::str::FromStr;
use itertools::*;
use std::env;

#[derive(Debug, PartialEq)]
enum Action {
    Lose,
    Gain,
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lose" => Ok(Action::Lose),
            "gain" => Ok(Action::Gain),
            _ => Err("unknown action"),
        }
    }
}


#[derive(Debug, PartialEq)]
struct Happy {
    action: Action,
    person: String,
    next: String,
    happiness: usize,
}


impl FromStr for Happy {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(REHAPPY)?;
        let caps = re.captures(&s).unwrap();
        Ok(Happy {
            action: caps.get(2).unwrap().as_str().parse()?,
            person: caps.get(1).unwrap().as_str().to_string(),
            next: caps.get(4).unwrap().as_str().to_string(),
            happiness: caps.get(3).unwrap().as_str().parse()?,
        })
    }
}

static REHAPPY: &str = r#"^(\w+)\swould\s(lose|gain)\s(\d{1,3})\shappiness\sunits\sby\ssitting\snext\sto\s(\w+).$"#;

fn calc_happiness(table: &Vec<&String>, happiness: &Vec<Happy>) -> i32 {
    let happy_map: HashMap<(String, String), usize> = happiness.iter().enumerate().map(|(index, x)| {
        ((x.person.to_string(), x.next.to_string()), index)
    }).into_iter().collect();

    let len = table.len();
    let mut sum = 0;
    for i in 0..table.len() {
        let current = i;
        let next = {
            if i == len - 1 { 0 } else { i + 1 }
        };
        let prev = {
            if i == 0 { len - 1 } else { i - 1 }
        };

        let key = (
            table.get(current).unwrap().to_string(),
            table.get(next).unwrap().to_string(),
        );
        let happiness_value = happy_map.get(&key);
        if let Some(&h) = happiness_value {
            if let Some(h) = happiness.get(h) {
                sum += match h.action {
                    Action::Lose => -1 * (h.happiness as i32),
                    Action::Gain => h.happiness as i32,
                }
            }
        }

        let key = (
            table.get(current).unwrap().to_string(),
            table.get(prev).unwrap().to_string(),
        );
        let happiness_value = happy_map.get(&key);
        if let Some(&h) = happiness_value {
            if let Some(h) = happiness.get(h) {
                sum += match h.action {
                    Action::Lose => -1 * (h.happiness as i32),
                    Action::Gain => h.happiness as i32,
                }
            }
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let me = args.nth(1).unwrap_or("".to_string()) == "me";


    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut happiness: Vec<Happy> = Vec::new();
    let mut persons: Vec<String> = Vec::new();
    for line in lines {
        let v = line?;
        if v.len() == 0 {
            continue;
        }
        let happy: Happy = v.parse().unwrap();

        if !persons.contains(&happy.person) {
            persons.push(happy.person.clone());
        }

        happiness.push(happy);
    }

    if me {
        persons.push("Me".to_string());
        for p in &persons {
            let happy: Happy = format!("Me would gain 0 happiness units by sitting next to {}.", p).parse().unwrap();
            happiness.push(happy);
        }
    }

    let perms = persons.iter().permutations(persons.len()).collect::<Vec<Vec<&String>>>();
    let mut max_happiness = 0;
    for item in perms {
        let happy = calc_happiness(&item, &happiness);
        if happy > max_happiness {
            max_happiness = happy;
        }
    }

    println!("max happiness: {}", max_happiness);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_from_str() {
        let happy: Happy = "Bob would lose 14 happiness units by sitting next to Alice.".parse().unwrap();
        assert_eq!(happy, Happy {
            action: Action::Lose,
            person: "Bob".to_string(),
            next: "Alice".to_string(),
            happiness: 14,
        });


        let happy: Happy = "Alice would gain 51 happiness units by sitting next to Carol.".parse().unwrap();
        assert_eq!(happy, Happy {
            action: Action::Gain,
            person: "Alice".to_string(),
            next: "Carol".to_string(),
            happiness: 51,
        });
    }
}