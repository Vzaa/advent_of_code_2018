use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum ParseDateError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseDateError {
    fn from(e: ParseIntError) -> Self {
        ParseDateError::Int(e)
    }
}

#[derive(Debug, Clone, Copy)]
struct Date {
    yr: i64,
    mth: i64,
    day: i64,
    hr: i64,
    min: i64,
}

impl Date {
    fn _as_mins(&self) -> i64 {
        let days_in_month = match self.mth {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28, // rip leap years
            _ => panic!("invalid month"),
        };

        self.min
            + (self.hr * 60)
            + (self.day * 24 * 60)
            + (self.mth * days_in_month * 24 * 60)
            + (self.yr * 365 * 31 * 24 * 60)
    }
}

impl FromStr for Date {
    type Err = ParseDateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.replace("[", "").replace("]", "");
        let mut sp = stripped.split_whitespace();

        let datestr = sp.next().ok_or(ParseDateError::Empty)?;
        let timestr = sp.next().ok_or(ParseDateError::Empty)?;

        let mut dateit = datestr.split("-");
        let mut timeit = timestr.split(":");

        let yr = dateit.next().ok_or(ParseDateError::Empty)?.parse()?;
        let mth = dateit.next().ok_or(ParseDateError::Empty)?.parse()?;
        let day = dateit.next().ok_or(ParseDateError::Empty)?.parse()?;
        let hr = timeit.next().ok_or(ParseDateError::Empty)?.parse()?;
        let min = timeit.next().ok_or(ParseDateError::Empty)?.parse()?;

        Ok(Date {
            yr,
            mth,
            day,
            hr,
            min,
        })
    }
}

#[derive(Debug)]
enum Action {
    Begin,
    Sleep,
    WakeUp,
}

#[derive(Debug)]
struct Entry {
    date: Date,
    act: Action,
    id: Option<i64>,
}

impl FromStr for Entry {
    type Err = ParseDateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ts, actstr) = s.split_at(s.find(']').ok_or(ParseDateError::Empty)? + 1);
        let date = ts.parse()?;
        let mut id = None;

        let act = if actstr.contains("begins") {
            let s = actstr
                .split_whitespace()
                .nth(1)
                .ok_or(ParseDateError::Empty)?
                .replace("#", "");
            id = Some(s.parse()?);
            Action::Begin
        } else if actstr.contains("asleep") {
            Action::Sleep
        } else if actstr.contains("wakes") {
            Action::WakeUp
        } else {
            return Err(ParseDateError::Empty);
        };

        Ok(Entry { date, act, id })
    }
}

// Propogate ids from Begin actions
fn assign_ids(entries: &mut [Entry]) {
    let mut cur_id = None;
    for e in entries.iter_mut() {
        if e.id.is_some() {
            cur_id = e.id;
        } else {
            assert_ne!(cur_id, None);
            e.id = cur_id;
        }
    }
}

// Get entries in order and with IDs set
fn get_entries() -> Vec<Entry> {
    let rdr = BufReader::new(File::open("input").unwrap());
    let mut lines: Vec<String> = rdr.lines().map(|l| l.unwrap()).collect();
    lines.sort();
    let mut entries: Vec<Entry> = lines.iter().map(|l| l.parse().unwrap()).collect();
    assign_ids(&mut entries);
    entries
}

fn main() {
    let entries = get_entries();
    let mut sleep_mins = HashMap::new();

    // process sleep mins
    for (e1, e2) in entries.iter().zip(entries[1..].iter()) {
        match (&e1.act, &e2.act) {
            (Action::Sleep, Action::WakeUp) => {
                let (min1, min2) = (e1.date.min, e2.date.min);
                assert!(e1.id == e2.id);
                assert!(min1 < min2);
                for min in min1..min2 {
                    let v = sleep_mins.entry(e1.id).or_insert(Vec::new());
                    v.push(min);
                }
            }
            _ => (),
        }
    }

    // Part 1
    {
        // Get max sleeper
        let (id, mins) = sleep_mins
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len()))
            .unwrap();

        // Count occurences of minutes
        let mut hist = HashMap::new();
        for &min in mins.iter() {
            let v = hist.entry(min).or_insert(0);
            *v += 1;
        }

        let (&min, _) = hist.iter().max_by(|(_, v1), (_, v2)| v1.cmp(&v2)).unwrap();

        println!(
            "Part 1: id({}) x min({}) = {}",
            id.unwrap(),
            min,
            (id.unwrap()) * min
        );
    }

    // Part 2
    {
        let mut most_mins = HashMap::new();
        for id in sleep_mins.keys() {
            let mins = match sleep_mins.get(id) {
                Some(m) => m,
                None => continue, // never slept so not in sleep_mins
            };

            let mut hist = HashMap::new();
            for &min in mins.iter() {
                let v = hist.entry(min).or_insert(0);
                *v += 1;
            }

            let (&min, &cnt) = hist.iter().max_by(|(_, v1), (_, v2)| v1.cmp(&v2)).unwrap();

            most_mins.insert(id, (min, cnt));
        }

        let (&id, (min, _)) = most_mins
            .iter()
            .max_by(|(_, (_, v1)), (_, (_, v2))| v1.cmp(&v2))
            .unwrap();

        println!(
            "Part 2: id({}) x min({}) = {}",
            id.unwrap(),
            min,
            (id.unwrap()) * min
        );
    }
}
