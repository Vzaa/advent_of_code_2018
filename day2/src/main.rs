use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let boxes: Vec<_> = rdr.lines().map(|l| l.unwrap()).collect();

    let mut twos = 0;
    let mut threes = 0;

    for b in boxes {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for c in b.chars() {
            let cnt = *hm.get(&c).unwrap_or(&0) + 1;
            hm.insert(c, cnt);
        }

        if hm.values().any(|&v| v == 2) {
            twos += 1;
        }

        if hm.values().any(|&v| v == 3) {
            threes += 1;
        }
    }

    let checksum = twos * threes;

    println!("{}", checksum);
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let boxes: Vec<_> = rdr.lines().map(|l| l.unwrap()).collect();

    let diff = |a: &str, b: &str| {
        a.chars()
            .zip(b.chars())
            .fold(0, |acc, (c1, c2)| if c1 != c2 { acc + 1 } else { acc })
    };

    let (b1, b2) = {
        let mut b1 = None;
        let mut b2 = None;

        for (idx, a) in boxes.iter().enumerate() {
            for b in boxes.iter().skip(idx + 1) {
                if diff(a, b) == 1 {
                    b1 = Some(a);
                    b2 = Some(b);
                    break;
                }
            }
        }

        // YOLO
        (b1.unwrap(), b2.unwrap())
    };

    let common: String = b1
        .chars()
        .zip(b2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect();

    println!("Boxes: {} {}", b1, b2);
    println!("Common: {}", common);
}

fn main() {
    p1();
    p2();
}
