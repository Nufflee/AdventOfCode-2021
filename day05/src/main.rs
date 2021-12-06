use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn slope(&self) -> f64 {
        (self.end.y as f64 - self.start.y as f64) / (self.end.x as f64 - self.start.x as f64)
    }

    fn is_diagonal(&self) -> bool {
        if self.is_horizontal() || self.is_vertical() {
            return false;
        }

        self.slope().abs() == 1.0
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

// TODO: Try to implement the mathematical solution
fn compute_intersections(lines: &[Line]) -> HashMap<Point, u32> {
    let mut points = HashMap::new();

    for line in lines {
        if line.is_diagonal() {
            let dx = (line.end.x - line.start.x).signum();
            let dy = (line.end.y - line.start.y).signum();

            for i in 0..(line.start.x - line.end.x).abs() + 1 {
                let x = line.start.x + i * dx;
                let y = line.start.y + i * dy;

                *points.entry(Point::new(x, y)).or_insert(0) += 1;
            }
        } else if line.is_horizontal() || line.is_vertical() {
            let (start_x, end_x) = if line.start.x > line.end.x {
                (line.end.x, line.start.x)
            } else {
                (line.start.x, line.end.x)
            };

            let (start_y, end_y) = if line.start.y > line.end.y {
                (line.end.y, line.start.y)
            } else {
                (line.start.y, line.end.y)
            };

            for x in start_x..end_x + 1 {
                for y in start_y..end_y + 1 {
                    let point = Point::new(x, y);

                    *points.entry(point).or_insert(0) += 1;
                }
            }
        }
    }

    points
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let input_lines = input.lines();

    let mut lines = Vec::new();

    for line in input_lines {
        let (start, end) = line.split_once("->").unwrap();

        let parse_point = |s: &str| -> Point {
            let (x, y) = s.split_once(",").unwrap();
            Point::new(x.trim().parse().unwrap(), y.trim().parse().unwrap())
        };

        lines.push(Line::new(parse_point(start), parse_point(end)));
    }

    let orthogonal_lines = lines
        .iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .copied()
        .collect::<Vec<_>>();

    let orthogonal_points = compute_intersections(&orthogonal_lines);

    println!("Part 1:");
    println!(
        "\t2 lines overlap at {:?} points",
        orthogonal_points
            .iter()
            .filter(|(_, count)| **count >= 2)
            .count()
    );

    let points = compute_intersections(&lines);

    println!("Part 2:");
    println!(
        "\t2 lines overlap at {:?} points",
        points.iter().filter(|(_, count)| **count >= 2).count()
    );
}
