# snyder
A suboptimal backtracking Constraint Satisfaction Problem solver

## Quick Start

### Latin Square

> In combinatorics and in experimental design, a Latin square is an n × n array filled with n different symbols, each occurring exactly once in each row and exactly once in each column.

https://en.wikipedia.org/wiki/Latin_square


```rust
extern crate snyder;                                                                                                                                                                          
```

Define the variable type.
```rust
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}
```

Define the domain type.
```rust
type Symbol = u32;
```

Define the problem using variable and domain type.
```rust
type LatinSquare = snyder::State<Position, Symbol>;
```

Implement `snyder::Searchable` to define your constraints.
```rust
impl snyder::Searchable<Position, Symbol> for LatinSquare {
    fn check_constraints(&self, position: &Position, symbol: Symbol) -> bool {
        // the currently placed symbol is different from all other numbers on the same line or
        // column
        !self
            .determined()
            .any(|(k, v)| k != position && (k.x == position.x || k.y == position.y) && *v == symbol)
    }
}
```

Optionally, help the system by simplifying the states.
```rust
impl snyder::Searchable<Position, Symbol> for LatinSquare {
    (...)

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
```

Define the set of variables and domains and iterate the solutions.
```rust
    use std::collections::HashSet;
    
    (...)
    
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
    let domain_set = HashSet::from([1, 2, 3]);
    let latin_square = LatinSquare::new(&variable_set, &domain_set);
    for state in latin_square.solution_iter() {
        println!("{state:?}");
    }
```

Enjoy the results.
```
State { domains: {Position { x: 2, y: 1 }: Determined(2), Position { x: 2, y: 2 }: Determined(3), Position { x: 1, y: 0 }: Determined(2), Position { x: 0, y: 1 }: Determined(1), Position { x: 1, y: 2 }: Determined(1), Position { x: 1, y: 1 }: Determined(3), Position { x: 2, y: 0 }: Determined(1), Position { x: 0, y: 2 }: Determined(2), Position { x: 0, y: 0 }: Determined(3)} }
State { domains: {Position { x: 2, y: 1 }: Determined(2), Position { x: 2, y: 2 }: Determined(3), Position { x: 1, y: 0 }: Determined(3), Position { x: 0, y: 1 }: Determined(3), Position { x: 1, y: 2 }: Determined(2), Position { x: 1, y: 1 }: Determined(1), Position { x: 2, y: 0 }: Determined(1), Position { x: 0, y: 2 }: Determined(1), Position { x: 0, y: 0 }: Determined(2)} }
State { domains: {Position { x: 2, y: 1 }: Determined(2), Position { x: 2, y: 2 }: Determined(1), Position { x: 1, y: 0 }: Determined(2), Position { x: 0, y: 1 }: Determined(3), Position { x: 1, y: 2 }: Determined(3), Position { x: 1, y: 1 }: Determined(1), Position { x: 2, y: 0 }: Determined(3), Position { x: 0, y: 2 }: Determined(2), Position { x: 0, y: 0 }: Determined(1)} }
State { domains: {Position { x: 2, y: 1 }: Determined(2), Position { x: 2, y: 2 }: Determined(1), Position { x: 1, y: 0 }: Determined(1), Position { x: 0, y: 1 }: Determined(1), Position { x: 1, y: 2 }: Determined(2), Position { x: 1, y: 1 }: Determined(3), Position { x: 2, y: 0 }: Determined(3), Position { x: 0, y: 2 }: Determined(3), Position { x: 0, y: 0 }: Determined(2)} }
State { domains: {Position { x: 2, y: 1 }: Determined(3), Position { x: 2, y: 2 }: Determined(2), Position { x: 1, y: 0 }: Determined(2), Position { x: 0, y: 1 }: Determined(2), Position { x: 1, y: 2 }: Determined(3), Position { x: 1, y: 1 }: Determined(1), Position { x: 2, y: 0 }: Determined(1), Position { x: 0, y: 2 }: Determined(1), Position { x: 0, y: 0 }: Determined(3)} }
State { domains: {Position { x: 2, y: 1 }: Determined(3), Position { x: 2, y: 2 }: Determined(2), Position { x: 1, y: 0 }: Determined(3), Position { x: 0, y: 1 }: Determined(1), Position { x: 1, y: 2 }: Determined(1), Position { x: 1, y: 1 }: Determined(2), Position { x: 2, y: 0 }: Determined(1), Position { x: 0, y: 2 }: Determined(3), Position { x: 0, y: 0 }: Determined(2)} }
State { domains: {Position { x: 2, y: 1 }: Determined(3), Position { x: 2, y: 2 }: Determined(1), Position { x: 1, y: 0 }: Determined(3), Position { x: 0, y: 1 }: Determined(2), Position { x: 1, y: 2 }: Determined(2), Position { x: 1, y: 1 }: Determined(1), Position { x: 2, y: 0 }: Determined(2), Position { x: 0, y: 2 }: Determined(3), Position { x: 0, y: 0 }: Determined(1)} }
State { domains: {Position { x: 2, y: 1 }: Determined(3), Position { x: 2, y: 2 }: Determined(1), Position { x: 1, y: 0 }: Determined(1), Position { x: 0, y: 1 }: Determined(1), Position { x: 1, y: 2 }: Determined(3), Position { x: 1, y: 1 }: Determined(2), Position { x: 2, y: 0 }: Determined(2), Position { x: 0, y: 2 }: Determined(2), Position { x: 0, y: 0 }: Determined(3)} }
State { domains: {Position { x: 2, y: 1 }: Determined(1), Position { x: 2, y: 2 }: Determined(2), Position { x: 1, y: 0 }: Determined(2), Position { x: 0, y: 1 }: Determined(2), Position { x: 1, y: 2 }: Determined(1), Position { x: 1, y: 1 }: Determined(3), Position { x: 2, y: 0 }: Determined(3), Position { x: 0, y: 2 }: Determined(3), Position { x: 0, y: 0 }: Determined(1)} }
State { domains: {Position { x: 2, y: 1 }: Determined(1), Position { x: 2, y: 2 }: Determined(2), Position { x: 1, y: 0 }: Determined(1), Position { x: 0, y: 1 }: Determined(3), Position { x: 1, y: 2 }: Determined(3), Position { x: 1, y: 1 }: Determined(2), Position { x: 2, y: 0 }: Determined(3), Position { x: 0, y: 2 }: Determined(1), Position { x: 0, y: 0 }: Determined(2)} }
State { domains: {Position { x: 2, y: 1 }: Determined(1), Position { x: 2, y: 2 }: Determined(3), Position { x: 1, y: 0 }: Determined(3), Position { x: 0, y: 1 }: Determined(3), Position { x: 1, y: 2 }: Determined(1), Position { x: 1, y: 1 }: Determined(2), Position { x: 2, y: 0 }: Determined(2), Position { x: 0, y: 2 }: Determined(2), Position { x: 0, y: 0 }: Determined(1)} }
State { domains: {Position { x: 2, y: 1 }: Determined(1), Position { x: 2, y: 2 }: Determined(3), Position { x: 1, y: 0 }: Determined(1), Position { x: 0, y: 1 }: Determined(2), Position { x: 1, y: 2 }: Determined(2), Position { x: 1, y: 1 }: Determined(3), Position { x: 2, y: 0 }: Determined(2), Position { x: 0, y: 2 }: Determined(1), Position { x: 0, y: 0 }: Determined(3)} }
```

![things-2634520628](https://user-images.githubusercontent.com/7264271/197081099-b8f2fad9-5440-4e57-ab56-fe0c5caee47b.jpeg)
