use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Pos(i32, i32, i32, i32);

#[derive(Debug)]
enum ParsePosError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParsePosError {
    fn from(e: ParseIntError) -> Self {
        ParsePosError::Int(e)
    }
}

impl FromStr for Pos {
    type Err = ParsePosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');

        Ok(Pos(
            it.next().ok_or(ParsePosError::Empty)?.parse()?,
            it.next().ok_or(ParsePosError::Empty)?.parse()?,
            it.next().ok_or(ParsePosError::Empty)?.parse()?,
            it.next().ok_or(ParsePosError::Empty)?.parse()?,
        ))
    }
}

#[derive(Debug)]
struct Const {
    points: Vec<Pos>,
}

impl Const {
    pub fn new() -> Const {
        Const { points: vec![] }
    }

    fn dist_pos(&self, other: Pos) -> i32 {
        self.points.iter().map(|&p| m_dist(p, other)).min().unwrap()
    }

    fn dist_con(&self, other: &Const) -> i32 {
        other
            .points
            .iter()
            .map(|&o| self.dist_pos(o))
            .min()
            .unwrap()
    }
}

fn m_dist(a: Pos, b: Pos) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let ps: Vec<Pos> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let mut id_gen = 0..;
    let mut consts: HashMap<_, Const> = HashMap::new();

    for p in ps {
        let mut closest = consts.values_mut().min_by_key(|c| c.dist_pos(p));

        match closest {
            Some(ref mut c) if c.dist_pos(p) <= 3 => c.points.push(p),
            _ => {
                let mut new_c = Const::new();
                new_c.points.push(p);
                consts.insert(id_gen.next().unwrap(), new_c);
            }
        }
    }

    loop {
        let mut equil = true;
        let keys: Vec<_> = consts.keys().cloned().collect();

        for cur_key in &keys {
            let mut cur = consts.remove(cur_key).unwrap();

            let mut closest = consts.values_mut().min_by_key(|o| o.dist_con(&cur));

            match closest {
                Some(ref mut c) if c.dist_con(&cur) <= 3 => {
                    c.points.append(&mut cur.points);
                    equil = false;
                }
                _ => {
                    consts.insert(*cur_key, cur);
                }
            }
        }

        if equil {
            break;
        }
    }

    println!("{}", consts.len());
}
