use serde_json::{Value};
use std::fs::File;
use std::error::Error;
use std::io::Read;

fn scan(v: &Value) -> i64 {
    let mut sum = 0;
    if v.is_i64() || v.is_u64() {
        let v64 = v.as_i64().unwrap();
        sum += v64;
        println!("int: {:?}", v64);
    }

    if v.is_f64() || v.is_f64() {
        let f64 = v.as_f64().unwrap();
        sum += f64 as i64;
        println!("int: {:?}", f64);
    }

    if v.is_object() {
        for item in v.as_object() {
            for value in item.values() {
                sum += scan(&value);
            }
        }
    }

    if v.is_array() {
        for item in v.as_array() {
            for value in item {
                sum += scan(&value);
            }
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.json")?;
    let v: Value = serde_json::from_reader(file)?;
    let sum = scan(&v);
    println!("sum: {}", sum);
    Ok(())
}
