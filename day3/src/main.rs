use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum ParseRectError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParseRectError {
    fn from(e: ParseIntError) -> Self {
        ParseRectError::Int(e)
    }
}

#[derive(Debug)]
struct Rect {
    id: usize,
    corner: (usize, usize),
    size: (usize, usize),
}

impl FromStr for Rect {
    type Err = ParseRectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols: Vec<_> = s.split_whitespace().collect();
        let id: usize = cols
            .get(0)
            .ok_or(ParseRectError::Empty)?
            .replace("#", "")
            .parse()?;

        let cstr = cols.get(2).ok_or(ParseRectError::Empty)?.replace(":", "");

        let mut iterc = cstr.split(',').map(|x| x.parse::<usize>());

        let mut iters = cols
            .get(3)
            .ok_or(ParseRectError::Empty)?
            .split('x')
            .map(|x| x.parse::<usize>());

        let size = (
            iters.next().ok_or(ParseRectError::Empty)??,
            iters.next().ok_or(ParseRectError::Empty)??,
        );

        let corner = (
            iterc.next().ok_or(ParseRectError::Empty)??,
            iterc.next().ok_or(ParseRectError::Empty)??,
        );

        Ok(Rect { id, corner, size })
    }
}

#[derive(Debug)]
struct Fabric {
    w: usize,
    h: usize,
    buf: Vec<usize>,
}

impl Fabric {
    fn new(w: usize, h: usize) -> Fabric {
        Fabric {
            w,
            h,
            buf: vec![0; w * h],
        }
    }

    fn add_rect(&mut self, r: &Rect) {
        let lines = self.buf.chunks_mut(self.w);

        for line in lines.skip(r.corner.1).take(r.size.1) {
            for s in line.iter_mut().skip(r.corner.0).take(r.size.0) {
                *s += 1;
            }
        }
    }

    // range version
    fn _add_rect_range(&mut self, r: &Rect) {
        let lines = self.buf[(r.corner.1 * self.w)..].chunks_mut(self.w);

        for line in lines.take(r.size.1) {
            for s in line[r.corner.0..].iter_mut().take(r.size.0) {
                *s += 1;
            }
        }
    }

    // flat_map version
    fn _add_rect_flat_map(&mut self, r: &Rect) {
        let lines = self.buf.chunks_mut(self.w);

        lines
            .skip(r.corner.1)
            .take(r.size.1)
            .flat_map(|l| l.iter_mut().skip(r.corner.0).take(r.size.0))
            .for_each(|s| *s += 1);
    }

    fn count_overlap(&self) -> usize {
        self.buf.iter().filter(|&&v| v > 1).count()
    }

    fn is_safe(&self, r: &Rect) -> bool {
        let lines = self.buf.chunks(self.w);

        lines
            .skip(r.corner.1)
            .take(r.size.1)
            .flat_map(|l| l.iter().skip(r.corner.0).take(r.size.0))
            .all(|&v| v == 1)
    }
}

fn main() {
    let rdr = BufReader::new(File::open("input").unwrap());
    let rects: Vec<Rect> = rdr.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut f = Fabric::new(1000, 1000);

    for r in &rects {
        f.add_rect(r);
    }

    println!("Overlaps: {}", f.count_overlap());

    for r in &rects {
        if f.is_safe(r) {
            println!("ID {} is safe", r.id);
        }
    }
}
