fn look_and_say(s: &str) -> String {
    let mut nums: Vec<(u32, u32)> = Vec::new();

    let mut prev: Option<char> = None;
    let mut count = 0;
    for ch in s.chars() {
        match prev {
            Some(p) => {
                if ch == p {
                    count += 1;
                } else {
                    nums.push((count, p.to_digit(10).unwrap()));
                    prev = Some(ch);
                    count = 1;
                }
            }
            None => {
                prev = Some(ch);
                count = 1;
            }
        }
    }
    if prev.is_some() {
        nums.push((count, prev.unwrap().to_digit(10).unwrap()));
    }

    nums.iter().map(|(repeat, digit)| {
        format!("{}{}", repeat, digit)
    }).collect::<Vec<String>>().join("")
}

fn main() {
    for i in [40, 50] {
        let mut s = "1113122113".to_string();
        for _ in 0..i {
            s = look_and_say(&s);
        }
        println!("times:{}, len:{}", i, s.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::look_and_say;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
