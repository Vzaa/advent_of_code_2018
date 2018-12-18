use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
enum C {
    Open,
    Tree,
    Lumberyard,
}

impl C {
    fn from_char(c: char) -> Result<C, ()> {
        let out = match c {
            '.' => C::Open,
            '|' => C::Tree,
            '#' => C::Lumberyard,
            _ => return Err(()),
        };

        Ok(out)
    }
}

fn neieghbors(area: &HashMap<Pos, C>, p: Pos) -> Vec<C> {
    let nlist = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0), // sides
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1), // corners
    ];

    nlist
        .iter()
        .map(|&s| (s.0 + p.0, s.1 + p.1))
        .map(|n| area.get(&n).map(|a| *a).unwrap_or(C::Open))
        .collect()
}

#[allow(dead_code)]
fn draw_map(area: &HashMap<Pos, C>) {
    'outer: for y in 0.. {
        for x in 0.. {
            if let Some(c) = area.get(&(x, y)) {
                let x = match c {
                    C::Open => '.',
                    C::Tree => '|',
                    C::Lumberyard => '#',
                };
                print!("{}", x);
            } else {
                if x == 0 {
                    break 'outer;
                } else {
                    break;
                }
            };
        }
        println!("");
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut area = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            area.insert((x as i32, y as i32), C::from_char(c).unwrap());
        }
    }

    let mut past = HashMap::new();
    let mut cycle_start = 0;
    let mut cycle = 0;

    let target = 1000000000;

    for t in 1.. {
        let mut area2 = HashMap::new();

        for (k, v) in area.iter() {
            let n = neieghbors(&area, *k);

            let n = match v {
                C::Open => {
                    let c = n.iter().filter(|&&a| a == C::Tree).count();
                    if c >= 3 {
                        C::Tree
                    } else {
                        *v
                    }
                }
                C::Tree => {
                    let c = n.iter().filter(|&&a| a == C::Lumberyard).count();
                    if c >= 3 {
                        C::Lumberyard
                    } else {
                        *v
                    }
                }
                C::Lumberyard => {
                    let l = n.iter().any(|&a| a == C::Lumberyard);
                    let t = n.iter().any(|&a| a == C::Tree);
                    if l && t {
                        *v
                    } else {
                        C::Open
                    }
                }
            };

            area2.insert(*k, n);
        }

        area = area2;

        // Amazing stuff here
        let mut keys: Vec<_> = area.keys().collect();
        keys.sort();
        let sorted: Vec<_> = keys.iter().map(|k| area[&k]).collect();

        if let Some(past_turn) = past.get(&sorted) {
            cycle = t - past_turn;
            cycle_start = *past_turn;
            break;
        }

        past.insert(sorted, t);
    }

    // Part 1
    {
        let (k, _) = past.iter().find(|(_, v)| **v == 10).unwrap();
        let t = k.iter().filter(|&&a| a == C::Tree).count();
        let l = k.iter().filter(|&&a| a == C::Lumberyard).count();
        println!("{} * {} = {}", t, l, t * l);
    }

    // Part 2
    {
        let (k, _) = past
            .iter()
            .find(|(_, v)| **v == ((target - cycle_start) % cycle) + cycle_start)
            .unwrap();
        let t = k.iter().filter(|&&a| a == C::Tree).count();
        let l = k.iter().filter(|&&a| a == C::Lumberyard).count();
        println!("{} * {} = {}", t, l, t * l);
    }
}
