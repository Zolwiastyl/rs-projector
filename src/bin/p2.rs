use anyhow::Result;
use std::str::FromStr;

fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn main() {
    let lines = get_input()
        .lines()
        .flat_map(|line| str::parse(line))
        .filter(|line: &Line| line.is_horver())
        .collect::<Vec<Line>>();

    println!("{:?}", lines);
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(",");
        if result.is_none() {
            return Err(anyhow::anyhow!("Expected a comma"));
        }
        let (x, y) = result.unwrap();
        let x = str::parse(x)?;
        let y = str::parse(y)?;
        Ok(Point { x, y })
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let result = s.split_once(" -> ");
        if result.is_none() {
            return Err(anyhow::anyhow!("Expected a -> "));
        }
        let (p1, p2) = result.unwrap();
        let p1 = str::parse(p1)?;
        let p2 = str::parse(p2)?;
        Ok(Line { p1, p2 })
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }
    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
    fn is_horver(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}
