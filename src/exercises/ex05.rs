use crate::helpers::file::{read, write};

pub fn ex05() {
    let e = "05";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

struct Seat {
    row: u8,
    col: u8,
}

impl From<&str> for Seat {
    fn from(s: &str) -> Self {
        let mut row_min = 0;
        let mut row_max = 127;
        for l in s.chars().take(7) {
            match l {
                'F' => row_max -= (row_max - row_min + 1) / 2,
                'B' => row_min += (row_max - row_min + 1) / 2,
                _ => unreachable!(),
            }
        }
        let mut col_min = 0;
        let mut col_max = 7;
        for l in s.chars().skip(7) {
            match l {
                'L' => col_max -= (col_max - col_min + 1) / 2,
                'R' => col_min += (col_max - col_min + 1) / 2,
                _ => unreachable!(),
            }
        }
        Seat {
            row: row_min,
            col: col_min,
        }
    }
}

impl Seat {
    fn id(&self) -> usize {
        self.row as usize * 8 + self.col as usize
    }
}

pub fn sub1(s: &str) -> usize {
    s.split("\n").map(|p| Seat::from(p).id()).max().unwrap()
}

pub fn sub2(s: &str) -> usize {
    let ids = s
        .split("\n")
        .map(|p| Seat::from(p).id())
        .collect::<Vec<usize>>();
    (0..1024usize)
        .filter(|i| {
            if ids.contains(i) {
                false
            } else {
                (if *i == 0 {
                    false
                } else {
                    ids.contains(&(i - 1))
                }) && (if *i == ids.len() - 1 {
                    false
                } else {
                    ids.contains(&(i + 1))
                })
            }
        })
        .nth(0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat1() {
        assert_eq!(Seat::from("FBFBBFFRLR").id(), 357);
    }

    #[test]
    fn seat2() {
        assert_eq!(Seat::from("BFFFBBFRRR").id(), 567);
    }

    #[test]
    fn seat3() {
        assert_eq!(Seat::from("FFFBBBFRRR").id(), 119);
    }

    #[test]
    fn seat4() {
        assert_eq!(Seat::from("BBFFBBFRLL").id(), 820);
    }
}
