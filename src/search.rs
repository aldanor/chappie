use std::iter::Iterator;
use std::collections::HashSet;
use std::hash::Hash;

struct Visited<T> {
    hash_set: HashSet<T>
}

impl<T> Visited<T> where T: Hash + Clone + Eq {
    fn new() -> Visited<T> {
        Visited {
            hash_set: HashSet::new()
        }
    }

    fn insert(&mut self, value: &T) -> bool {
        self.hash_set.insert(value.clone())
    }
}

pub trait SearchGoal<T> {
    fn is_goal(&self, state: &T) -> bool;
}

impl<T> SearchGoal<T> for T where T: PartialEq {
    fn is_goal(&self, state: &T) -> bool {
        *self == *state
    }
}

pub trait SearchSpace {
    type State: Hash + Clone + Eq;
    type Action;
    type Iterator: Iterator<Item=(Self::Action, Self::State)>;

    fn expand(&self, state: &Self::State) -> Self::Iterator;

    fn dfs<G>(&self, start: Self::State, goal: G) -> Option<Vec<Self::Action>>
    where G: SearchGoal<Self::State> {
        if goal.is_goal(&start) {
            return Some(vec![]);
        }

        let mut visited = Visited::new();
        let mut stack = vec![(self.expand(&start), None)];

        loop {
            let next = match stack.last_mut() {
                None => return None,
                Some(&mut (ref mut iter, _)) => iter.next()
            };
            if let Some((action, state)) = next {
                if !visited.insert(&state) {
                    continue;
                }
                if goal.is_goal(&state) {
                    return Some(
                        stack.into_iter()
                             .filter_map(|(_, a)| a)
                             .chain(Some(action).into_iter())
                             .collect()
                    )
                }
                stack.push((self.expand(&state), Some(action)));
            } else {
                stack.pop();
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::vec::IntoIter;
    use super::SearchSpace;

    #[test]
    pub fn test_dfs() {
        struct TestSearch;

        #[derive(Debug, PartialEq)]
        enum Dir { Left, Right }

        impl SearchSpace for TestSearch {
            type State = i32;
            type Action = Dir;
            type Iterator = IntoIter<(Self::Action, Self::State)>;

            fn expand(&self, state: &Self::State) -> Self::Iterator {
                if *state == 0 {
                    vec![(Dir::Left, 1), (Dir::Right, 2)].into_iter()
                } else if *state == 1 {
                    vec![(Dir::Left, 3), (Dir::Right, 4)].into_iter()
                } else if *state == 2 {
                    vec![(Dir::Left, 2)].into_iter()
                } else {
                    vec![].into_iter()
                }
            }
        }

        let ts = TestSearch;

        assert_eq!(ts.dfs(0, 0).unwrap(), vec![]);
        assert_eq!(ts.dfs(0, 1).unwrap(), vec![Dir::Left]);
        assert_eq!(ts.dfs(0, 2).unwrap(), vec![Dir::Right]);
        assert_eq!(ts.dfs(0, 3).unwrap(), vec![Dir::Left, Dir::Left]);
        assert_eq!(ts.dfs(0, 4).unwrap(), vec![Dir::Left, Dir::Right]);
        assert_eq!(ts.dfs(2, 2).unwrap(), vec![]);
        assert!(ts.dfs(2, 0).is_none());
        assert!(ts.dfs(5, 0).is_none());
    }
}
