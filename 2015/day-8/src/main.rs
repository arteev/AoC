use regex::Regex;
use std::io::{self, BufRead};
use std::fs::File;
use std::error::Error;

fn calc(s: &str) -> (usize, usize) {
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

    let mut s_encode = s.
        replace("\\", "\\\\").
        replace("\"", "\\\"");
    s_encode.insert_str(0, "\"");
    s_encode.push_str("\"");
    (sout.chars().count(), s_encode.chars().count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    let mut count_enc = 0;
    let mut count_all = 0;
    for line in lines {
        let v = line?;
        let vc = v.len();
        let (dec, enc) = calc(&v);
        count += vc - dec;
        count_all += vc;
        count_enc += enc;
    }
    count_enc = count_enc - count_all;
    println!("count {}", count);
    println!("count encode {}", count_enc);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn calc_test() {
        assert_eq!(calc("\"\""), (0, 6));
        assert_eq!(calc("\"abc\""), (3, 9));
        assert_eq!(calc(r#""aaa\"aaa""#), (7, 16));
        assert_eq!(calc(r#""\x27""#), (1, 11));
        assert_eq!(calc(r#""""#), (0, 6));
        assert_eq!(calc(r#""\"""#), (1, 10));
        assert_eq!(calc(r#""czbggabkzo\"wsnw\"voklp\"s""#), (23, 38));
        assert_eq!(calc(r#""vkfam\"yllr\"q\x92o\x4ebecnvhshhqe\\""#), (27, 50));
    }
}
