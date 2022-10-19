extern crate snyder;

use std::collections::HashSet;
use std::hash::Hash;

type NQueens = snyder::State<Position, bool>;

#[derive(Clone, Eq, Hash, PartialEq, Copy, Debug)]
struct Position {
    line: usize,
    column: usize,
}

trait NQueenMeta {
    const SIZE: usize = 4;
}

impl NQueenMeta for NQueens {}

impl snyder::Searchable<Position, bool> for NQueens {
    fn check_constraints(&self, position: &Position, value: bool) -> bool {
        match value {
            true => {
                if self
                    .domains
                    .iter()
                    .filter(|(_, v)| v.len() == 1 && v.contains(&true))
                    .count()
                    > NQueens::SIZE
                {
                    return false;
                }
                if self.domains.iter().any(|(k, v)| {
                    v.len() == 1
                        && v.contains(&true)
                        && (k.line != position.line || k.column != position.column)
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
                if self
                    .domains
                    .iter()
                    .filter(|(_, v)| v.len() == 1 && v.contains(&false))
                    .count()
                    > NQueens::SIZE * NQueens::SIZE - NQueens::SIZE
                {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let variables = &(0..NQueens::SIZE)
        .flat_map(|j| (0..NQueens::SIZE).map(move |k| Position { line: j, column: k }))
        .collect::<Vec<Position>>();
    let domain = &HashSet::<bool>::from([true, false]);
    let nqueens: NQueens = NQueens::new(variables, domain);
    for state in nqueens.solution_iter() {
        println!("{state:?}");
    }
}
