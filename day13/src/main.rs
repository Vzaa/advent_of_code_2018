use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum C {
    Beam,
    Inter,
    Line,
    L,
    R,
}

impl C {
    fn from_char(c: char) -> Option<C> {
        let out = match c {
            '|' => C::Beam,
            '^' => C::Beam,
            'v' => C::Beam,
            '+' => C::Inter,
            '-' => C::Line,
            '>' => C::Line,
            '<' => C::Line,
            '\\' => C::L,
            '/' => C::R,
            ' ' => return None,
            x => panic!("'{}' nope", x),
        };
        Some(out)
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Straight,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(self, t: Turn) -> Dir {
        let left = |d| match d {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        };

        let right = |d| match d {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        };

        match t {
            Turn::Straight => self,
            Turn::Left => left(self),
            Turn::Right => right(self),
        }
    }
}

#[derive(Debug, Clone)]
struct TurnGen {
    turns: Vec<Turn>,
    id: usize,
}

impl TurnGen {
    pub fn new(t: &[Turn]) -> TurnGen {
        TurnGen {
            turns: t.to_vec(),
            id: 0,
        }
    }
}

impl Iterator for TurnGen {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.turns[self.id];
        self.id = (self.id + 1) % self.turns.len();
        Some(t)
    }
}

#[derive(Debug, Clone)]
struct Cart {
    dead: bool,
    pos: (usize, usize),
    dir: Dir,
    turns: TurnGen,
}

const TURN_RULE: [Turn; 3] = [Turn::Left, Turn::Straight, Turn::Right];

impl Cart {
    fn from_char(pos: (usize, usize), c: char) -> Option<Cart> {
        let dir = match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => return None,
        };

        let turns = TurnGen::new(&TURN_RULE);

        Some(Cart {
            pos,
            dir,
            turns,
            dead: false,
        })
    }

    fn mv(&mut self, map: &HashMap<(usize, usize), C>) {
        let next_pos = match self.dir {
            Dir::Up => (self.pos.0, self.pos.1 - 1),
            Dir::Down => (self.pos.0, self.pos.1 + 1),
            Dir::Left => (self.pos.0 - 1, self.pos.1),
            Dir::Right => (self.pos.0 + 1, self.pos.1),
        };

        let next_dir = match map[&next_pos] {
            C::L => match self.dir {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            },
            C::R => match self.dir {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
            },
            C::Inter => {
                let turn = self.turns.next().unwrap();
                self.dir.turn(turn)
            }
            _ => self.dir,
        };

        self.dir = next_dir;
        self.pos = next_pos;
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut map = HashMap::new();
    let mut carts = vec![];

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if let Some(cell) = C::from_char(c) {
                map.insert((x, y), cell);
            }

            if let Some(cart) = Cart::from_char((x, y), c) {
                carts.push(cart);
            }
        }
    }

    // Part 1
    {
        let mut carts = carts.clone();
        'movement: loop {
            carts.sort_by_key(|c| (c.pos.1, c.pos.0));

            // Onwership RIP
            //for (idx, cart) in carts.iter_mut().enumerate() {
            //cart.mv(&map);

            //let collision = carts.iter().any(|c| c.pos == cart.pos);
            //}

            for i in 0..carts.len() {
                carts[i].mv(&map);

                let collision = carts
                    .iter()
                    .filter(|c| *c as *const Cart != &carts[i] as *const Cart)
                    .any(|c| c.pos == carts[i].pos);
                if collision {
                    println!("Part 1: {:?}", carts[i].pos);
                    break 'movement;
                }
            }
        }
    }

    {
        let mut carts = carts.clone();
        loop {
            carts.sort_by_key(|c| (c.pos.1, c.pos.0));

            // Onwership RIP
            for i in 0..carts.len() {
                if carts[i].dead {
                    continue;
                }

                carts[i].mv(&map);

                let collided = carts
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| !c.dead)
                    .filter(|(_, c)| *c as *const Cart != &carts[i] as *const Cart)
                    .filter(|(_, c)| c.pos == carts[i].pos)
                    .next();

                if let Some((other, _)) = collided {
                    carts[i].dead = true;
                    carts[other].dead = true;
                }
            }

            let alive_cnt = carts.iter().filter(|c| !c.dead).count();

            if alive_cnt == 1 {
                let alive = carts.iter().filter(|c| !c.dead).next().unwrap();
                println!("Part 2: {:?}", alive.pos);
                break;
            }
        }
    }
}
