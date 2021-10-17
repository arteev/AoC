use md5;
use std::fmt::format;
use std::env;

static SECRET: &str = "bgvyzdsv";

fn help() {
    println!("usage:
     main <number>
        number - start with at least <number> zeroes");
}


fn main() {
    let mut leading_zeros_count = 6;

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            leading_zeros_count = args[1].parse().unwrap();
        }
        _ => {
            help();
            return;
        }
    }

    let leading_zeros = "0".repeat(leading_zeros_count);
    for n in 1.. {
        let src = format!("{}{}", SECRET, n);
        let digest = md5::compute(src);
        let hex = format!("{:x}", digest);
        if hex.starts_with(&leading_zeros) {
            println!("hex: {}, number: {}", hex, n);
            return;
        }
    }
}