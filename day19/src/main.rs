use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Reg = [i32; 6];
type Args = [i32; 3];
type OpFn = fn([i32; 6], [i32; 3]) -> [i32; 6];

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

fn main() {
    let opcode_map: HashMap<_, _> = vec![
        ("addi", addi as OpFn),
        ("addr", addr),
        ("mulr", mulr),
        ("muli", muli),
        ("banr", banr),
        ("bani", bani),
        ("borr", borr),
        ("bori", bori),
        ("setr", setr),
        ("seti", seti),
        ("gtir", gtir),
        ("gtri", gtri),
        ("gtrr", gtrr),
        ("eqir", eqir),
        ("eqri", eqri),
        ("eqrr", eqrr),
    ]
    .into_iter()
    .collect();

    // The Amazing Parse-Man
    let (pc_reg, program) = {
        let rdr = BufReader::new(File::open("input").unwrap());
        let mut line_it = rdr.lines();
        let mut program = vec![];

        let first = line_it.next().unwrap().unwrap();
        let pc_reg: usize = first.split_whitespace().nth(1).unwrap().parse().unwrap();

        for line in line_it {
            let line = line.unwrap();
            let mut words = line.split_whitespace();
            let op = words.next().unwrap().to_owned();

            let args: Args = [
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
                words.next().unwrap().parse().unwrap(),
            ];

            program.push((op, args));
        }

        (pc_reg, program)
    };

    // Part 1
    {
        let mut reg = [0_i32; 6];

        for _ in 0.. {
            let pc = reg[pc_reg];
            let (op, args) = if let Some((o, a)) = program.get(pc as usize) {
                (o, a)
            } else {
                break;
            };
            let f = opcode_map[op.as_str()];
            reg = f(reg, *args);
            reg[pc_reg] += 1;
        }

        println!("Part 1 Final reg: {:?}", reg);
    }

    // Part 2
    // Converted to C and ran lol:

    // #include <stdio.h>
    // int main(int argc, char *argv[])
    // {
    //     // Skip init code
    //     int a = 0;
    //     int d = 1;
    //     int f = 1;
    //     int b = 10551315;
    //
    // #if 0 // Initial conversion
    //     for(;;) {
    //         if (d * f == b) { //pc 6
    //             a = a + d; //pc 7
    //         }
    //
    //         f = f + 1; // pc 8
    //         if (f > b) { // pc 10
    //             d = d + 1;//pc 12
    //             if (d > b) { //pc 14
    //                 break; // pc 16
    //             } else {
    //                 f = 1; // pc 2
    //             }
    //         }
    //     }
    // #endif
    //
    //     for(d = 1; d <= b; d++) {
    //         for (f = 1; f <= b / d; f++) {
    //             if (d * f == b) {
    //                 a = a + d;
    //             }
    //         }
    //     }
    //     printf("%d\n", a);
    //     return 0;
    // }
}
