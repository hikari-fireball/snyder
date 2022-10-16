extern crate snyder;

use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

type NQueens = snyder::State<Position, bool>;

#[derive(Clone, Eq, Hash, PartialEq, Copy, Debug)] // TODO is there a way to define all these traits somewhere else as a set?
struct Position {
    line: usize,
    column: usize,
}

trait NQueenMeta {
    const SIZE: usize = 4;
}

impl NQueenMeta for NQueens {}

impl snyder::Searchable<Position, bool> for NQueens {
    fn check_constraints(&self) -> bool {
        // TODO: move this check to snyder?
        if self.domains.iter().filter(|(_, v)| v.len() != 1).count() > 0 {
            return false;
        }
        let queen_positions: Vec<&Position> = self
            .domains
            .iter()
            .filter(|(_, v)| v.len() == 1 && v.contains(&true))
            .map(|(k, _)| k)
            .collect();
        if queen_positions.len() != NQueens::SIZE {
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

    fn simplify(
        &mut self,
        _variable: &Position,
        _value: bool,
    ) -> Result<(), snyder::InvalidStateError> {
        Ok(())
    }
}

fn main() {
    let variables = &(0..NQueens::SIZE)
        .flat_map(|j| (0..NQueens::SIZE).map(move |k| Position { line: j, column: k }))
        .collect::<Vec<Position>>();
    let domain = &HashSet::<bool>::from([true, false]);
    let nqueens: NQueens = NQueens::new(variables, domain);
    snyder::find_all(nqueens);
}
