extern crate snyder;

use std::collections::HashSet;
use std::hash::Hash;
use std::ops::RangeInclusive;

type Sudoku = snyder::State<Position, Domain>;

#[derive(Clone, Eq, Hash, PartialEq, Copy, Debug)]
struct Position {
    line: usize,
    column: usize,
}

type Domain = u32;

trait SudokuExtra {
    const BLOCK_SIZE: usize = 2;
    const BOARD_SIZE: usize = Sudoku::BLOCK_SIZE.pow(2);
    const CELL_DOMAIN: RangeInclusive<Domain> = 1..=(Sudoku::BOARD_SIZE as Domain);

    fn adjacent_mut(&mut self, position: &Position) -> Vec<(&Position, &mut HashSet<Domain>)>;
    fn iter(&self) -> std::collections::hash_map::Iter<'_, Position, HashSet<Domain>>;
    fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, Position, HashSet<Domain>>;
    fn domains_iter(&self) -> Box<dyn Iterator<Item = &HashSet<Domain>> + '_>;
}

impl SudokuExtra for Sudoku {
    fn iter(&self) -> std::collections::hash_map::Iter<'_, Position, HashSet<Domain>> {
        self.domains.iter()
    }
    fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, Position, HashSet<Domain>> {
        self.domains.iter_mut()
    }
    fn domains_iter(&self) -> Box<dyn Iterator<Item = &HashSet<Domain>> + '_> {
        Box::new(self.domains.iter().map(|(_, v)| v))
    }

    fn adjacent_mut(&mut self, position: &Position) -> Vec<(&Position, &mut HashSet<Domain>)> {
        // TODO modify function to return an iterator std::iter::Iterator<Item=(V, Vec<D>)>
        let left_bracket = (position.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let right_bracket =
            (position.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        let upper_bracket = (position.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let lower_bracket =
            (position.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        self.domains
            .iter_mut()
            .filter(|(k, _)| {
                (k.line == position.line
                    || k.column == position.column
                    || ((k.line >= left_bracket && k.line < right_bracket)
                        && (k.column >= upper_bracket && k.column < lower_bracket)))
                    && (k.line != position.line || k.column != position.column)
            })
            .collect::<Vec<(&Position, &mut HashSet<Domain>)>>()
    }
}

impl snyder::Searchable<Position, Domain> for Sudoku {
    fn check_constraints(&self) -> bool {
        // TODO: if the snyder library sets an invalid value, we never know and keep searching.
        // we are returning false if a cell is not determined or when values are repeated
        // we never return false if we find any other invalid thing
        // in other words, the snyder module is exploring states that are invalid
        // could we return a result? invalid -> Error, complete -> true, incomplete -> false?
        // TODO: imporove this code somehow
        for line in 0..Sudoku::BOARD_SIZE {
            let mut present: HashSet<Domain> = HashSet::new();
            for column in 0..Sudoku::BOARD_SIZE {
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
        for column in 0..Sudoku::BOARD_SIZE {
            let mut present: HashSet<Domain> = HashSet::new();
            for line in 0..Sudoku::BOARD_SIZE {
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
        for line_offset in (0..Sudoku::BOARD_SIZE).step_by(Sudoku::BLOCK_SIZE as usize) {
            for column_offset in (0..Sudoku::BOARD_SIZE).step_by(Sudoku::BLOCK_SIZE as usize) {
                let mut present: HashSet<Domain> = HashSet::new();
                for line in line_offset..(line_offset + Sudoku::BLOCK_SIZE) {
                    for column in column_offset..(column_offset + Sudoku::BLOCK_SIZE) {
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

    fn simplify(
        &mut self,
        position: &Position,
        value: Domain,
    ) -> Result<(), snyder::InvalidStateError> {
        // TODO: [IMPORTANT] call simplify again when a cell becomes determined: if cell.remove() and cell.len() == 1
        for (_, domain) in self.adjacent_mut(position) {
            domain.remove(&value);
            if domain.is_empty() {
                return Err(snyder::InvalidStateError);
            }
        }
        Ok(())
    }
}

fn main() {
    let variables = &(0..Sudoku::BOARD_SIZE)
        .flat_map(|j| (0..Sudoku::BOARD_SIZE).map(move |k| Position { line: j, column: k }))
        .collect::<Vec<Position>>();
    let domain = &Sudoku::CELL_DOMAIN.collect();
    let sudoku = Sudoku::new(variables, domain);
    snyder::find_all(sudoku);
}
