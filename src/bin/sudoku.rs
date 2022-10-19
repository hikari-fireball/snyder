extern crate snyder;

use std::hash::Hash;
use std::ops::RangeInclusive;

type Sudoku = snyder::State<Position, Domain>;

#[derive(Clone, Eq, Hash, PartialEq, Copy, Debug)]
struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn is_adjacent(&self, other: &Position) -> bool {
        let left_bracket = (self.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let right_bracket =
            (self.line / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;
        let upper_bracket = (self.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE;
        let lower_bracket =
            (self.column / Sudoku::BLOCK_SIZE) * Sudoku::BLOCK_SIZE + Sudoku::BLOCK_SIZE;

        (other.line != self.line || other.column != self.column)
            && (other.line == self.line
                || other.column == self.column
                || ((other.line >= left_bracket && other.line < right_bracket)
                    && (other.column >= upper_bracket && other.column < lower_bracket)))
    }
}

type Domain = u32;

trait SudokuExtra {
    const BLOCK_SIZE: usize = 2;
    const BOARD_SIZE: usize = Sudoku::BLOCK_SIZE.pow(2);
    const CELL_DOMAIN: RangeInclusive<Domain> = 1..=(Sudoku::BOARD_SIZE as Domain);
}

impl SudokuExtra for Sudoku {}

impl snyder::Searchable<Position, Domain> for Sudoku {
    fn check_constraints(&self, position: &Position, value: Domain) -> bool {
        if self
            .iter()
            .any(|(k, v)| k.is_adjacent(position) && v.len() == 1 && v.contains(&value))
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
        for (_, domain) in self.iter_mut().filter(|(k, _)| k.is_adjacent(position)) {
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
