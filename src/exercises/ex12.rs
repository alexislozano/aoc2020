use crate::helpers::file::{read, write};

pub fn ex12() {
    let e = "12";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

enum Op {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let (instruction, value) =
            (s.chars().nth(0).unwrap(), &s[1..].parse::<usize>().unwrap());
        match instruction {
            'N' => Self::North(*value),
            'S' => Self::South(*value),
            'E' => Self::East(*value),
            'W' => Self::West(*value),
            'L' => Self::Left(*value),
            'R' => Self::Right(*value),
            'F' => Self::Forward(*value),
            _ => unreachable!(),
        }
    }
}

enum Heading {
    North,
    South,
    West,
    East,
}

struct Boat1 {
    x: isize,
    y: isize,
    heading: Heading,
}

impl Boat1 {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: Heading::East,
        }
    }

    fn step(&mut self, op: Op) {
        match op {
            Op::North(value) => self.y += value as isize,
            Op::South(value) => self.y -= value as isize,
            Op::East(value) => self.x += value as isize,
            Op::West(value) => self.x -= value as isize,
            Op::Left(value) => {
                self.heading = match (&self.heading, value) {
                    (Heading::North, 0)
                    | (Heading::East, 90)
                    | (Heading::South, 180)
                    | (Heading::West, 270) => Heading::North,
                    (Heading::East, 0)
                    | (Heading::South, 90)
                    | (Heading::West, 180)
                    | (Heading::North, 270) => Heading::East,
                    (Heading::South, 0)
                    | (Heading::West, 90)
                    | (Heading::North, 180)
                    | (Heading::East, 270) => Heading::South,
                    (Heading::West, 0)
                    | (Heading::North, 90)
                    | (Heading::East, 180)
                    | (Heading::South, 270) => Heading::West,
                    _ => unreachable!(),
                }
            }
            Op::Right(value) => {
                self.heading = match (&self.heading, value) {
                    (Heading::North, 0)
                    | (Heading::West, 90)
                    | (Heading::South, 180)
                    | (Heading::East, 270) => Heading::North,
                    (Heading::East, 0)
                    | (Heading::North, 90)
                    | (Heading::West, 180)
                    | (Heading::South, 270) => Heading::East,
                    (Heading::South, 0)
                    | (Heading::East, 90)
                    | (Heading::North, 180)
                    | (Heading::West, 270) => Heading::South,
                    (Heading::West, 0)
                    | (Heading::South, 90)
                    | (Heading::East, 180)
                    | (Heading::North, 270) => Heading::West,
                    _ => unreachable!(),
                }
            }
            Op::Forward(value) => match &self.heading {
                Heading::North => self.y += value as isize,
                Heading::South => self.y -= value as isize,
                Heading::East => self.x += value as isize,
                Heading::West => self.x -= value as isize,
            },
        }
    }

    fn distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

struct Boat2 {
    x: isize,
    y: isize,
    waypoint: (isize, isize),
}

impl Boat2 {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint: (10, 1),
        }
    }

    fn step(&mut self, op: Op) {
        match op {
            Op::North(value) => self.waypoint.1 += value as isize,
            Op::South(value) => self.waypoint.1 -= value as isize,
            Op::East(value) => self.waypoint.0 += value as isize,
            Op::West(value) => self.waypoint.0 -= value as isize,
            Op::Left(value) => {
                self.waypoint = match value {
                    0 => (self.waypoint.0, self.waypoint.1),
                    90 => (-self.waypoint.1, self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (self.waypoint.1, -self.waypoint.0),
                    _ => unreachable!(),
                }
            }
            Op::Right(value) => {
                self.waypoint = match value {
                    0 => (self.waypoint.0, self.waypoint.1),
                    90 => (self.waypoint.1, -self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (-self.waypoint.1, self.waypoint.0),
                    _ => unreachable!(),
                }
            }
            Op::Forward(value) => {
                self.x += self.waypoint.0 * value as isize;
                self.y += self.waypoint.1 * value as isize;
            }
        }
    }

    fn distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

pub fn sub1(s: &str) -> usize {
    let mut boat = Boat1::new();
    for op in s.split("\n").map(|op| Op::from(op)) {
        boat.step(op);
    }
    boat.distance()
}

pub fn sub2(s: &str) -> usize {
    let mut boat = Boat2::new();
    for op in s.split("\n").map(|op| Op::from(op)) {
        boat.step(op);
    }
    boat.distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "F10
N3
F7
R90
F11"
            ),
            25
        )
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "F10
N3
F7
R90
F11"
            ),
            286
        )
    }
}
