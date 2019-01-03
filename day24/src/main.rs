use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Side {
    ImmuneSystem,
    Infection,
}

// stringly typed rip
#[derive(Debug, Clone)]
struct Group {
    side: Side,
    units: i32,
    hp: i32,
    ap: i32,
    weak: Vec<String>,
    immune: Vec<String>,
    attack_type: String,
    initiative: i32,
    tgt: Option<usize>,
}

impl Group {
    fn epower(&self) -> i32 {
        self.ap * self.units
    }

    fn calc_dmg_from(&self, other: &Group) -> i32 {
        if self.weak.contains(&other.attack_type) {
            other.epower() * 2
        } else if self.immune.contains(&other.attack_type) {
            0
        } else {
            other.epower()
        }
    }

    fn get_attacked(&mut self, dmg: i32) -> i32 {
        let dead_cnt = dmg / self.hp;
        self.units -= dead_cnt;
        if self.units < 0 {
            self.units = 0;
        }
        dead_cnt
    }

    fn target(&self, others: &HashMap<usize, Group>, tgts: &Vec<usize>) -> Option<usize> {
        tgts.iter()
            .filter(|&g| others[g].side != self.side)
            .filter(|&g| others[g].calc_dmg_from(self) != 0)
            .max_by(|&a, &b| {
                let dmg_cmp = others[a]
                    .calc_dmg_from(self)
                    .cmp(&others[b].calc_dmg_from(self));
                if let Ordering::Equal = dmg_cmp {
                    let ep_cmp = others[a].epower().cmp(&others[b].epower());
                    if let Ordering::Equal = ep_cmp {
                        others[a].initiative.cmp(&others[b].initiative)
                    } else {
                        ep_cmp
                    }
                } else {
                    dmg_cmp
                }
            })
            .map(|t| *t)
    }
}

#[derive(Debug)]
enum ParseGroupError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseGroupError {
    fn from(e: ParseIntError) -> Self {
        ParseGroupError::Int(e)
    }
}

impl FromStr for Group {
    type Err = ParseGroupError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The Amazing Parse-Man:
        let mut val: Option<i32> = None;
        let mut prev: Option<&str> = None;
        let mut units: Option<i32> = None;
        let mut hp: Option<i32> = None;
        let mut ap: Option<i32> = None;
        let mut attack_type: Option<&str> = None;
        let mut weak = vec![];
        let mut immune = vec![];

        for w in s.split_whitespace() {
            if let Ok(v) = w.parse() {
                val = Some(v);
            }

            match w {
                "units" => {
                    units = val.take();
                }
                "hit" => {
                    hp = val.take();
                }
                "damage" => {
                    attack_type = prev.take();
                    ap = val.take();
                }
                _ => (),
            }
            prev = Some(w);
        }

        let initiative = val;

        // Parse weak/immune
        {
            let weakto = s.split("weak to").nth(1);
            if let Some(w) = weakto {
                let wstr: String = w
                    .chars()
                    .take_while(|&c| c != ';' && c != ')')
                    .filter(|&c| c != ',')
                    .collect();
                weak = wstr.split_whitespace().map(|s| s.to_owned()).collect();
            }
        }

        {
            let immuneto = s.split("immune to").nth(1);
            if let Some(w) = immuneto {
                let wstr: String = w
                    .chars()
                    .take_while(|&c| c != ';' && c != ')')
                    .filter(|&c| c != ',')
                    .collect();
                immune = wstr.split_whitespace().map(|s| s.to_owned()).collect();
            }
        }

        Ok(Group {
            units: units.ok_or(ParseGroupError::Empty)?,
            hp: hp.ok_or(ParseGroupError::Empty)?,
            ap: ap.ok_or(ParseGroupError::Empty)?,
            weak,
            immune: immune,
            attack_type: attack_type.ok_or(ParseGroupError::Empty)?.to_owned(),
            initiative: initiative.ok_or(ParseGroupError::Empty)?,
            tgt: None,
            side: Side::ImmuneSystem,
        })
    }
}

fn run(grps: &mut HashMap<usize, Group>, boost: i32) -> bool {
    for g in grps.values_mut().filter(|g| g.side == Side::ImmuneSystem) {
        g.ap += boost;
    }

    loop {
        // Sort by desc epower, initiative
        let by_epower: Vec<usize> = {
            let mut tmp: Vec<_> = grps.iter().collect();
            tmp.sort_by(|(_, a), (_, b)| {
                let ep_cmp = b.epower().cmp(&a.epower());
                if let Ordering::Equal = ep_cmp {
                    b.initiative.cmp(&a.initiative)
                } else {
                    ep_cmp
                }
            });
            tmp.iter().map(|a| *a.0).collect()
        };

        let mut tgts: Vec<usize> = grps.keys().cloned().collect();

        // Target the ids
        for id in &by_epower {
            let tgt = grps[id].target(&grps, &tgts);
            grps.get_mut(id).unwrap().tgt = tgt;
            if let Some(t) = tgt {
                tgts.retain(|&g| g != t);
            }
        }

        // Sort by initiatives
        let by_init: Vec<usize> = {
            let mut tmp: Vec<_> = grps.iter().collect();
            tmp.sort_by(|(_, a), (_, b)| b.initiative.cmp(&a.initiative));
            tmp.iter().map(|a| *a.0).collect()
        };

        let mut dead_sum = 0;
        for id in &by_init {
            if grps[id].units <= 0 {
                continue;
            }

            if let Some(tgt_idx) = grps[id].tgt {
                let dmg = grps[&tgt_idx].calc_dmg_from(&grps[id]);
                dead_sum += grps.get_mut(&tgt_idx).take().unwrap().get_attacked(dmg);
            }
        }

        grps.retain(|_, g| g.units > 0);

        let units_a: i32 = grps
            .values()
            .filter(|g| g.side == Side::Infection)
            .map(|g| g.units)
            .sum();
        let units_b: i32 = grps
            .values()
            .filter(|g| g.side == Side::ImmuneSystem)
            .map(|g| g.units)
            .sum();

        if units_a == 0 {
            println!("Part 2: Immune: {}, boost: {}", units_b, boost);
            return true;
        }

        if units_b == 0 {
            if boost == 0 {
                println!("Part 1: Infection: {}", units_a);
            }
            return false;
        }

        // Deadlock case
        if dead_sum == 0 {
            return false;
        }
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut grps = HashMap::new();

    let mut id_gen = 0..;
    let mut cur_side = None;

    for line in rdr.lines() {
        let line = line.unwrap();
        match line.as_str() {
            "Immune System:" => cur_side = Some(Side::ImmuneSystem),
            "Infection:" => cur_side = Some(Side::Infection),
            "" => (),
            _ => {
                let mut g: Group = line.parse().unwrap();
                g.side = cur_side.unwrap();
                grps.insert(id_gen.next().unwrap(), g);
            }
        }
    }

    for b in 0.. {
        let mut grps = grps.clone();
        if run(&mut grps, b) {
            break;
        }
    }
}
