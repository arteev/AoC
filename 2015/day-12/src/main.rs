use serde_json::{Value};
use std::fs::File;
use std::error::Error;
use std::io::Read;

fn scan(v: &Value, ignore: bool, ignore_skip_level: bool) -> (i64, bool) {
    let mut sum = 0;

    if v.is_string() && ignore && v.as_str().unwrap() == "red" && !ignore_skip_level {
        return (0, true);
    }

    if v.is_i64() || v.is_u64() {
        let v64 = v.as_i64().unwrap();
        sum += v64;
    }

    if v.is_f64() || v.is_f64() {
        let f64 = v.as_f64().unwrap();
        sum += f64 as i64;
    }


    if v.is_object() {
        for item in v.as_object() {
            for value in item.values() {
                let (sum_result, was_ignored) = scan(&value, ignore, false);
                sum += sum_result;
                if was_ignored {
                    return (0, false);
                }
            }
        }
    }

    if v.is_array() {
        for item in v.as_array() {
            for value in item {
                let (sum_result, was_ignored) = scan(&value, ignore, true);
                sum += sum_result;
                if was_ignored {
                    return (0, false);
                }
            }
        }
    }
    (sum, false)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.json")?;
    let v: Value = serde_json::from_reader(file)?;

    let (sum, _) = scan(&v, false, false);
    println!("sum: {}", sum);

    let (sum, _) = scan(&v, true, false);
    println!(r#"sum ignore "red": {}"#, sum);

    Ok(())
}
