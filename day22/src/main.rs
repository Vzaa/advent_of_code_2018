use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}

impl Type {
    fn from_lvl(lvl: i32) -> Type {
        match lvl % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => unreachable!(),
        }
    }

    fn as_char(&self) -> char {
        match *self {
            Type::Rocky => '.',
            Type::Wet => '=',
            Type::Narrow => '|',
        }
    }

    fn risk(&self) -> i32 {
        match *self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
        }
    }

    fn tools(&self) -> [Tool; 2] {
        match *self {
            Type::Rocky => [Tool::CGear, Tool::Torch],
            Type::Wet => [Tool::CGear, Tool::Neither],
            Type::Narrow => [Tool::Torch, Tool::Neither],
        }
    }
}

fn elevel(glevels: &mut HashMap<Pos, i32>, p: Pos) -> i32 {
    let g = geo_index(p, glevels);
    (g + DEPTH) % 20183
}

fn get_type(glevels: &mut HashMap<Pos, i32>, p: Pos) -> Type {
    let e = elevel(glevels, p);
    Type::from_lvl(e)
}

const TARGET: Pos = (10, 725);
const DEPTH: i32 = 8787;
//const TARGET: Pos = (10, 10);
//const DEPTH: i32 = 510;

fn geo_index(p: Pos, glevels: &mut HashMap<Pos, i32>) -> i32 {
    if let Some(g) = glevels.get(&p) {
        return *g;
    }

    let g = match p {
        (0, 0) | TARGET => 0,
        (x, 0) => x * 16807,
        (0, y) => y * 48271,
        (x, y) => {
            let p1 = (x - 1, y);
            let p2 = (x, y - 1);
            let e1 = elevel(glevels, p1);
            let e2 = elevel(glevels, p2);
            e1 * e2
        }
    };

    glevels.insert(p, g);
    g
}

fn main() {
    let mut glevels = HashMap::new();
    let mut sum = 0;

    for y in 0..=TARGET.1 {
        for x in 0..=TARGET.0 {
            let e = elevel(&mut glevels, (x, y));
            let t = Type::from_lvl(e);

            sum += t.risk();
        }
    }

    println!("Part 1: {}", sum);

    ucs(&mut glevels);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tool {
    Neither,
    Torch,
    CGear,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    p: Pos,
    t: Tool,
}

impl State {
    pub fn new(t: Tool, p: Pos) -> State {
        State { p, t }
    }
}

#[derive(PartialEq, Eq)]
struct C(State, i32);

impl Ord for C {
    fn cmp(&self, other: &C) -> Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for C {
    fn partial_cmp(&self, other: &C) -> Option<Ordering> {
        Some(self.1.cmp(&other.1).reverse())
    }
}

fn ucs(maze: &mut HashMap<Pos, i32>) {
    let adj = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let initial = State::new(Tool::Torch, (0, 0));

    let mut frontier = BinaryHeap::new();
    frontier.push(C(initial, 0));
    let mut bests = HashMap::new();

    loop {
        // get min cost
        let C(cur, cost) = frontier.pop().unwrap();

        let p = cur.p;
        let t = get_type(maze, p);

        if p == TARGET && cur.t == Tool::Torch {
            println!("Part 2: {}", cost);
            return;
        }

        let neighbors = adj
            .iter()
            .map(|&s| (s.0 + p.0, s.1 + p.1))
            .filter(|&n| n.0 >= 0 && n.1 >= 0);

        for tool in &t.tools() {
            if cur.t != *tool { // same pos, different tool
                let new_state = State::new(*tool, p);
                let new_cost = cost + 7;

                if let Some(b) = bests.get_mut(&new_state) {
                    if new_cost < *b {
                        *b = new_cost;
                        frontier.push(C(new_state, new_cost));
                    }
                } else {
                    bests.insert(new_state.clone(), new_cost);
                    frontier.push(C(new_state, new_cost));
                }
            }
        }

        for n in neighbors {
            let nt = get_type(maze, n);
            if nt.tools().contains(&cur.t) { // can we use this tool there?
                let new_state = State::new(cur.t, n);
                let new_cost = cost + 1;

                if let Some(b) = bests.get_mut(&new_state) {
                    if new_cost < *b {
                        *b = new_cost;
                        frontier.push(C(new_state, new_cost));
                    }
                } else {
                    bests.insert(new_state.clone(), new_cost);
                    frontier.push(C(new_state, new_cost));
                }
            }
        }
    }
}
