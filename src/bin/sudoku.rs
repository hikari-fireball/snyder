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
    const BLOCK_SIZE: usize = 3;
    const BOARD_SIZE: usize = Sudoku::BLOCK_SIZE.pow(2);
    const CELL_DOMAIN: RangeInclusive<Domain> = 1..=(Sudoku::BOARD_SIZE as Domain);

    fn adjacent<'a>(
        &'a self,
        position: &'a Position,
    ) -> Box<dyn Iterator<Item = (&Position, &HashSet<Domain>)> + 'a>;
    fn adjacent_mut<'a>(
        &'a mut self,
        position: &'a Position,
    ) -> Box<dyn Iterator<Item = (&Position, &mut HashSet<Domain>)> + 'a>;
}

impl SudokuExtra for Sudoku {
    fn adjacent<'a>(
        &'a self,
        position: &'a Position,
    ) -> Box<dyn Iterator<Item = (&Position, &HashSet<Domain>)> + 'a> {
        let left_bracket = (position.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let right_bracket =
            (position.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        let upper_bracket = (position.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let lower_bracket =
            (position.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        Box::new(self.domains.iter().filter(move |(k, _)| {
            (k.line != position.line || k.column != position.column)
                && (k.line == position.line
                    || k.column == position.column
                    || ((k.line >= left_bracket && k.line < right_bracket)
                        && (k.column >= upper_bracket && k.column < lower_bracket)))
        }))
    }

    fn adjacent_mut<'a>(
        &'a mut self,
        position: &'a Position,
    ) -> Box<dyn Iterator<Item = (&Position, &mut HashSet<Domain>)> + 'a> {
        let left_bracket = (position.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let right_bracket =
            (position.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        let upper_bracket = (position.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let lower_bracket =
            (position.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        Box::new(self.domains.iter_mut().filter(move |(k, _)| {
            (k.line != position.line || k.column != position.column)
                && (k.line == position.line
                    || k.column == position.column
                    || ((k.line >= left_bracket && k.line < right_bracket)
                        && (k.column >= upper_bracket && k.column < lower_bracket)))
        }))
    }
}

impl snyder::Searchable<Position, Domain> for Sudoku {
    fn check_constraints(&self, position: &Position, value: Domain) -> bool {
        if self
            .adjacent(position)
            .any(|(_, v)| v.len() == 1 && v.contains(&value))
        {
            return false;
        }
        true
    }

    fn simplify(
        &mut self,
        position: &Position,
        value: Domain,
    ) -> Result<(), snyder::InvalidStateError> {
        // TODO maybe also find a way to delete adjacent_mut too
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
    for state in sudoku.solution_iter() {
        println!("{state:?}");
    }
}
