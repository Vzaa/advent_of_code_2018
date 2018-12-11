use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn mv(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

#[derive(Debug)]
enum ParsePointError {
    Int(ParseIntError),
    Empty,
}

impl From<ParseIntError> for ParsePointError {
    fn from(e: ParseIntError) -> Self {
        ParsePointError::Int(e)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean: String = s
            .matches(|c: char| c.is_numeric() || c == '-' || c == ' ')
            .collect();

        let mut it = clean.split_whitespace();
        let x: i32 = it.next().ok_or(ParsePointError::Empty)?.parse()?;
        let y: i32 = it.next().ok_or(ParsePointError::Empty)?.parse()?;
        let vx: i32 = it.next().ok_or(ParsePointError::Empty)?.parse()?;
        let vy: i32 = it.next().ok_or(ParsePointError::Empty)?.parse()?;

        Ok(Point { x, y, vx, vy })
    }
}

#[derive(Debug)]
struct Canvas {
    buf: Option<Vec<char>>,
    w: usize,
    h: usize,
    x_offset: i32,
    y_offset: i32,
}

impl Canvas {
    fn from_corners((min_x, min_y): (i32, i32), (max_x, max_y): (i32, i32)) -> Canvas {
        let x_offset = min_x as i32;
        let y_offset = min_y as i32;

        let w = (max_x - min_x + 1) as usize;
        let h = (max_y - min_y + 1) as usize;

        // 100 x 100 found by trial and error
        let buf = if w < 100 && h < 100 {
            Some(vec!['.'; w * h])
        } else {
            None
        };

        Canvas {
            buf,
            w,
            h,
            x_offset,
            y_offset,
        }
    }

    fn display(&self, s: usize) {
        if let Some(buf) = &self.buf {
            let lines = buf.chunks(self.w);

            println!("Second: {}\n", s);
            for line in lines {
                for c in line {
                    print!("{}", c);
                }
                println!();
            }
            println!();
        }
    }

    fn draw(&mut self, p: &Point) {
        if let Some(buf) = &mut self.buf {
            let x = (p.x - self.x_offset) as usize;
            let y = (p.y - self.y_offset) as usize;
            buf[(self.w * y) + x] = '#';
        }
    }
}

fn main() {
    let dat = fs::read_to_string("input").unwrap();
    let mut points: Vec<Point> = dat.lines().map(|x| x.parse().unwrap()).collect();

    // Trial and error bounds
    for s in 1..100000 {
        for point in &mut points {
            point.mv();
        }

        let min_corner = (
            points.iter().map(|p| p.x).min().unwrap(),
            points.iter().map(|p| p.y).min().unwrap(),
        );

        let max_corner = (
            points.iter().map(|p| p.x).max().unwrap(),
            points.iter().map(|p| p.y).max().unwrap(),
        );

        let mut c = Canvas::from_corners(min_corner, max_corner);

        for point in &points {
            c.draw(point, );
        }
        c.display(s);
    }
}
