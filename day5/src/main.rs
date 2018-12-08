use std::collections::HashMap;
use std::fs;

fn process(input: &Vec<char>) -> Vec<char> {
    let mut polymer = input.clone();
    loop {
        let mut polymer_new = Vec::new();
        let mut it_1 = polymer.iter();
        let mut it_2 = polymer.iter().skip(1);

        loop {
            match (it_1.next(), it_2.next()) {
                (Some(p0), Some(p1)) => {
                    if p0 != p1 && p0.eq_ignore_ascii_case(p1) {
                        // Skip next cycle so we don't handle p1 again
                        it_1.next();
                        it_2.next();
                    } else {
                        polymer_new.push(*p0);
                    }
                }
                (Some(p0), None) => polymer_new.push(*p0),
                (None, Some(_)) => unreachable!(),
                (None, None) => break, // Reached the end
            }
        }

        if polymer.len() == polymer_new.len() {
            break;
        }

        polymer = polymer_new;
    }
    polymer
}

fn main() {
    let polymer_str = fs::read_to_string("input").unwrap();
    let polymer_org: Vec<char> = polymer_str.trim().chars().collect();

    // Part 1
    {
        let finished = process(&polymer_org);
        println!("Part 1: {:?}", finished.len());
    }

    // Part 2
    {
        let mut results = HashMap::new();
        for &c in &polymer_org {
            let lc = c.to_ascii_lowercase();
            results.entry(lc).or_insert_with(|| {
                let clean: Vec<char> = polymer_org
                    .iter()
                    .filter(|&&p| !c.eq_ignore_ascii_case(&p))
                    .cloned()
                    .collect();
                process(&clean).len()
            });
        }

        let shortest = results.values().min().unwrap();
        println!("Part 2: {}", shortest);
    }
}
