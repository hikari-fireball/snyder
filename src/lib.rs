use core::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Domain<T> {
    Determined(T),
    Undetermined(HashSet<T>),
}

#[derive(Debug, Clone)]
pub struct State<V, D> {
    domains: HashMap<V, Domain<D>>,
}

impl<V, D> State<V, D>
where
    Self: Searchable<V, D> + Clone,
    V: Eq + Hash + Copy,
    D: Eq + Hash + Copy,
{
    pub fn new(variables: &[V], domain: &HashSet<D>) -> Self {
        return Self {
            domains: variables
                .iter()
                .map(|v| (*v, Domain::Undetermined(domain.clone())))
                .collect(),
        };
    }

    pub fn iter<'state>(&'state self) -> impl Iterator<Item = (&V, &Domain<D>)> + 'state {
        return self.domains.iter();
    }

    pub fn iter_mut<'state>(
        &'state mut self,
    ) -> impl Iterator<Item = (&V, &mut Domain<D>)> + 'state {
        return self.domains.iter_mut();
    }

    pub fn determined<'state>(&'state self) -> impl Iterator<Item = (&V, &D)> + 'state {
        self.domains.iter().filter_map(|(k, v)| match v {
            Domain::Determined(v) => return Some((k, v)),
            _ => return None,
        })
    }

    pub fn undetermined<'state>(&'state self) -> impl Iterator<Item = (&V, &HashSet<D>)> + 'state {
        self.domains.iter().filter_map(|(k, v)| match v {
            Domain::Undetermined(v) => return Some((k, v)),
            _ => return None,
        })
    }

    pub fn undetermined_mut<'state>(
        &'state mut self,
    ) -> impl Iterator<Item = (&V, &mut HashSet<D>)> + 'state {
        self.domains.iter_mut().filter_map(|(k, v)| match v {
            Domain::Undetermined(v) => return Some((k, v)),
            _ => return None,
        })
    }

    #[inline]
    pub fn solution_iter(&self) -> SolutionIterator<V, D> {
        return SolutionIterator::new(self.clone());
    }

    fn most_constrained_variable(&self) -> Option<(&V, &HashSet<D>)> {
        return self
            .undetermined()
            .min_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len()));
    }

    fn is_solved(&self) -> bool {
        return !self.undetermined().any(|_| true);
    }

    fn offspring(&self) -> Vec<Self> {
        match self.most_constrained_variable() {
            Some((variable, value_set)) => {
                let mut children: Vec<Self> = vec![];
                for value in value_set {
                    let mut child = self.clone();
                    child.domains.insert(*variable, Domain::Determined(*value));
                    if child.check_constraints(variable, *value) {
                        child.simplify(variable, *value);
                        children.push(child);
                    }
                }
                return children;
            }
            None => return Vec::new(),
        }
    }
}

pub struct InvalidStateError;

pub trait Searchable<V, D> {
    fn check_constraints(&self, variable: &V, value: D) -> bool;
    #[inline]
    fn simplify(&mut self, _variable: &V, _value: D) {}
}

pub struct SolutionIterator<V, D> {
    stack: Vec<State<V, D>>,
}

impl<V, D> SolutionIterator<V, D> {
    fn new(state: State<V, D>) -> Self {
        Self { stack: vec![state] }
    }
}

impl<V, D> Iterator for SolutionIterator<V, D>
where
    State<V, D>: Searchable<V, D> + Clone,
    V: Eq + Hash + Copy,
    D: Eq + Hash + Copy,
{
    type Item = State<V, D>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(state) => {
                if state.is_solved() {
                    return Some(state);
                } else {
                    self.stack.extend(state.offspring());
                    return self.next();
                }
            }
            None => return None,
        }
    }
}
