use std::collections::HashMap;

struct Cell(i64, i64);

impl Cell {
    fn rack_id(&self) -> i64 {
        self.0 + 10
    }

    fn power_level(&self, serial: i64) -> i64 {
        let mut pwr = self.rack_id() * self.1;
        pwr = serial + pwr;
        pwr = pwr * self.rack_id();
        pwr = (pwr / 100) % 10;
        pwr - 5
    }
}

const AREA: usize = 300;

fn total_power_level(buf: &[i64], x: usize, y: usize, square: usize) -> i64 {
    let mut sum = 0;

    for l in buf[y * AREA..].chunks(AREA).take(square) {
        sum += l[x..].iter().take(square).sum::<i64>();
    }

    sum
}

fn main() {
    let serial = 7403;
    let mut buf = [0; (AREA * AREA)];

    for (y, l) in buf.chunks_mut(AREA).enumerate() {
        for (x, c) in l.iter_mut().enumerate() {
            *c = Cell(x as i64 + 1, y as i64 + 1).power_level(serial);
        }
    }

    // Part 1
    {
        let mut dat = HashMap::new();
        for y in 0..(AREA - 3 + 1) {
            for x in 0..(AREA - 3 + 1) {
                let lvl = total_power_level(&buf, x, y, 3);
                dat.insert((x + 1, y + 1), lvl);
            }
        }
        let max_cell = dat.iter().max_by_key(|kv| kv.1).unwrap();
        println!("Part 1: {:?}", max_cell);
    }

    // Part 2
    {
        let mut dat = HashMap::new();
        for sq in 1..300 {
            for y in 0..(AREA - sq + 1) {
                for x in 0..(AREA - sq + 1) {
                    let lvl = total_power_level(&buf, x, y, sq);
                    dat.insert((x + 1, y + 1, sq), lvl);
                }
            }
        }

        let max_cell = dat.iter().max_by_key(|kv| kv.1).unwrap();
        println!("Part 2: {:?}", max_cell);
    }
}
