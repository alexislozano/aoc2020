use crate::helpers::file::{read, write};

pub fn ex03() {
    let e = "03";
    let s = read(e);
    write(e, &sub1(&s, 3, 1).to_string(), &sub2(&s).to_string());
}

enum Cell {
    Empty,
    Tree,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if c == '#' {
            Self::Tree
        } else {
            Self::Empty
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        let width = cells[0].len();
        let height = cells.len();
        Self {
            cells,
            width,
            height,
        }
    }

    fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x % self.width]
    }

    fn traverse(&self, move_x: usize, move_y: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut n = 0;
        while y < self.height {
            n += match self.cell(x, y) {
                Cell::Tree => 1,
                Cell::Empty => 0,
            };
            x += move_x;
            y += move_y;
        }
        n
    }
}

pub fn sub1(s: &str, x: usize, y: usize) -> usize {
    let cells = s
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|cell| Cell::from(cell))
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();
    let grid = Grid::new(cells);
    grid.traverse(x, y)
}

pub fn sub2(s: &str) -> usize {
    let cells = s
        .split("\n")
        .map(|row| {
            row.chars()
                .map(|cell| Cell::from(cell))
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();
    let grid = Grid::new(cells);
    grid.traverse(1, 1)
        * grid.traverse(3, 1)
        * grid.traverse(5, 1)
        * grid.traverse(7, 1)
        * grid.traverse(1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#", 1, 1), 2);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#", 3, 1), 7);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#", 5, 1), 3);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#", 7, 1), 4);
    }

    #[test]
    fn sub15() {
        assert_eq!(sub1("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#", 1, 2), 2);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#"), 336);
    }
}
