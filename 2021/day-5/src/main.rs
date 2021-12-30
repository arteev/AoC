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
        self.start.0 == self.end.0 
    }
    
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1 
    }

    fn is_diagonal(&self) -> bool {
        let x = (self.start.0 - self.end.0).abs();
        let y = (self.start.1 - self.end.1).abs();
        x>0 && y>0 && x==y
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

    fn draw_horizontal(&mut self,line: &Line) {
        for x in range(line.start.0,line.end.0) {
            let p = Point(x, line.start.1);
            let visit = self.v.entry(p).or_insert(0);
            *visit +=1;
           }

    }

    fn draw_vertical(&mut self,line: &Line) {
        for y in range(line.start.1,line.end.1) {
            let p = Point( line.start.0,y);
            let visit = self.v.entry(p).or_insert(0);
            *visit +=1;
           }
    }

    fn draw_diagonal(&mut self, line: &Line) {
        let y1 = line.start.1;
        let incr_y = {
            if y1>line.end.1 {-1} else {1}
        };
        let x1 = line.start.0;
        let incr_x = {
            if x1 > line.end.0 {-1} else {1}
        };

        for i in 0..=(line.start.0-line.end.0).abs() {
            let x = x1 + i*incr_x;
            let y = y1 + i*incr_y;
            let p = Point(x,y);
            let visit = self.v.entry(p).or_insert(0);
            *visit +=1;
        }

    }

    fn draw(&mut self,line: &Line, use_diag: bool) {
       if line.is_horizontal() {
           self.draw_horizontal(line);
       } else if line.is_vertical() {          
           self.draw_vertical(line);
       } else if use_diag && line.is_diagonal(){
           self.draw_diagonal(line);
        
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
    let mut board_diag = Board::new();

    for line in lines {
        let l = line.parse::<Line>()?;
        board.draw(&l,false);
        board_diag.draw(&l,true);
    }
    
    println!("count: {}", board.count_above2());
    println!("count_diag: {}", board_diag.count_above2());
    Ok(())
}

