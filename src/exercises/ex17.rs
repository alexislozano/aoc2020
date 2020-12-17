use crate::helpers::file::{read, write};
use std::collections::HashSet;

pub fn ex17() {
    let e = "17";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

struct Dimension3 {
    layout: HashSet<(isize, isize, isize)>,
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
    z_bounds: (isize, isize),
}

impl From<&str> for Dimension3 {
    fn from(s: &str) -> Self {
        let mut layout = HashSet::new();
        let s = s
            .split("\n")
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        for (y, row) in s.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    layout.insert((x as isize, y as isize, 0));
                }
            }
        }
        let x_bounds = (0, (s[0].len() - 1) as isize);
        let y_bounds = (0, (s.len() - 1) as isize);
        let z_bounds = (0, 0);
        Self {
            layout,
            x_bounds,
            y_bounds,
            z_bounds,
        }
    }
}

impl Dimension3 {
    fn adjacents(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0;
        let moves = [
            (-1, -1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, 0, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 1, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            (0, -1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, 0, -1),
            (0, 0, 1),
            (0, 1, -1),
            (0, 1, 0),
            (0, 1, 1),
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, 0, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
        ];
        for (m_x, m_y, m_z) in moves.iter() {
            count += if self.layout.contains(&(x + m_x, y + m_y, z + m_z)) {
                1
            } else {
                0
            }
        }
        count
    }

    fn cycle(&mut self) {
        let mut new_layout = HashSet::new();
        self.x_bounds = (self.x_bounds.0 - 1, self.x_bounds.1 + 1);
        self.y_bounds = (self.y_bounds.0 - 1, self.y_bounds.1 + 1);
        self.z_bounds = (self.z_bounds.0 - 1, self.z_bounds.1 + 1);
        let mut x = self.x_bounds.0;
        let mut y;
        let mut z;
        while x <= self.x_bounds.1 {
            y = self.y_bounds.0;
            while y <= self.y_bounds.1 {
                z = self.y_bounds.0;
                while z <= self.z_bounds.1 {
                    match (
                        self.layout.contains(&(x, y, z)),
                        self.adjacents(x, y, z),
                    ) {
                        (true, 2) | (true, 3) | (false, 3) => {
                            new_layout.insert((x, y, z));
                        }
                        _ => {}
                    }
                    z += 1;
                }
                y += 1;
            }
            x += 1;
        }
        self.layout = new_layout;
    }

    fn count_actives(&self) -> usize {
        self.layout.len()
    }
}

struct Dimension4 {
    layout: HashSet<(isize, isize, isize, isize)>,
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
    z_bounds: (isize, isize),
    w_bounds: (isize, isize),
}

impl From<&str> for Dimension4 {
    fn from(s: &str) -> Self {
        let mut layout = HashSet::new();
        let s = s
            .split("\n")
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        for (y, row) in s.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    layout.insert((x as isize, y as isize, 0, 0));
                }
            }
        }
        let x_bounds = (0, (s[0].len() - 1) as isize);
        let y_bounds = (0, (s.len() - 1) as isize);
        let z_bounds = (0, 0);
        let w_bounds = (0, 0);
        Self {
            layout,
            x_bounds,
            y_bounds,
            z_bounds,
            w_bounds,
        }
    }
}

