// XXX add the value to the cell when we are sure, also change the filter to get all the cells
// where we are not sure, even if there's only one possibility

// TODO what if `pub` is removed from function sugrnatures?

use itertools::Itertools;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
enum InvalidCell {
    EmptyValueSet,
    ImpossibleValue,
}

#[derive(Clone)]
struct Cell {
    line: usize,
    column: usize,
    possible: HashSet<u8>,
}

impl Cell {
    pub fn new(line: usize, column: usize) -> Self {
        Cell {
            possible: (1..10).collect(),
            line: line,
            column: column,
        }
    }

    pub fn set(&mut self, value: u8) -> Result<(), InvalidCell> {
        if self.possible.contains(&value) {
            self.possible.retain(|v| v == &value);
            Ok(())
        } else {
            Err(InvalidCell::ImpossibleValue)
        }
    }

    pub fn remove(&mut self, value: u8) -> Result<(), InvalidCell> {
        self.possible.remove(&value);
        if self.possible.is_empty() {
            Err(InvalidCell::EmptyValueSet)
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}@({},{})",
            self.possible.iter().join(","),
            self.line,
            self.column
        )
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: {
                let mut cells: Vec<Vec<Cell>> = Vec::new(); //vec![];
                for line in 0..9 {
                    let mut row: Vec<Cell> = vec![];
                    for column in 0..9 {
                        row.push(Cell::new(line, column));
                    }
                    cells.push(row);
                }
                cells
            },
        }
    }

    pub fn set_cell_value(
        &mut self,
        line: usize,
        column: usize,
        value: u8,
    ) -> Result<(), InvalidCell> {
        // TODO: check if the board is valid after this function call
        // If any of the cells are empty of possibilities, then it's invalid

        self.cells[line][column].set(value);
        for c in 0..9 {
            if c != column {
                match self.cells[line][c].remove(value) {
                    Err(x) => return Err(x),
                    _ => (),
                }
            }
        }
        for l in 0..9 {
            if l != line {
                match self.cells[l][column].remove(value) {
                    Err(x) => return Err(x),
                    _ => (),
                }
            }
        }
        for l in (line / 3) * 3..(line / 3) * 3 + 3 {
            for c in (column / 3) * 3..(column / 3) * 3 + 3 {
                if l != line || c != column {
                    match self.cells[l][c].remove(value) {
                        Err(x) => return Err(x),
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn children(&self) -> Vec<Board> {
        let free_cells: Vec<Cell> = self
            .cells
            .clone()
            .into_iter()
            .flatten()
            .filter(|c| c.possible.len() > 1)
            .collect();

        // TODO: into_iter vs iter
        let free_cell_min_possible: usize = free_cells
            .clone()
            .into_iter()
            .map(|c| c.possible.len())
            .min()
            .unwrap(); // TODO what if?

        let target_cells: Vec<Cell> = free_cells
            .into_iter()
            .filter(|c| c.possible.len() == free_cell_min_possible)
            .collect();

        // TODO is this random right?
        let target_cell = target_cells.choose(&mut rand::thread_rng()).unwrap();
        println!("target_cell: {:?}", target_cell);

        let mut children: Vec<Board> = vec![];
        // TODO randomize
        for value in target_cell.possible.clone() {
            let mut child = self.clone();
            match child.set_cell_value(target_cell.line, target_cell.column, value) {
                Err(_) => {
                    continue;
                }
                _ => (),
            }
            println!("{:?}", value);
            println!("child: {:?}", child);
            children.push(child);
        }

        match children.len() {
            1 => children.pop().unwrap().children(),
            _ => children,
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.cells {
            writeln!(f);
            for cell in line {
                write!(f, "|{:?}", cell);
            }
        }
        Ok(())
    }
}

fn main() {
    let mut stack: Vec<Board> = vec![Board::new()];
    while let Some(board) = stack.pop() {
        println!("===\nprocessing board: {:?}", board);
        let children = board.children();
        match children.len() {
            0 => {
                println!("no solution: {:?}", board);
            }
            1 => {
                println!("unique solution: {:?}", board);
                // println!("\twith children: {:?}", children[0]);
            }
            _ => {
                println!("multiple solutions: {:?}", board);
                stack.extend(children);
            }
        }
    }
}
