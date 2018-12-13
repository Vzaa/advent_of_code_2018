use std::collections::HashMap;
use std::fs;

fn gen_rule(s: &str) -> (String, char) {
    let mut it = s.split_whitespace();
    let rule = it.next().unwrap().to_owned();
    let out = it.nth(1).unwrap().parse().unwrap();

    (rule, out)
}

const WIDTH: usize = 5;

struct Field {
    eq: Option<i64>,
    offset: i64,
    state: String,
    rules: HashMap<String, char>,
}

impl Field {
    fn next(&mut self) {
        let padded = format!("....{}....", self.state);
        let tmp_vec: Vec<char> = padded.chars().collect(); // convert to Vec<char> for .windows() :P
        let old_offset = self.offset;

        let mut new_state: String = tmp_vec
            .windows(WIDTH)
            .map(|r| {
                let s: String = r.iter().collect(); // convert to string for HashMap
                self.rules[&s]
            })
            .collect();

        self.offset += 2 - new_state.find('#').unwrap() as i64;
        new_state = new_state.trim_matches('.').to_owned();

        if new_state == self.state {
            self.eq = Some(self.offset - old_offset);
        }

        self.state = new_state;
    }

    fn sum(&self) -> i64 {
        self.state
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| (i as i64) - self.offset)
            .sum()
    }

    fn is_eq(&self) -> bool {
        self.eq.is_some()
    }

    fn yolo(&mut self, turns: i64) {
        assert!(self.is_eq());
        self.offset += self.eq.unwrap() * turns;
    }
}

fn main() {
    let text = fs::read_to_string("input").unwrap();
    let mut lines = text.lines();
    let first_line = lines.next().unwrap();

    let state: String = first_line
        .chars()
        .filter_map(|c| if c == '#' || c == '.' { Some(c) } else { None })
        .collect();

    let rules: HashMap<String, char> = lines.skip(1).map(gen_rule).collect();
    assert_eq!(rules.len(), 2_usize.pow(WIDTH as u32));

    // Part 1
    {
        let mut field = Field {
            state: state.clone(),
            rules: rules.clone(),
            offset: 0,
            eq: None,
        };

        for _ in 1..=20 {
            field.next();
        }

        println!("Part 1: {}", field.sum());
    }

    // Part 2
    {
        let limit = 50000000000_i64;

        let mut field = Field {
            state: state.clone(),
            rules: rules.clone(),
            offset: 0,
            eq: None,
        };

        let mut turn = 0;

        while !field.is_eq() {
            field.next();
            turn += 1;
        }

        field.yolo(limit - turn);

        println!("Part 2: {}", field.sum());
    }
}