impl Dimension4 {
    fn adjacents(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0;
        let moves = [
            (-1, -1, -1, -1),
            (-1, -1, -1, 0),
            (-1, -1, -1, 1),
            (-1, -1, 0, -1),
            (-1, -1, 0, 0),
            (-1, -1, 0, 1),
            (-1, -1, 1, -1),
            (-1, -1, 1, 0),
            (-1, -1, 1, 1),
            (-1, 0, -1, -1),
            (-1, 0, -1, 0),
            (-1, 0, -1, 1),
            (-1, 0, 0, -1),
            (-1, 0, 0, 0),
            (-1, 0, 0, 1),
            (-1, 0, 1, -1),
            (-1, 0, 1, 0),
            (-1, 0, 1, 1),
            (-1, 1, -1, -1),
            (-1, 1, -1, 0),
            (-1, 1, -1, 1),
            (-1, 1, 0, -1),
            (-1, 1, 0, 0),
            (-1, 1, 0, 1),
            (-1, 1, 1, -1),
            (-1, 1, 1, 0),
            (-1, 1, 1, 1),
            (0, -1, -1, -1),
            (0, -1, -1, 0),
            (0, -1, -1, 1),
            (0, -1, 0, -1),
            (0, -1, 0, 0),
            (0, -1, 0, 1),
            (0, -1, 1, -1),
            (0, -1, 1, 0),
            (0, -1, 1, 1),
            (0, 0, -1, -1),
            (0, 0, -1, 0),
            (0, 0, -1, 1),
            (0, 0, 0, -1),
            (0, 0, 0, 1),
            (0, 0, 1, -1),
            (0, 0, 1, 0),
            (0, 0, 1, 1),
            (0, 1, -1, -1),
            (0, 1, -1, 0),
            (0, 1, -1, 1),
            (0, 1, 0, -1),
            (0, 1, 0, 0),
            (0, 1, 0, 1),
            (0, 1, 1, -1),
            (0, 1, 1, 0),
            (0, 1, 1, 1),
            (1, -1, -1, -1),
            (1, -1, -1, 0),
            (1, -1, -1, 1),
            (1, -1, 0, -1),
            (1, -1, 0, 0),
            (1, -1, 0, 1),
            (1, -1, 1, -1),
            (1, -1, 1, 0),
            (1, -1, 1, 1),
            (1, 0, -1, -1),
            (1, 0, -1, 0),
            (1, 0, -1, 1),
            (1, 0, 0, -1),
            (1, 0, 0, 0),
            (1, 0, 0, 1),
            (1, 0, 1, -1),
            (1, 0, 1, 0),
            (1, 0, 1, 1),
            (1, 1, -1, -1),
            (1, 1, -1, 0),
            (1, 1, -1, 1),
            (1, 1, 0, -1),
            (1, 1, 0, 0),
            (1, 1, 0, 1),
            (1, 1, 1, -1),
            (1, 1, 1, 0),
            (1, 1, 1, 1),
        ];
        for (m_x, m_y, m_z, m_w) in moves.iter() {
            count +=
                if self.layout.contains(&(x + m_x, y + m_y, z + m_z, w + m_w)) {
                    1
                } else {
                    0
                }
        }
        count
    }

    fn cycle(&mut self) {
        let mut new_layout = HashSet::new();
        self.x_bounds = (self.x_bounds.0 - 1, self.x_bounds.1 + 1);
        self.y_bounds = (self.y_bounds.0 - 1, self.y_bounds.1 + 1);
        self.z_bounds = (self.z_bounds.0 - 1, self.z_bounds.1 + 1);
        self.w_bounds = (self.w_bounds.0 - 1, self.w_bounds.1 + 1);
        let mut x = self.x_bounds.0;
        let mut y;
        let mut z;
        let mut w;
        while x <= self.x_bounds.1 {
            y = self.y_bounds.0;
            while y <= self.y_bounds.1 {
                z = self.z_bounds.0;
                while z <= self.z_bounds.1 {
                    w = self.w_bounds.0;
                    while w <= self.w_bounds.1 {
                        match (
                            self.layout.contains(&(x, y, z, w)),
                            self.adjacents(x, y, z, w),
                        ) {
                            (true, 2) | (true, 3) | (false, 3) => {
                                new_layout.insert((x, y, z, w));
                            }
                            _ => {}
                        }
                        w += 1
                    }
                    z += 1;
                }
                y += 1;
            }
            x += 1;
        }
        self.layout = new_layout;
    }

    fn count_actives(&self) -> usize {
        self.layout.len()
    }
}

pub fn sub1(s: &str) -> usize {
    let mut dimension = Dimension3::from(s);
    for _ in 0..6 {
        dimension.cycle();
    }
    dimension.count_actives()
}

pub fn sub2(s: &str) -> usize {
    let mut dimension = Dimension4::from(s);
    for _ in 0..6 {
        dimension.cycle();
    }
    dimension.count_actives()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                ".#.
..#
###"
            ),
            112
        );
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                ".#.
..#
###"
            ),
            848
        );
    }
}
