use std::io::{self, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Dimensions {
    width: i32,
    height: i32,
    length: i32,
}

impl Dimensions {
    fn new(s: &str) -> Dimensions {
        let r: Vec<i32> = s.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
        Dimensions {
            length: r[0],
            width: r[1],
            height: r[2],
        }
    }

    fn area(&self) -> i32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn extra(&self) -> i32 {
        *[self.length * self.width, self.width * self.height, self.height * self.length].
            iter().
            min().
            unwrap()
    }

    fn len_ribbon(&self) -> i32 {
        let mut v = [self.length, self.width, self.height];
        v.sort();
        self.length * self.width * self.height +
            v.iter().take(2).fold(0, |s, item| s + item * 2)
    }
}

fn dim_from_file(file_name: &str) -> io::Result<Vec<Dimensions>> {
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();
    let mut v = Vec::new();
    for line in lines {
        let d = Dimensions::new(&line?);
        v.push(d);
    }
    Ok(v)
}

fn main() -> io::Result<()> {
    if let Ok(dims) = dim_from_file("input.txt") {
        let measure = dims.iter().fold((0, 0), |s, d| {
            (s.0 + d.area() + d.extra(), s.1 + d.len_ribbon())
        });
        println!("square: {}", measure.0);
        println!("ribbon: {}", measure.1);
    }
    Ok(())
}