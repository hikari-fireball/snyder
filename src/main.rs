use std::collections::HashSet;
use std::ops::RangeInclusive;

const BLOCK_SIZE: usize = 2;
const BOARD_SIZE: usize = BLOCK_SIZE.pow(2);
const CELL_DOMAIN: RangeInclusive<usize> = 1..=BOARD_SIZE;

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Vec<HashSet<usize>>>,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: vec![vec![CELL_DOMAIN.collect(); BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn is_valid(&self) -> bool {
        for line in &self.cells {
            let mut present: HashSet<usize> = HashSet::new();
            for cell in line {
                if cell.len() != 1 {
                    return false;
                }
                let single = cell.iter().next().unwrap();
                if present.contains(single) {
                    return false;
                }
                present.insert(*single);
            }
        }

        for column in 0..BOARD_SIZE {
            let mut present: HashSet<usize> = HashSet::new();
            for line in 0..BOARD_SIZE {
                let cell = &self.cells[line][column];
                if cell.len() != 1 {
                    return false;
                }
                let single = cell.iter().next().unwrap();
                if present.contains(single) {
                    return false;
                }
                present.insert(*single);
            }
        }

        for line_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE) {
            for column_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE) {
                let mut present: HashSet<usize> = HashSet::new();
                for line in line_offset..(line_offset + BLOCK_SIZE) {
                    for column in column_offset..(column_offset + BLOCK_SIZE) {
                        let cell = &self.cells[line][column];
                        if cell.len() != 1 {
                            return false;
                        }
                        let single = cell.iter().next().unwrap();
                        if present.contains(single) {
                            return false;
                        }
                        present.insert(*single);
                    }
                }
            }
        }

        true
    }

    fn most_constrained_variable(&self) -> Option<(usize, usize)> {
        let domain_size = CELL_DOMAIN.collect::<Vec<usize>>().len();
        let (mut x, mut y, mut min) = (0, 0, domain_size + 1);
        for line in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                if self.cells[line][column].len() < min && self.cells[line][column].len() > 1 {
                    y = line;
                    x = column;
                    min = self.cells[line][column].len();
                }
            }
        }
        if min <= domain_size {
            Some((y, x))
        } else {
            None
        }
    }

    fn simplify(&mut self, line: usize, column: usize, value: usize) -> bool {
        for c in 0..BOARD_SIZE {
            if c != column {
                self.cells[line][c].remove(&value);
                if self.cells[line][c].len() == 0 {
                    return false;
                }
            }
        }

        for l in 0..BOARD_SIZE {
            if l != line {
                self.cells[l][column].remove(&value);
                if self.cells[l][column].len() == 0 {
                    return false;
                }
            }
        }

        for l in (line / BLOCK_SIZE) * BLOCK_SIZE..(line / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE {
            for c in
                (column / BLOCK_SIZE) * BLOCK_SIZE..(column / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE
            {
                if l != line || c != column {
                    self.cells[l][c].remove(&value);
                    if self.cells[l][c].len() == 0 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn offspring(&self) -> Vec<Board> {
        match self.most_constrained_variable() {
            Some((line, column)) => {
                let mut children: Vec<Board> = vec![];
                for value in self.cells[line][column].clone() {
                    let mut child = self.clone();
                    child.cells[line][column].clear();
                    child.cells[line][column].insert(value);
                    if child.simplify(line, column, value) {
                        children.push(child);
                    }
                }
                children
            }
            None => Vec::new(),
        }
    }
}

fn main() {
    println!(
        "sudoku {:?}/{:?} ({:?})",
        BLOCK_SIZE, BOARD_SIZE, CELL_DOMAIN
    );

    // let mut root = Board::new();
    // root.cells[0][0] = HashSet::from([1]);
    // root.cells[0][1] = HashSet::from([4]);
    // root.cells[0][2] = HashSet::from([3]);
    // root.cells[0][3] = HashSet::from([2]);
    // root.cells[1][0] = HashSet::from([3]);
    // root.cells[1][1] = HashSet::from([2]);
    // root.cells[1][2] = HashSet::from([1]);
    // root.cells[1][3] = HashSet::from([4]);
    // root.cells[2][0] = HashSet::from([4]);
    // root.cells[2][1] = HashSet::from([1]);
    // root.cells[2][2] = HashSet::from([2]);
    // root.cells[2][3] = HashSet::from([3]);
    // root.cells[3][0] = HashSet::from([2]);
    // root.cells[3][1] = HashSet::from([3]);
    // root.cells[3][2] = HashSet::from([4]);
    // root.cells[3][3] = HashSet::from([1]);
    // assert!(root.is_valid());

    let root = Board::new();
    let mut stack: Vec<Board> = vec![root];
    while let Some(parent) = stack.pop() {
        for child in parent.offspring() {
            if child.is_valid() {
                println!("{:?}", child);
            } else {
                stack.push(child);
            }
        }
    }
}
