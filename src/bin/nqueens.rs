extern crate snyder;

use std::collections::HashSet;
use std::hash::Hash;

const BOARD_SIZE: usize = 8;

type NQueens = snyder::State<Position, bool>;

#[derive(Clone, Eq, Hash, PartialEq, Copy, Debug)]
struct Position {
    line: usize,
    column: usize,
}

impl snyder::Searchable<Position, bool> for NQueens {
    fn check_constraints(&self, position: &Position, value: bool) -> bool {
        match value {
            true => {
                // there are no more than SIZE queens
                if self.determined().filter(|(_, v)| **v).count() > BOARD_SIZE {
                    return false;
                }
                // the new queen is not in check
                if self.determined().any(|(k, v)| {
                    *v && k != position
                        && (k.line == position.line
                            || k.column == position.column
                            || k.line as i32 - k.column as i32
                                == position.line as i32 - position.column as i32
                            || k.line + k.column == position.line + position.column)
                }) {
                    return false;
                }
            }
            false => {
                // there are no more than BOARD_SIZE² - BOARD_SIZE empty squares
                if self.determined().filter(|(_, v)| !(**v)).count()
                    > BOARD_SIZE * BOARD_SIZE - BOARD_SIZE
                {
                    return false;
                }
            }
        }
        true
    }

    fn simplify(&mut self, position: &Position, value: bool) {
        if value {
            for (_, value_set) in self.undetermined_mut().filter(|(k, _)| {
                k.line == position.line
                    || k.column == position.column
                    || k.line as i32 - k.column as i32
                        == position.line as i32 - position.column as i32
                    || k.line + k.column == position.line + position.column
            }) {
                value_set.remove(&value);
            }
        }
    }
}

fn main() {
    let variables = &(0..BOARD_SIZE)
        .flat_map(|j| (0..BOARD_SIZE).map(move |k| Position { line: j, column: k }))
        .collect::<Vec<Position>>();
    let domain = &HashSet::<bool>::from([true, false]);
    let nqueens: NQueens = NQueens::new(variables, domain);
    for state in nqueens.solution_iter() {
        println!("{state:?}");
    }
}
