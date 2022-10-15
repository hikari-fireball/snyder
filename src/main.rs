use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::ops::RangeInclusive;

// TODO move these constants to main()
const BLOCK_SIZE: usize = 2;
const BOARD_SIZE: usize = BLOCK_SIZE.pow(2);
const CELL_DOMAIN: RangeInclusive<usize> = 1..=BOARD_SIZE;

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

type Domain = HashSet<usize>;

#[derive(Debug, Clone)]
struct Board {
    // TODO add a reference to the list of variables
    variables: HashMap<Position, Domain>,
}

impl Board {
    fn new(variables: &[Position], domain: &Domain) -> Self {
        Board {
            variables: variables
                .iter()
                .map(|v| (*v, domain.clone()))
                .collect::<HashMap<Position, Domain>>(),
        }
    }

    fn is_valid(&self) -> bool {
        // TODO: imporove this code somehow
        for line in 0..BOARD_SIZE {
            let mut present: HashSet<usize> = HashSet::new();
            for column in 0..BOARD_SIZE {
                let cell = self.variables.get(&Position { line, column }).unwrap();
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
            let mut present: HashSet<usize> = HashSet::new();
            for line in 0..BOARD_SIZE {
                let cell = self.variables.get(&Position { line, column }).unwrap();
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

        for line_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE) {
            for column_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE) {
                let mut present: HashSet<usize> = HashSet::new();
                for line in line_offset..(line_offset + BLOCK_SIZE) {
                    for column in column_offset..(column_offset + BLOCK_SIZE) {
                        let cell = self.variables.get(&Position { line, column }).unwrap();
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

    fn most_constrained_variable(&self) -> Option<&Position> {
        self.variables
            .iter()
            .filter(|v| v.1.len() > 1)
            .min_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len()))
            .map(|(k, _)| k)
    }

    fn simplify(&mut self, position: &Position, value: usize) -> bool {
        // TODO: [IMPORTANT] call simplify again when a cell becomes determined: if cell.remove() and cell.len() == 1
        // TODO: return result instead
        // TODO write a function mut_adjacent to return an iterator to mutable adjacent
        // position/domains
        let left_bracket = (position.line / BLOCK_SIZE) * BLOCK_SIZE;
        let right_bracket = (position.line / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        let upper_bracket = (position.column / BLOCK_SIZE) * BLOCK_SIZE;
        let lower_bracket = (position.column / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        let adjacent = self.variables.iter_mut().filter(|(k, _)| {
            (k.line == position.line
                || k.column == position.column
                || ((k.line >= left_bracket && k.line < right_bracket)
                    && (k.column >= upper_bracket && k.column < lower_bracket)))
                && (k.line != position.line || k.column != position.column)
        });
        for (_, domain) in adjacent {
            domain.remove(&value);
            if domain.is_empty() {
                return false;
            }
        }
        true
    }

    fn offspring(&self) -> Vec<Board> {
        match self.most_constrained_variable() {
            Some(position) => {
                let mut children: Vec<Board> = vec![];
                for value in self.variables.get(position).unwrap() {
                    let mut child = self.clone();
                    let cell = child.variables.get_mut(position).unwrap();
                    cell.clear();
                    cell.insert(*value);
                    if child.simplify(position, *value) {
                        children.push(child);
                    }
                }
                children
            }
            None => Vec::new(),
        }
    }
}

fn main() {
    // TODO move this code out and leave only initialization and a function call
    println!(
        "sudoku {:?}/{:?} ({:?})",
        BLOCK_SIZE, BOARD_SIZE, CELL_DOMAIN
    );

    let variables: Vec<Position> = (0..BOARD_SIZE)
        .flat_map(|j| (0..BOARD_SIZE).map(move |k| Position { line: j, column: k }))
        .collect();
    let domain: Domain = CELL_DOMAIN.collect::<Domain>();

    let root = Board::new(&variables, &domain);
    let mut stack: Vec<Board> = vec![root];
    while let Some(parent) = stack.pop() {
        for child in parent.offspring() {
            if child.is_valid() {
                println!("{:?}", child);
            } else {
                stack.push(child);
            }
        }
    }
}
