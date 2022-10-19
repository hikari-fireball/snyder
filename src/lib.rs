use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct State<V, D> {
    pub domains: HashMap<V, HashSet<D>>, // TODO make private
}

impl<V, D> State<V, D>
where
    State<V, D>: Searchable<V, D> + Clone,
    V: Eq + Hash + Copy,
    D: Eq + Hash + Copy,
{
    pub fn new(variables: &[V], domain: &HashSet<D>) -> State<V, D> {
        State {
            domains: variables.iter().map(|v| (*v, domain.clone())).collect(),
        }
    }

    pub fn solution_iter(&self) -> SolutionIterator<V, D> {
        SolutionIterator::new(self.clone())
    }

    fn most_constrained_variable(&self) -> Option<&V> {
        self.domains
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .min_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len()))
            .map(|(k, _)| k)
    }

    fn is_solved(&self) -> bool {
        !self.domains.iter().any(|(_, v)| v.len() > 1)
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
                    if self.check_constraints(position, *value)
                        && child.simplify(position, *value).is_ok()
                    {
                        children.push(child);
                    }
                }
                children
            }
            None => Vec::new(),
        }
    }
}

pub struct InvalidStateError;

pub trait Searchable<V, D> {
    fn check_constraints(&self, variable: &V, value: D) -> bool;
    fn simplify(&mut self, _position: &V, _value: D) -> Result<(), InvalidStateError> {
        Ok(())
    }
}

pub struct SolutionIterator<V, D> {
    stack: Vec<State<V, D>>,
}

impl<V, D> SolutionIterator<V, D> {
    fn new(state: State<V, D>) -> SolutionIterator<V, D> {
        SolutionIterator { stack: vec![state] }
    }
}

impl<V, D> Iterator for SolutionIterator<V, D>
where
    State<V, D>: Searchable<V, D> + Clone,
    V: Eq + Hash + Copy,
    D: Eq + Hash + Copy,
{
    type Item = State<V, D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(state) => {
                if state.is_solved() {
                    Some(state)
                } else {
                    self.stack.extend(state.offspring());
                    self.next()
                }
            }
            None => None,
        }
    }
}
