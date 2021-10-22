use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;

fn calc(s: &str) -> usize {
    let mut sout = s.
        replace(r#"\""#, r#"""#).
        replace(r#"\\"#, r#"\"#).
        to_string();
    sout.remove(0);
    sout.remove(sout.len() - 1);

    let re_str = r#"(\\x[[:xdigit:]]{2})"#;
    let re = Regex::new(re_str).unwrap();
    for mat in re.find_iter(s) {
        let ms = mat.as_str();
        let ch = u32::from_str_radix(
            ms.trim_start_matches(r"\x"),
            16).unwrap() as u8 as char;
        sout = sout.replace(ms, ch.to_string().as_str());
    }
    sout.chars().count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    for line in lines {
        let v = line?;
        let vc = v.len();
        count += vc - calc(&v)
    }
    println!("count {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn calc_test() {
        assert_eq!(calc("\"\""), 0);
        assert_eq!(calc("\"abc\""), 3);
        assert_eq!(calc(r#""aaa\"aaa""#), 7);
        assert_eq!(calc(r#""\x27""#), 1);
        assert_eq!(calc(r#""""#), 0);
        assert_eq!(calc(r#""\"""#), 1);
        assert_eq!(calc(r#""czbggabkzo\"wsnw\"voklp\"s""#), 23);
        assert_eq!(calc(r#""vkfam\"yllr\"q\x92o\x4ebecnvhshhqe\\""#), 27);
    }
}
