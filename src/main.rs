use std::collections::HashSet;
use std::ops::RangeInclusive;

const BLOCK_SIZE: usize = 2;
const BOARD_SIZE: usize = BLOCK_SIZE.pow(2);
const CELL_DOMAIN: RangeInclusive<usize> = 1..=BOARD_SIZE;

#[derive(Debug)]
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
}

fn main() {
    println!(
        "sudoku {:?}/{:?} ({:?})",
        BLOCK_SIZE, BOARD_SIZE, CELL_DOMAIN
    );
    // let mut stack: Vec<Board> = vec![Board::new()];
    // println!("{:?}", stack);

    let mut root = Board::new();
    root.cells[0][0] = HashSet::from([1]);
    root.cells[0][1] = HashSet::from([4]);
    root.cells[0][2] = HashSet::from([3]);
    root.cells[0][3] = HashSet::from([2]);
    root.cells[1][0] = HashSet::from([3]);
    root.cells[1][1] = HashSet::from([2]);
    root.cells[1][2] = HashSet::from([1]);
    root.cells[1][3] = HashSet::from([4]);
    root.cells[2][0] = HashSet::from([4]);
    root.cells[2][1] = HashSet::from([1]);
    root.cells[2][2] = HashSet::from([2]);
    root.cells[2][3] = HashSet::from([3]);
    root.cells[3][0] = HashSet::from([2]);
    root.cells[3][1] = HashSet::from([3]);
    root.cells[3][2] = HashSet::from([4]);
    root.cells[3][3] = HashSet::from([1]);
    println!("{:?}", root.is_valid());
}
