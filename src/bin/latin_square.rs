extern crate snyder;

use std::collections::HashSet;

// Define the variable type
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

// Define the domain type
type Number = char;

// Define the problem using variable and domain type
type LatinSquare = snyder::State<Position, Number>;

impl snyder::Searchable<Position, Number> for LatinSquare {
    // Check for the problem constraints
    fn check_constraints(&self, position: &Position, number: Number) -> bool {
        // there currently placed number is different from all other numbers on the same line or
        // column
        !self
            .determined()
            .any(|(k, v)| k != position && (k.x == position.x || k.y == position.y) && *v == number)
    }

    // [OPTIONAL] Simplify the state after the system has determined a value
    fn simplify(&mut self, position: &Position, number: Number) {
        // the currently placed number removes the possibilty of an identiccal value on the same
        // line or column
        for (_, number_set) in self.undetermined_mut().filter(|(k, v)| {
            *k != position && (k.x == position.x || k.y == position.y) && v.contains(&number)
        }) {
            number_set.remove(&number);
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
    let domain_set = HashSet::from(['A', 'B', 'C']);
    // Iterate the solutions
    let latin_square = LatinSquare::new(&variable_set, &domain_set);
    for state in latin_square.solution_iter() {
        println!("{state:?}");
    }
}
