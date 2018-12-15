use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Pos = (i32, i32);
type TileMap = HashMap<Pos, Tile>;
type ActorMap = HashMap<Pos, Actor>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '#' => Tile::Wall,
            '.' | 'G' | 'E' => Tile::Floor,
            x => panic!("'{}' nope", x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    Elf,
    Goblin,
}

#[derive(Debug, Clone)]
struct Actor {
    pos: Pos,
    ap: i32,
    hp: i32,
    role: Role,
}

/// Get moveable spaces for a given position
fn spaces(map: &TileMap, actors: &ActorMap, p: Pos) -> Vec<Pos> {
    let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    neighbors
        .iter()
        .map(|&s| (s.0 + p.0, s.1 + p.1))
        .filter(|s| map[&s] != Tile::Wall) // No walls
        .filter(|s| actors.get(&s).is_none()) // No other actors
        .collect()
}

/// Get all reachable locations with their distances
fn distance_map(
    map: &TileMap,
    actors: &ActorMap,
    p: Pos,
    stop_point: Option<Pos>,
) -> HashMap<Pos, usize> {
    let mut dmap = HashMap::new();

    dmap.insert(p, 0);
    let mut last = vec![p]; // vec is faster than hashset for the input size

    for dist in 0.. {
        let mut cur = vec![];
        'dmap: for p in &last {
            for s in spaces(map, actors, *p) {
                if !dmap.contains_key(&s) && !cur.contains(&s) {
                    cur.push(s);
                }
                if Some(s) == stop_point {
                    break 'dmap;
                }
            }
        }

        if cur.is_empty() {
            break;
        }

        for t in &cur {
            dmap.insert(*t, dist + 1);
            if Some(*t) == stop_point {
                return dmap;
            }
        }
        last = cur;
    }

    dmap
}

/// Finds the min distance by calculating everything, RIP A*
fn disance_between(map: &TileMap, actors: &ActorMap, a: Pos, b: Pos) -> usize {
    distance_map(map, actors, a, Some(b))[&b]
}

impl Actor {
    fn from_char(pos: Pos, c: char) -> Option<Actor> {
        let role = match c {
            'E' => Role::Elf,
            'G' => Role::Goblin,
            '.' | '#' => return None,
            x => panic!("'{}' nope", x),
        };
        Some(Actor {
            role,
            pos,
            hp: 200,
            ap: 3,
        })
    }

    /// Get target enemy pos if in range
    fn enemy(&self, actors: &ActorMap) -> Option<Pos> {
        let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let p = self.pos;

        neighbors
            .iter()
            .map(|&s| (s.0 + p.0, s.1 + p.1))
            .filter_map(|s| actors.get(&s))
            .filter(|a| a.role != self.role)
            .min_by(|a, b| {
                let hp_cmp = a.hp.cmp(&b.hp);
                if let Ordering::Equal = hp_cmp {
                    ((a.pos).1, (a.pos).0).cmp(&((b.pos).1, (b.pos).0)) // Thanks Obama
                } else {
                    hp_cmp
                }
            })
            .map(|a| a.pos)
    }

    /// One turn
    fn mv(&mut self, map: &TileMap, actors: &mut ActorMap) {
        // Enemy in target
        if let Some(enemy_pos) = self.enemy(actors) {
            let mut enemy = actors.get_mut(&enemy_pos).unwrap();
            enemy.hp -= self.ap;
            if enemy.hp <= 0 {
                actors.remove(&enemy_pos);
            }
            return;
        }

        let in_range = actors
            .values()
            .filter(|a| a.role != self.role)
            .flat_map(|a| spaces(map, actors, a.pos));

        let dmap = distance_map(map, actors, self.pos, None);

        let reachables = in_range
            .filter(|p| dmap.contains_key(p))
            .map(|p| (p, dmap[&p]));

        let target_dest = reachables.min_by(|a, b| {
            let dist_cmp = a.1.cmp(&b.1);
            if let Ordering::Equal = dist_cmp {
                ((a.0).1, (a.0).0).cmp(&((b.0).1, (b.0).0)) // Thanks Obama
            } else {
                dist_cmp
            }
        });

        let next_pos = if let Some((p, _)) = target_dest {
            spaces(map, actors, self.pos)
                .iter()
                .map(|s| (s, disance_between(map, actors, *s, p)))
                .min_by(|a, b| {
                    let dist_cmp = a.1.cmp(&b.1);
                    if let Ordering::Equal = dist_cmp {
                        ((a.0).1, (a.0).0).cmp(&((b.0).1, (b.0).0)) // Thanks Obama
                    } else {
                        dist_cmp
                    }
                })
                .map(|a| *a.0)
        } else {
            None
        };

        self.pos = next_pos.unwrap_or(self.pos);

        // Check for enemies again
        if let Some(enemy_pos) = self.enemy(actors) {
            let mut enemy = actors.get_mut(&enemy_pos).unwrap();
            enemy.hp -= self.ap;
            if enemy.hp <= 0 {
                actors.remove(&enemy_pos);
            }
        }
    }
}

