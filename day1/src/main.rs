use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn p1() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let vals_iter = rdr.lines().map(|l| l.unwrap().parse::<i32>().unwrap());
    let s: i32 = vals_iter.sum();

    println!("{:?}", s);
}

fn p2() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let vals: Vec<i32> = rdr.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let sum_iter = vals.iter().cycle().scan(0, |acc, x| { *acc += x; Some(*acc) });
    let mut hs = HashSet::new();
    hs.insert(0);
    let non_uniq = sum_iter.skip_while(|s| hs.insert(*s)).next().unwrap();

    println!("{}", non_uniq);
}

fn main() {
    println!("Part 1");
    p1();
    println!("Part 2");
    p2();
}
