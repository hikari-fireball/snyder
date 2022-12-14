extern crate snyder;

use std::hash::Hash;
use std::ops::RangeInclusive;

const BLOCK_SIZE: usize = 3;
const BOARD_SIZE: usize = BLOCK_SIZE.pow(2);
const CELL_DOMAIN: RangeInclusive<Domain> = 1..=(BOARD_SIZE as Domain);

type Sudoku = snyder::State<Position, Domain>;

#[derive(Clone, Eq, Hash, PartialEq, Copy, Debug)]
struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn is_adjacent(&self, other: &Position) -> bool {
        let left_bracket = (self.line / BLOCK_SIZE) * BLOCK_SIZE;
        let right_bracket = (self.line / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        let upper_bracket = (self.column / BLOCK_SIZE) * BLOCK_SIZE;
        let lower_bracket = (self.column / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;

        (other.line != self.line || other.column != self.column)
            && (other.line == self.line
                || other.column == self.column
                || ((other.line >= left_bracket && other.line < right_bracket)
                    && (other.column >= upper_bracket && other.column < lower_bracket)))
    }
}

type Domain = u32;

impl snyder::Searchable<Position, Domain> for Sudoku {
    fn check_constraints(&self, position: &Position, value: Domain) -> bool {
        if self
            .determined()
            .any(|(k, v)| k.is_adjacent(position) && *v == value)
        {
            return false;
        }
        true
    }

    fn simplify(&mut self, position: &Position, value: Domain) {
        for (_, value_set) in self
            .undetermined_mut()
            .filter(|(k, _)| k.is_adjacent(position))
        {
            value_set.remove(&value);
        }
    }
}

fn main() {
    let variables = &(0..BOARD_SIZE)
        .flat_map(|j| (0..BOARD_SIZE).map(move |k| Position { line: j, column: k }))
        .collect::<Vec<Position>>();
    let domain = &CELL_DOMAIN.collect();
    let sudoku = Sudoku::new(variables, domain);
    for state in sudoku.solution_iter() {
        println!("{state:?}");
    }
}
