mod csp;

use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::ops::RangeInclusive;

#[derive(Clone, Eq, Hash, PartialEq, Copy)]
struct Position {
    line: usize,
    column: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

// TODO move these constants somewhrere where they can be accessed by the Sudoku code
const BLOCK_SIZE: usize = 2;
const BOARD_SIZE: usize = BLOCK_SIZE.pow(2);
const CELL_DOMAIN: RangeInclusive<SudokuDomainValue> = 1..=(BOARD_SIZE as u32);

type SudokuDomainValue = u32;
type Sudoku = csp::State<Position, SudokuDomainValue>;

impl Sudoku {
    fn adjacent_mut(
        &mut self,
        position: &Position,
    ) -> Vec<(&Position, &mut HashSet<SudokuDomainValue>)> {
        // TODO modify function to return an iterator std::iter::Iterator<Item=(V, Vec<D>)>
        let left_bracket = (position.line / BLOCK_SIZE) * BLOCK_SIZE;
        let right_bracket = (position.line / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        let upper_bracket = (position.column / BLOCK_SIZE) * BLOCK_SIZE;
        let lower_bracket = (position.column / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        self.domains
            .iter_mut()
            .filter(|(k, _)| {
                (k.line == position.line
                    || k.column == position.column
                    || ((k.line >= left_bracket && k.line < right_bracket)
                        && (k.column >= upper_bracket && k.column < lower_bracket)))
                    && (k.line != position.line || k.column != position.column)
            })
            .collect::<Vec<(&Position, &mut HashSet<SudokuDomainValue>)>>()
    }
}

impl csp::Searchable<Position, SudokuDomainValue> for Sudoku {
    fn check_constraints(&self) -> bool {
        // TODO: if the csp library sets an invalid value, we never know and keep searching.
        // we are returning false if a cell is not determined or when values are repeated
        // we never return false if we find any other invalid thing
        // in other words, the csp module is exploring states that are invalid
        // could we return a result? invalid -> Error, complete -> true, incomplete -> false?
        // TODO: imporove this code somehow
        for line in 0..BOARD_SIZE {
            let mut present: HashSet<SudokuDomainValue> = HashSet::new();
            for column in 0..BOARD_SIZE {
                let cell = self.domains.get(&Position { line, column }).unwrap();
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
            let mut present: HashSet<SudokuDomainValue> = HashSet::new();
            for line in 0..BOARD_SIZE {
                let cell = self.domains.get(&Position { line, column }).unwrap();
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
        for line_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE as usize) {
            for column_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE as usize) {
                let mut present: HashSet<SudokuDomainValue> = HashSet::new();
                for line in line_offset..(line_offset + BLOCK_SIZE) {
                    for column in column_offset..(column_offset + BLOCK_SIZE) {
                        let cell = self.domains.get(&Position { line, column }).unwrap();
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

    fn simplify(&mut self, position: &Position, value: SudokuDomainValue) -> Result<(), ()> {
        // TODO: [IMPORTANT] call simplify again when a cell becomes determined: if cell.remove() and cell.len() == 1
        for (_, domain) in self.adjacent_mut(position) {
            domain.remove(&value);
            if domain.is_empty() {
                return Err(());
            }
        }
        Ok(())
    }
}

const NQUEEN_SIZE: usize = 4;

type Nqueens = csp::State<Position, bool>;
impl csp::Searchable<Position, bool> for Nqueens {
    fn check_constraints(&self) -> bool {
        // TODO: move this to csp
        if self.domains.iter().filter(|(_, v)| v.len() != 1).count() > 0 {
            return false;
        }

        let queen_positions: Vec<&Position> = self
            .domains
            .iter()
            .filter(|(_, v)| v.len() == 1 && v.contains(&true))
            .map(|(k, _)| k)
            .collect();

        if queen_positions.len() != NQUEEN_SIZE {
            return false;
        }
        for positions in queen_positions.iter().combinations(2) {
            let x1 = positions[0].line as i32;
            let y1 = positions[0].column as i32;
            let x2 = positions[1].line as i32;
            let y2 = positions[1].column as i32;
            if x1 == x2 || y1 == y2 || x1 - y1 == x2 - y2 || x1 + y1 == x2 + y2 {
                return false;
            }
        }

        true
    }

    fn simplify(&mut self, _variable: &Position, _value: bool) -> Result<(), ()> {
        Ok(())
    }
}

fn main() {
    println!("nqueens (dummy)",);
    let nqueens: Nqueens = Nqueens::new(
        &(0..NQUEEN_SIZE)
            .flat_map(|j| (0..NQUEEN_SIZE).map(move |k| Position { line: j, column: k }))
            .collect::<Vec<Position>>(),
        &HashSet::<bool>::from([true, false]),
    );
    csp::find_all(nqueens);

    println!(
        "sudoku {:?}/{:?} ({:?})",
        BLOCK_SIZE, BOARD_SIZE, CELL_DOMAIN
    );

    let sudoku = Sudoku::new(
        &(0..BOARD_SIZE)
            .flat_map(|j| (0..BOARD_SIZE).map(move |k| Position { line: j, column: k }))
            .collect::<Vec<Position>>(),
        &CELL_DOMAIN.collect(),
    );
    csp::find_all(sudoku);
}
