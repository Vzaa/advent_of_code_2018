use std::fs;

#[derive(Debug)]
struct Node {
    dat: usize,
    next: usize,
    prev: usize,
}

fn nth_fwd(circle: &[Node], id: usize, n: usize) -> usize {
    let mut cur = id;
    for _ in 0..n {
        cur = circle[cur].next;
    }
    cur
}

fn nth_back(circle: &[Node], id: usize, n: usize) -> usize {
    let mut cur = id;
    for _ in 0..n {
        cur = circle[cur].prev;
    }
    cur
}

fn insert_after(circle: &mut Vec<Node>, id: usize, v: usize) -> usize {
    let new_id = circle.len();
    let old_next = circle[id].next;
    circle[id].next = new_id;
    circle[old_next].prev = new_id;
    circle.push(Node { dat: v, next: old_next, prev: id });
    new_id
}

fn remove(circle: &mut Vec<Node>, id: usize) -> usize {
    let prev = circle[id].prev;
    let next = circle[id].next;
    circle[prev].next = next;
    circle[next].prev = prev;
    circle[id].dat
}

fn play(players: usize, points: usize) -> usize {
    let mut scores = vec![0; players];

    let mut circle = Vec::new();
    circle.push(Node {
        dat: 0,
        next: 0,
        prev: 0,
    });

    let turns = (0..players).cycle();
    let mut marbles = 1..=points;

    let mut cur = 0;

    for t in turns {
        let marble = {
            if let Some(m) = marbles.next() {
                m
            } else {
                break;
            }
        };

        if marble % 23 != 0 {
            let next = nth_fwd(&circle, cur, 1);
            cur = insert_after(&mut circle, next, marble);
        } else {
            let next = nth_back(&circle, cur, 7);
            cur = nth_fwd(&circle, next, 1);
            let removed = remove(&mut circle, next);
            scores[t] += marble + removed;
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let in_text = fs::read_to_string("input").unwrap();
    let mut it = in_text.split_whitespace();
    let players: usize = it.next().unwrap().parse().unwrap();
    let points: usize = it.nth(5).unwrap().parse().unwrap();

    let part1 = play(players, points);
    println!("Part 1: {}", part1);

    let part2 = play(players, points * 100);
    println!("Part 2: {}", part2);
}
