use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);

const SRC: Pos = (500, 0);

fn down(p: Pos) -> Pos {
    (p.0, p.1 + 1)
}

fn left(p: Pos) -> Pos {
    (p.0 - 1, p.1)
}

fn right(p: Pos) -> Pos {
    (p.0 + 1, p.1)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Water {
    Still,
    Stream,
}

#[derive(Debug)]
enum Outcome {
    Pour,
    Stay,
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[allow(dead_code)]
fn draw_map(clays: &HashSet<Pos>, water: &HashMap<Pos, Water>) {
    let (max_x, max_y) = (
        clays
            .iter()
            .map(|p| p.0)
            .chain(water.keys().map(|p| p.0))
            .max()
            .unwrap(),
        clays
            .iter()
            .map(|p| p.1)
            .chain(water.keys().map(|p| p.1))
            .max()
            .unwrap(),
    );

    let (min_x, min_y) = (
        clays
            .iter()
            .map(|p| p.0)
            .chain(water.keys().map(|p| p.0))
            .min()
            .unwrap(),
        clays
            .iter()
            .map(|p| p.1)
            .chain(water.keys().map(|p| p.1))
            .min()
            .unwrap(),
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(Water::Still) = water.get(&(x, y)) {
                print!("~");
            } else if let Some(Water::Stream) = water.get(&(x, y)) {
                print!("|");
            } else if clays.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn go_dir(
    clays: &HashSet<Pos>,
    water: &HashMap<Pos, Water>,
    p: Pos,
    d: Dir,
) -> (Outcome, Vec<Pos>) {
    let mut cur = p;
    let mut below = down(p);

    let mut points = vec![];

    loop {
        if !clays.contains(&below) && water.get(&below) != Some(&Water::Still) {
            points.push(cur);
            return (Outcome::Pour, points);
        } else if clays.contains(&cur) {
            // Should be still (if both sides hit)
            return (Outcome::Stay, points);
        }

        points.push(cur);

        match d {
            Dir::Left => {
                cur = left(cur);
                below = left(below);
            }
            Dir::Right => {
                cur = right(cur);
                below = right(below);
            }
        }
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());

    let mut clays = HashSet::new();
    let mut water = HashMap::new();
    water.insert(SRC, Water::Stream);

    for line in rdr.lines() {
        let line = line.unwrap();
        let clean: String = line
            .matches(|c: char| c.is_numeric() || c == ' ' || c == '.')
            .collect::<String>()
            .replace(".", " ");

        let mut it = clean.split_whitespace();

        let (p, from, to): (i32, i32, i32) = (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        );

        if line.starts_with('x') {
            let x = p;
            for y in from..=to {
                clays.insert((x, y));
            }
        } else if line.starts_with('y') {
            let y = p;
            for x in from..=to {
                clays.insert((x, y));
            }
        } else {
            panic!("nope");
        }
    }

    let max_y = clays.iter().map(|p| p.1).max().unwrap();
    let min_y = clays.iter().map(|p| p.1).min().unwrap();

    // Needs termination logic but got the star already :P
    for _ in 0..1000 {
        //loop {
        let streams: Vec<_> = water
            .iter()
            .filter(|(_, w)| **w == Water::Stream)
            .map(|(k, _)| *k)
            .collect();

        for s in &streams {
            // Find where streams hit below
            for y in s.1..max_y {
                let point = (s.0, y);
                let below = down(point);

                if water.get(&below) == Some(&Water::Stream) {
                    // Stream already goes down
                    break;
                } else if clays.contains(&below) || water.get(&below) == Some(&Water::Still) {
                    let (res_left, left) = go_dir(&clays, &water, point, Dir::Left);
                    let (res_right, right) = go_dir(&clays, &water, point, Dir::Right);

                    match (res_left, res_right) {
                        // Stay
                        (Outcome::Stay, Outcome::Stay) => {
                            for p in left.iter().chain(right.iter()) {
                                water.insert(*p, Water::Still);
                            }
                        }
                        // Pour
                        _ => {
                            for p in left.iter().chain(right.iter()) {
                                water.insert(*p, Water::Stream);
                            }
                        }
                    }
                    break;
                } else {
                    water.insert(below, Water::Stream);
                }
            }
        }
    }

    let part1 = water.keys().filter(|&k| k.1 >= min_y).count();
    let part2 = water
        .iter()
        .filter(|(k, v)| k.1 >= min_y && **v == Water::Still)
        .count();

    println!("Part 1 {}", part1);
    println!("Part 2 {}", part2);
}
