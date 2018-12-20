use std::fs;
use std::collections::HashMap;

type Pos = (i32, i32);

// "Magical" version that doesn't read directions at all
fn part1(text: &str) -> (usize, usize) {
    let nonletter = text.find(|c| c == '|' || c == '(' || c == ')');

    let token_len = if let Some(l) = nonletter {
        l
    } else {
        return (text.len(), text.len());
    };

    let ch = text.chars().nth(token_len).unwrap();
    let text_len = token_len + 1;

    match ch {
        '|' | ')' => (token_len, text_len),
        '(' => {
            let mut children_len = None;
            let mut child_text_len = 0;

            loop {
                if text.chars().nth(token_len + child_text_len).unwrap() == ')' {
                    break;
                }

                let (clen, ctext_len) = part1(&text[text_len + child_text_len..]);
                child_text_len += ctext_len;

                if clen == 0 {
                    children_len = Some(0);
                } else if let Some(m) = children_len.as_mut() {
                    *m = std::cmp::max(clen, *m);
                } else {
                    children_len = Some(clen);
                }
            }

            let children_len = children_len.unwrap_or(0);

            let (next_len, next_text_len) = part1(&text[text_len + child_text_len..]);

            (
                token_len + children_len + next_len,
                text_len + child_text_len + next_text_len,
            )
        }
        _ => panic!("nope son"),
    }
}


fn char_to_vec(c: char) -> (i32, i32) {
    match c {
         'N' => (1, 0),
         'S' => (-1, 0),
         'W' => (0, -1),
         'E' => (0, 1),
         _ => panic!("nope"),
    }
}

// Not as magical
fn part2(text: &str, maze: &mut HashMap<Pos, usize>, pos: Pos) -> (usize, Pos) {
    let nonletter = text.find(|c| c == '|' || c == '(' || c == ')');

    let mut dist = maze[&pos];

    let mut cur_pos = pos;

    for c in text.chars().take_while(|&c| c != '|' && c != '(' && c != ')') {
        let v = char_to_vec(c);
        cur_pos = (cur_pos.0 + v.0, cur_pos.1 + v.1);
        dist += 1;
        // Only insert when first seen, we only enter the same room on detours
        maze.entry(cur_pos).or_insert(dist);
    }

    let token_len = if let Some(l) = nonletter {
        l
    } else {
        return (text.len(), cur_pos);
    };

    let ch = text.chars().nth(token_len).unwrap();
    let text_len = token_len + 1;

    match ch {
        '|' | ')' => (text_len, cur_pos),
        '(' => {
            let mut child_text_len = 0;
            let mut cpos = cur_pos;
            loop {
                if text.chars().nth(token_len + child_text_len).unwrap() == ')' {
                    break;
                }

                let (ctext_len, c) = part2(&text[text_len + child_text_len..], maze, cur_pos);
                cpos = c;
                child_text_len += ctext_len;
            }

            let (next_text_len, fpos) = part2(&text[text_len + child_text_len..], maze, cpos);

            (text_len + child_text_len + next_text_len, fpos)
        }
        _ => panic!("nope son"),
    }
}

fn run(text: &str) -> usize {
    let stripped = &text[1..text.len() - 1];
    let (max_len, text_read) = part1(stripped);
    // Assert we read all of it
    assert_eq!(text_read, stripped.len());
    max_len
}

fn run2(text: &str) -> usize {
    let stripped = &text[1..text.len() - 1];
    let mut maze = HashMap::new();
    maze.insert((0, 0), 0);
    let (text_read, _) = part2(stripped, &mut maze, (0, 0));
    // Assert we read all of it
    assert_eq!(text_read, stripped.len());

    maze.values().filter(|&&d| d >= 1000).count()
}


fn main() {
    let text = fs::read_to_string("input").unwrap();
    let len = run(&text.trim());
    println!("Part 1: {}", len);
    let len = run2(&text.trim());
    println!("Part 2: {}", len);
}