#[allow(dead_code)]
fn draw_map(map: &TileMap, actors: &ActorMap) {
    'outer: for y in 0.. {
        for x in 0.. {
            let c = if let Some(actor) = actors.get(&(x, y)) {
                match actor.role {
                    Role::Goblin => 'G',
                    Role::Elf => 'E',
                }
            } else if let Some(tile) = map.get(&(x, y)) {
                match tile {
                    Tile::Wall => '#',
                    Tile::Floor => '.',
                }
            } else {
                if x == 0 {
                    break 'outer;
                } else {
                    break;
                }
            };
            print!("{}", c)
        }
        println!("");
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut map = HashMap::new();
    let mut actors = HashMap::new();

    for (y, line) in rdr.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            map.insert((x, y), Tile::from_char(c));

            if let Some(actor) = Actor::from_char((x, y), c) {
                actors.insert((x, y), actor);
            }
        }
    }

    // Part 1
    println!("Part 1");
    {
        let mut actors = actors.clone();

        //draw_map(&map, &actors);
        //println!("");

        for round in 0.. {
            let mut turns = actors.keys().cloned().collect::<Vec<_>>();
            turns.sort_by_key(|c| (c.1, c.0));

            for t in turns {
                // Will get None if died mid turn so check it
                if let Some(mut act) = actors.remove(&t) {
                    act.mv(&map, &mut actors);
                    actors.insert(act.pos, act);
                }
            }
            //println!("{}", round);
            //draw_map(&map, &actors);
            //println!("");

            if actors.values().all(|x| x.role == Role::Elf) {
                let hps: i32 = actors.values().map(|a| a.hp).sum();
                let outcome = hps * round;
                println!("Elves won in {}, HP: {}, Outcome: {}", round, hps, outcome);
                break;
            }

            if actors.values().all(|x| x.role == Role::Goblin) {
                let hps: i32 = actors.values().map(|a| a.hp).sum();
                let outcome = hps * round;
                println!(
                    "Goblins won in {}, HP: {}, Outcome: {}",
                    round, hps, outcome
                );
                break;
            }
        }
    }

    // Part 2
    println!("Part 2");
    {
        let elves = actors.values().filter(|a| a.role == Role::Elf).count();

        // Maybe we should do binary search but whatevz, got the star already
        'ap: for ap in 4.. {
            let mut actors = actors.clone();
            for e in actors.values_mut().filter(|a| a.role == Role::Elf) {
                e.ap = ap;
            }

            for round in 0.. {
                let mut turns = actors.keys().cloned().collect::<Vec<_>>();
                turns.sort_by_key(|c| (c.1, c.0));

                for t in turns {
                    // Will get None if died mid turn so check it
                    if let Some(mut act) = actors.remove(&t) {
                        act.mv(&map, &mut actors);
                        actors.insert(act.pos, act);
                    }
                }

                let cur_elves = actors.values().filter(|a| a.role == Role::Elf).count();
                if cur_elves < elves {
                    println!("Can't win in {}", ap);
                    continue 'ap;
                }
                //println!("{}", round);
                //draw_map(&map, &actors);
                //println!("");

                if actors.values().all(|x| x.role == Role::Elf) {
                    let hps: i32 = actors.values().map(|a| a.hp).sum();
                    let outcome = hps * round;
                    println!("Elves won in {}, HP: {}, Outcome: {}", round, hps, outcome);
                    println!("AP: {}", ap);
                    break 'ap;
                }

                if actors.values().all(|x| x.role == Role::Goblin) {
                    let hps: i32 = actors.values().map(|a| a.hp).sum();
                    let outcome = hps * round;
                    println!(
                        "Goblins won in {}, HP: {}, Outcome: {}",
                        round, hps, outcome
                    );
                    break 'ap;
                }
            }
        }
    }
}
