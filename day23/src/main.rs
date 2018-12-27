use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

type Pos = (i32, i32, i32);

fn m_dist(a: Pos, b: Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

#[derive(Debug)]
struct NanoBot {
    p: Pos,
    r: i32,
}

#[derive(Debug)]
enum ParseError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

impl FromStr for NanoBot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean: String = s
            .matches(|c: char| c.is_numeric() || c == ' ' || c == '-' || c == ',')
            .collect::<String>()
            .replace(",", " ");

        let mut it = clean.split_whitespace();
        let x: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;
        let y: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;
        let z: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;
        let r: i32 = it.next().ok_or(ParseError::Empty)?.parse()?;

        Ok(NanoBot { p: (x, y, z), r })
    }
}

impl NanoBot {
    fn in_range(&self, p: Pos) -> bool {
        self.r >= m_dist(self.p, p)
    }
}

#[derive(Debug)]
struct AABB {
    min: Pos,
    max: Pos,
}

impl AABB {
    fn intersect(&self, b: &NanoBot) -> bool {
        let x = cmp::max(self.min.0, cmp::min(b.p.0, self.max.0));
        let y = cmp::max(self.min.1, cmp::min(b.p.1, self.max.1));
        let z = cmp::max(self.min.2, cmp::min(b.p.2, self.max.2));

        let p = (x, y, z);
        b.in_range(p)
    }

    fn intersect_all(&self, bots: &[NanoBot]) -> usize {
        bots.iter().filter(|b| self.intersect(b)).count()
    }

    fn is_size_one(&self) -> bool {
        self.min == self.max
    }

    fn div(self) -> Vec<AABB> {
        let div_x = (self.max.0 - self.min.0) / 2 + self.min.0;
        let div_y = (self.max.1 - self.min.1) / 2 + self.min.1;
        let div_z = (self.max.2 - self.min.2) / 2 + self.min.2;

        let mut out = vec![];

        for x in 0..=1 {
            for y in 0..=1 {
                for z in 0..=1 {
                    let (min_x, max_x) = if x == 0 { (self.min.0, div_x) } else { (div_x + 1, self.max.0) };
                    let (min_y, max_y) = if y == 0 { (self.min.1, div_y) } else { (div_y + 1, self.max.1) };
                    let (min_z, max_z) = if z == 0 { (self.min.2, div_z) } else { (div_z + 1, self.max.2) };

                    let mbox = AABB {
                        min: (min_x, min_y, min_z),
                        max: (max_x, max_y, max_z),
                    };

                    out.push(mbox);
                }
            }
        }

        out
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let bots: Vec<NanoBot> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let max_r = bots.iter().max_by_key(|b| b.r).unwrap();
    let cnt = bots.iter().filter(|b| max_r.in_range(b.p)).count();

    println!("Part 1: {}", cnt);

    let min_x = bots.iter().map(|b| b.p.0 - b.r).min().unwrap();
    let min_y = bots.iter().map(|b| b.p.1 - b.r).min().unwrap();
    let min_z = bots.iter().map(|b| b.p.2 - b.r).min().unwrap();

    let max_x = bots.iter().map(|b| b.p.0 + b.r).max().unwrap();
    let max_y = bots.iter().map(|b| b.p.1 + b.r).max().unwrap();
    let max_z = bots.iter().map(|b| b.p.2 + b.r).max().unwrap();

    let mbox = AABB {
        min: (min_x, min_y, min_z),
        max: (max_x, max_y, max_z),
    };

    let mut frontier = {
        let cnt = mbox.intersect_all(&bots);
        vec![(mbox, cnt)]
    };

    loop {
        let idx = frontier.iter().enumerate().max_by_key(|(_, i)| i.1).unwrap().0;

        let (most, _) = frontier.remove(idx);

        if most.is_size_one() {
            // Didn't need to check for the closest point for my input ¯\_(ツ)_/¯
            println!("Point is {:?}", most.min);
            println!("Part 2: {}", m_dist((0, 0, 0), most.min));
            break;
        }

        for child in most.div() {
            let cnt = child.intersect_all(&bots);
            frontier.push((child, cnt));
        }
    }
}
