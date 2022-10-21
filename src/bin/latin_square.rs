extern crate snyder;

use std::collections::HashSet;

// Define the variable type
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

// Define the domain type
type Symbol = u32;

// Define the problem using variable and domain type
type LatinSquare = snyder::State<Position, Symbol>;

impl snyder::Searchable<Position, Symbol> for LatinSquare {
    // Check for the problem constraints
    fn check_constraints(&self, position: &Position, symbol: Symbol) -> bool {
        // the currently placed symbol is different from all other symbol on the same line or
        // column
        !self
            .determined()
            .any(|(k, v)| k != position && (k.x == position.x || k.y == position.y) && *v == symbol)
    }

    // [OPTIONAL] Simplify the state after the system has determined a value
    fn simplify(&mut self, position: &Position, symbol: Symbol) {
        // the currently placed symbol removes the possibilty of an identiccal value on the same
        // line or column
        for (_, symbol_set) in self.undetermined_mut().filter(|(k, v)| {
            *k != position && (k.x == position.x || k.y == position.y) && v.contains(&symbol)
        }) {
            symbol_set.remove(&symbol);
        }
    }
}

fn main() {
    // Define the set of variables
    let variable_set = [
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 0, y: 2 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: 1 },
        Position { x: 1, y: 2 },
        Position { x: 2, y: 0 },
        Position { x: 2, y: 1 },
        Position { x: 2, y: 2 },
    ];
    // Define the set of domains
    let domain_set = HashSet::from([1, 2, 3]);
    // Iterate the solutions
    let latin_square = LatinSquare::new(&variable_set, &domain_set);
    for state in latin_square.solution_iter() {
        println!("{state:?}");
    }
}
