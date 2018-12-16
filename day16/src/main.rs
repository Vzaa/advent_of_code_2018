use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Reg = [i32; 4];
type Args = [i32; 3];
type OpFn = fn([i32; 4], [i32; 3]) -> [i32; 4];

fn addr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b as usize, c as usize),
    };

    reg_in[c] = reg_in[a] + reg_in[b];
    reg_in
}

fn addi(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b, c as usize),
    };

    reg_in[c] = reg_in[a] + b;
    reg_in
}

fn mulr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b as usize, c as usize),
    };

    reg_in[c] = reg_in[a] * reg_in[b];
    reg_in
}

fn muli(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b, c as usize),
    };

    reg_in[c] = reg_in[a] * b;
    reg_in
}

fn banr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b as usize, c as usize),
    };

    reg_in[c] = reg_in[a] & reg_in[b];
    reg_in
}

fn bani(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b, c as usize),
    };

    reg_in[c] = reg_in[a] & b;
    reg_in
}

fn borr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b as usize, c as usize),
    };

    reg_in[c] = reg_in[a] | reg_in[b];
    reg_in
}

fn bori(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b, c as usize),
    };

    reg_in[c] = reg_in[a] | b;
    reg_in
}

fn setr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, c) = match args {
        [a, _, c] => (a as usize, c as usize),
    };

    reg_in[c] = reg_in[a];
    reg_in
}

fn seti(mut reg_in: Reg, args: Args) -> Reg {
    let (a, c) = match args {
        [a, _, c] => (a, c as usize),
    };

    reg_in[c] = a;
    reg_in
}

fn gtir(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a, b as usize, c as usize),
    };

    reg_in[c] = (a > reg_in[b]) as i32;
    reg_in
}

fn gtri(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b, c as usize),
    };

    reg_in[c] = (reg_in[a] > b) as i32;
    reg_in
}

fn gtrr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b as usize, c as usize),
    };

    reg_in[c] = (reg_in[a] > reg_in[b]) as i32;
    reg_in
}

fn eqir(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a, b as usize, c as usize),
    };

    reg_in[c] = (a == reg_in[b]) as i32;
    reg_in
}

fn eqri(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b, c as usize),
    };

    reg_in[c] = (reg_in[a] == b) as i32;
    reg_in
}

fn eqrr(mut reg_in: Reg, args: Args) -> Reg {
    let (a, b, c) = match args {
        [a, b, c] => (a as usize, b as usize, c as usize),
    };

    reg_in[c] = (reg_in[a] == reg_in[b]) as i32;
    reg_in
}

const OPFNS: [OpFn; 16] = [
    addi, addr, // 0, 1
    mulr, muli, // 2, 3
    banr, bani, // 4, 5
    borr, bori, // 6, 7
    setr, seti, // 8, 9
    gtir, gtri, gtrr, // 10, 11, 12
    eqir, eqri, eqrr, // 13, 14, 15
];

fn get_matches(before: &Reg, after: &Reg, args: &Args) -> Vec<usize> {
    let mut same_res = vec![];

    for (idx, f) in OPFNS.iter().enumerate() {
        if f(*before, *args) == *after {
            same_res.push(idx);
        }
    }

    same_res
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());

    let mut op_lookup = HashMap::new();
    for o in 0..16 {
        op_lookup.insert(o, (0..16).collect::<HashSet<usize>>());
    }

    let mut program = vec![];
    let mut before = None;
    let mut after = None;
    let mut inst = None;
    let mut op = None;
    let mut cnt = 0;

    // Amazing parsing code
    for line in rdr.lines() {
        let line = line.unwrap();

        if line.trim().len() == 0 {
            continue;
        }

        let clean: String = line.matches(|c: char| c.is_numeric() || c == ' ').collect();

        let mut it = clean.split_whitespace();

        let vals: Reg = [
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        ];

        if line.contains("Before") {
            before = Some(vals);
        } else if line.contains("After") {
            after = Some(vals);
        } else {
            op = Some(vals[0]);
            inst = Some([vals[1], vals[2], vals[3]]);
        }

        if let (Some(b), Some(a), Some(i), Some(o)) = (before, after, inst, op) {
            before = None;
            after = None;
            inst = None;
            op = None;

            // Get which ops match this
            let matches = get_matches(&b, &a, &i);
            for oo in (0..16).filter(|v| !matches.contains(v)) {
                op_lookup.get_mut(&o).unwrap().remove(&oo);
            }

            // Count for Part 1
            if matches.len() >= 3 {
                cnt += 1;
            }
        } else if let (Some(i), Some(o), None, None) = (inst, op, before, after) {
            // Read the program for Part 2
            program.push((o, i));
            inst = None;
            op = None;
        }
    }

    println!("Part 1: {}", cnt);

    // Part 2
    {
        // Figure out ops
        loop {
            // Get values that can only be one op
            let ones: Vec<_> = op_lookup
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(k, v)| (*k, *v.iter().next().unwrap()))
                .collect();

            // All values match one possible op
            if ones.len() == 16 {
                break;
            }

            // Remove "ones" from others' lists
            for (op, del) in &ones {
                for o in (0..16).filter(|v| v != op) {
                    op_lookup.get_mut(&o).unwrap().remove(&del);
                }
            }
        }

        // Convert to (i32, usize) for easier use
        let op_lookup: HashMap<_, _> = op_lookup
            .into_iter()
            .map(|(k, v)| (k, *v.iter().next().unwrap()))
            .collect();

        // Run the program
        let mut reg = [0; 4];
        for (op, args) in &program {
            let f = OPFNS[op_lookup[op]];
            reg = f(reg, *args);
        }
        println!("{:?}", reg);
    }
}
