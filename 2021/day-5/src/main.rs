use std::error::Error;
use std::fs::File;
use std::io::{BufRead,self};
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug,Hash,PartialEq, Eq)]
struct Point(i32,i32);

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let coords: Vec<&str> = s.split(",")
            .map(|x|x.trim()) 
            .collect();
        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;
        Ok(Point(x,y))
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0 //&& self.start.1!=self.end.1
    }
    
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1 //&& self.start.1==self.end.1
    }
}

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self,Self::Err>{
        let points: Vec<&str> = s.split("->").collect();
        let start = points[0].parse::<Point>()?;
        let end = points[1].parse::<Point>()?;
        Ok(Line{start,end})
    }
}

struct Board {
    v: HashMap<Point,u32>,
}

fn range(a:i32,b:i32) -> RangeInclusive<i32> { 
    if a>=b {
        b..=a
    } else {
        a..=b
    }
}

impl Board {
    fn new() -> Board {
        Board{
            v: HashMap::new(),
        }
    }

    fn draw(&mut self,line: &Line) {
       if line.is_horizontal() {
           for x in range(line.start.0,line.end.0) {
            let p = Point(x, line.start.1);
            let visit = self.v.entry(p).or_insert(0);
            *visit +=1;
           }
        } else if line.is_vertical() {
          for y in range(line.start.1,line.end.1) {
            let p = Point( line.start.0,y);
            let visit = self.v.entry(p).or_insert(0);
            *visit +=1;
           }

        } 
    }

    fn count_above2(&self) -> usize {
       self.v.iter().filter(|&(_k,v)|{
            *v>=2   
       }).count()
    }
}

fn main() -> Result<(),Box<dyn Error>>{
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines().map(|x|x.unwrap());

    let mut board = Board::new();

    for line in lines {
        let l = line.parse::<Line>()?;
        board.draw(&l);
    }
    
    println!("count: {}", board.count_above2());
    Ok(())
}

