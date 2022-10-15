use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::ops::RangeInclusive;

// TODO export State related to it's one namespace

#[derive(Debug, Clone)]
struct State<V, D> {
    domains: HashMap<V, HashSet<D>>,
}

impl<V, D> State<V, D>
where
    State<V, D>: Searchable<V, D> + Clone,
    V: Eq + Hash + Copy,
    D: Eq + Hash + Copy,
{
    fn new(variables: &[V], domain: &HashSet<D>) -> State<V, D> {
        State {
            domains: variables.iter().map(|v| (*v, domain.clone())).collect(),
        }
    }

    fn most_constrained_variable(&self) -> Option<&V> {
        self.domains
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .min_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len()))
            .map(|(k, _)| k)
    }

    fn offspring(&self) -> Vec<State<V, D>> {
        match self.most_constrained_variable() {
            Some(position) => {
                let mut children: Vec<State<V, D>> = vec![];
                for value in self.domains.get(position).unwrap() {
                    let mut child = self.clone();
                    let cell = child.domains.get_mut(position).unwrap();
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

pub trait Searchable<V, D> {
    fn check_constraints(&self) -> bool;
    fn simplify(&mut self, variable: &V, value: D) -> bool; // TODO modify to return a Result
}

// --- //

#[derive(Clone, Eq, Hash, PartialEq, Copy)]
struct Position {
    line: u32,
    column: u32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

// TODO move these constants somewhrere where they can be accessed by the Sudoku code
const BLOCK_SIZE: u32 = 3; // TODO make usize
const BOARD_SIZE: u32 = BLOCK_SIZE.pow(2); // TODO make usize
const CELL_DOMAIN: RangeInclusive<SudokuDomainValue> = 1..=BOARD_SIZE;

type SudokuDomainValue = u32;
type Sudoku = State<Position, SudokuDomainValue>;
impl Sudoku {
    fn adjacent_mut(
        &mut self,
        position: &Position,
    ) -> Vec<(&Position, &mut HashSet<SudokuDomainValue>)> {
        // TODO modify function to return an iterator std::iter::Iterator<Item=(V, Vec<D>)>
        let left_bracket = (position.line / BLOCK_SIZE) * BLOCK_SIZE;
        let right_bracket = (position.line / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        let upper_bracket = (position.column / BLOCK_SIZE) * BLOCK_SIZE;
        let lower_bracket = (position.column / BLOCK_SIZE) * BLOCK_SIZE + BLOCK_SIZE;
        self.domains
            .iter_mut()
            .filter(|(k, _)| {
                (k.line == position.line
                    || k.column == position.column
                    || ((k.line >= left_bracket && k.line < right_bracket)
                        && (k.column >= upper_bracket && k.column < lower_bracket)))
                    && (k.line != position.line || k.column != position.column)
            })
            .collect::<Vec<(&Position, &mut HashSet<SudokuDomainValue>)>>()
    }
}
impl Searchable<Position, SudokuDomainValue> for Sudoku {
    fn check_constraints(&self) -> bool {
        // TODO: imporove this code somehow
        for line in 0..BOARD_SIZE {
            let mut present: HashSet<SudokuDomainValue> = HashSet::new();
            for column in 0..BOARD_SIZE {
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

        for column in 0..BOARD_SIZE {
            let mut present: HashSet<SudokuDomainValue> = HashSet::new();
            for line in 0..BOARD_SIZE {
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

        for line_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE as usize) {
            for column_offset in (0..BOARD_SIZE).step_by(BLOCK_SIZE as usize) {
                let mut present: HashSet<SudokuDomainValue> = HashSet::new();
                for line in line_offset..(line_offset + BLOCK_SIZE) {
                    for column in column_offset..(column_offset + BLOCK_SIZE) {
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

    fn simplify(&mut self, position: &Position, value: SudokuDomainValue) -> bool {
        // TODO: [IMPORTANT] call simplify again when a cell becomes determined: if cell.remove() and cell.len() == 1
        // TODO: return result instead
        for (_, domain) in self.adjacent_mut(position) {
            domain.remove(&value);
            if domain.is_empty() {
                return false;
            }
        }
        true
    }
}

type Nqueens = State<Position, bool>;
impl Searchable<Position, bool> for Nqueens {
    fn check_constraints(&self) -> bool {
        println!("nqueens");
        true
    }

    fn simplify(&mut self, _variable: &Position, _value: bool) -> bool {
        false
    }
}

//======

fn main() {
    // TODO move this code out and leave only initialization and a function call

    let nqueens: Nqueens =
        Nqueens::new(&[Position { line: 0, column: 0 }], &HashSet::<bool>::new());
    nqueens.check_constraints();
    nqueens.offspring();

    println!(
        "sudoku {:?}/{:?} ({:?})",
        BLOCK_SIZE, BOARD_SIZE, CELL_DOMAIN
    );

    let variables: Vec<Position> = (0..BOARD_SIZE)
        .flat_map(|j| (0..BOARD_SIZE).map(move |k| Position { line: j, column: k }))
        .collect();
    let domain: HashSet<SudokuDomainValue> = CELL_DOMAIN.collect::<HashSet<SudokuDomainValue>>();

    let root = Sudoku::new(&variables, &domain);
    let mut stack: Vec<State<Position, SudokuDomainValue>> = vec![root];
    while let Some(parent) = stack.pop() {
        for child in parent.offspring() {
            if child.check_constraints() {
                println!("{:?}", child);
            } else {
                stack.push(child);
            }
        }
    }
}
