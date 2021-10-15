use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut buffer = Vec::new();
    let mut pos = 0;
    f.read_to_end(&mut buffer)?;
    let floor = buffer.iter().enumerate().fold(0, |f, (i, value)| {
        let current = f + if *value == ('(' as u8) { 1 } else { -1 };
        if current == -1 && pos == 0 {
            pos = i + 1;
        }
        current
    });
    print!("Part 1: floor - {}\n", floor);
    print!("Part 2: position of the character that causes Santa to first enter the basement - {}\n", pos);
    Ok(())
}