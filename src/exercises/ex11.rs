use crate::helpers::file::{read, write};

pub fn ex11() {
    let e = "11";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

#[derive(PartialEq, Copy, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Seat {
    fn from(seat: char) -> Self {
        match seat {
            '#' => Self::Occupied,
            'L' => Self::Empty,
            _ => Self::Floor,
        }
    }
}

impl From<&Seat> for char {
    fn from(seat: &Seat) -> Self {
        match seat {
            Seat::Occupied => '#',
            Seat::Empty => 'L',
            Seat::Floor => '.',
        }
    }
}

struct WaitingArea {
    layout: Vec<Vec<Seat>>,
    rows: usize,
    cols: usize,
}

impl WaitingArea {
    fn new(layout: Vec<Vec<Seat>>) -> Self {
        Self {
            rows: layout.len(),
            cols: layout[0].len(),
            layout,
        }
    }

    fn adjacent(&self, row: usize, col: usize) -> Vec<Seat> {
        let mut seats = vec![];
        let moves = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (m_row, m_col) in moves.iter() {
            let new_row = row as isize + m_row;
            let new_col = col as isize + m_col;
            if new_row >= 0
                && new_row < self.rows as isize
                && new_col >= 0
                && new_col < self.cols as isize
            {
                seats.push(self.layout[new_row as usize][new_col as usize]);
            }
        }
        seats
    }

    fn step1(&mut self) -> bool {
        let mut has_changed = false;
        let mut new_layout = (0..self.rows)
            .map(|row| {
                (0..self.cols)
                    .map(|col| self.layout[row][col])
                    .collect::<Vec<Seat>>()
            })
            .collect::<Vec<Vec<Seat>>>();
        for row in 0..self.rows {
            for col in 0..self.cols {
                new_layout[row][col] = if self.layout[row][col] == Seat::Empty
                    && self
                        .adjacent(row, col)
                        .iter()
                        .filter(|seat| **seat == Seat::Occupied)
                        .count()
                        == 0
                {
                    has_changed = true;
                    Seat::Occupied
                } else if self.layout[row][col] == Seat::Occupied
                    && self
                        .adjacent(row, col)
                        .iter()
                        .filter(|seat| **seat == Seat::Occupied)
                        .count()
                        >= 4
                {
                    has_changed = true;
                    Seat::Empty
                } else {
                    self.layout[row][col]
                };
            }
        }
        self.layout = new_layout;
        has_changed
    }

    fn visible(&self, row: usize, col: usize) -> Vec<Seat> {
        let mut seats = vec![];
        let moves = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (m_row, m_col) in moves.iter() {
            let mut new_row = row as isize + m_row;
            let mut new_col = col as isize + m_col;
            loop {
                if new_row < 0
                    || new_row >= self.rows as isize
                    || new_col < 0
                    || new_col >= self.cols as isize
                {
                    break;
                } else if self.layout[new_row as usize][new_col as usize]
                    != Seat::Floor
                {
                    seats.push(self.layout[new_row as usize][new_col as usize]);
                    break;
                } else {
                    new_row += m_row;
                    new_col += m_col;
                }
            }
        }
        seats
    }

    fn step2(&mut self) -> bool {
        let mut has_changed = false;
        let mut new_layout = (0..self.rows)
            .map(|row| {
                (0..self.cols)
                    .map(|col| self.layout[row][col])
                    .collect::<Vec<Seat>>()
            })
            .collect::<Vec<Vec<Seat>>>();
        for row in 0..self.rows {
            for col in 0..self.cols {
                new_layout[row][col] = if self.layout[row][col] == Seat::Empty
                    && self
                        .visible(row, col)
                        .iter()
                        .filter(|seat| **seat == Seat::Occupied)
                        .count()
                        == 0
                {
                    has_changed = true;
                    Seat::Occupied
                } else if self.layout[row][col] == Seat::Occupied
                    && self
                        .visible(row, col)
                        .iter()
                        .filter(|seat| **seat == Seat::Occupied)
                        .count()
                        >= 5
                {
                    has_changed = true;
                    Seat::Empty
                } else {
                    self.layout[row][col]
                };
            }
        }
        self.layout = new_layout;
        has_changed
    }

    fn count_occupied(&self) -> usize {
        self.layout
            .iter()
            .map(|row| {
                row.iter().filter(|seat| **seat == Seat::Occupied).count()
            })
            .sum()
    }
}

impl From<&WaitingArea> for String {
    fn from(area: &WaitingArea) -> Self {
        let mut s = "".to_string();
        for row in area.layout.iter() {
            for seat in row.iter() {
                s.push(char::from(seat));
            }
            s.push_str("\n");
        }
        s
    }
}

impl From<&str> for WaitingArea {
    fn from(s: &str) -> Self {
        let layout = s
            .split("\n")
            .map(|row| {
                row.chars()
                    .map(|seat| Seat::from(seat))
                    .collect::<Vec<Seat>>()
            })
            .collect::<Vec<Vec<Seat>>>();
        Self::new(layout)
    }
}

pub fn sub1(s: &str) -> usize {
    let mut area = WaitingArea::from(s);
    while area.step1() {}
    area.count_occupied()
}

pub fn sub2(s: &str) -> usize {
    let mut area = WaitingArea::from(s);
    while area.step2() {}
    area.count_occupied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent() {
        let area = WaitingArea::from(
            "L.#
###
L.#",
        );
        assert_eq!(area.adjacent(0, 0).len(), 3);
        assert_eq!(area.adjacent(0, 1).len(), 5);
        assert_eq!(area.adjacent(0, 2).len(), 3);
        assert_eq!(area.adjacent(1, 0).len(), 5);
        assert_eq!(area.adjacent(1, 1).len(), 8);
        assert_eq!(area.adjacent(1, 2).len(), 5);
        assert_eq!(area.adjacent(2, 0).len(), 3);
        assert_eq!(area.adjacent(2, 1).len(), 5);
        assert_eq!(area.adjacent(2, 2).len(), 3);
    }

    #[test]
    fn sub11() {
        assert_eq!(
            sub1(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            ),
            37
        );
    }

    #[test]
    fn sub21() {
        assert_eq!(
            sub2(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            ),
            26
        );
    }
}
