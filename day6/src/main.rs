use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum ParsePointError {
    Int(ParseIntError),
    Empty,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl From<ParseIntError> for ParsePointError {
    fn from(e: ParseIntError) -> Self {
        ParsePointError::Int(e)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(|x| x.trim());
        let x = it.next().ok_or(ParsePointError::Empty)?.parse()?;
        let y = it.next().ok_or(ParsePointError::Empty)?.parse()?;
        Ok(Point { x, y })
    }
}

fn m_dist(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let points: Vec<Point> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let (min_x, min_y) = (
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(),
    );

    let (max_x, max_y) = (
        points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).max().unwrap(),
    );

    // Part 1
    {
        let mut map_a = vec![];
        let mut map_b = vec![];

        // just yolo a size
        for y in (min_y - 10)..(max_y + 10) {
            for x in (min_x - 10)..(max_x + 10) {
                let p = Point { x, y };
                let mut dists: Vec<_> = points
                    .iter()
                    .map(|p2| m_dist(&p, &p2))
                    .enumerate()
                    .collect();
                dists.sort_by_key(|d| d.1);

                if dists[0].1 != dists[1].1 {
                    let closest = dists[0].0;
                    if (x > min_x && x < max_x) && (y > min_y && y < max_y) {
                        map_a.push(closest);
                    }
                    map_b.push(closest);
                }
            }
        }

        let mut results_a = HashMap::new();
        let mut results_b = HashMap::new();

        for &id in map_a.iter() {
            let v = results_a.entry(id).or_insert(0);
            *v += 1;
        }

        for &id in map_b.iter() {
            let v = results_b.entry(id).or_insert(0);
            *v += 1;
        }

        let max_one = results_a
            .iter()
            .filter(|(k, v)| results_b[k] == **v)
            .max_by_key(|a| a.1)
            .unwrap();

        println!("{:?}", max_one);
    }

    // Part 2 (Not a correct solution but worked for the given input)
    {
        let mut map = vec![];

        for y in (min_y - 10)..(max_y + 10) {
            for x in (min_x - 10)..(max_x + 10) {
                let p = Point { x, y };
                let sum = points
                    .iter()
                    .map(|p2| m_dist(&p, &p2))
                    .sum::<i32>();
                map.push(sum);
            }
        }

        let cnt = map.iter().filter(|&&d| d < 10000).count();
        println!("{}", cnt);
    }
}
