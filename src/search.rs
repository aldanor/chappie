use std::iter::Iterator;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher, SipHasher};

struct HashVisitor<T> {
    hash_set: HashSet<u64>,
    phantom: PhantomData<T>,
}

enum Visit {
    FirstTime,
    Revisit
}

impl<T> HashVisitor<T> where T: Hash {
    fn new() -> HashVisitor<T> {
        HashVisitor {
            hash_set: HashSet::new(),
            phantom: PhantomData
        }
    }

    fn visit(&mut self, item: &T) -> Visit {
        let mut hasher = SipHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        match self.hash_set.insert(hash) {
            true => Visit::FirstTime,
            false => Visit::Revisit
        }
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
    type State: Hash;
    type Action;
    type Iterator: Iterator<Item=(Self::Action, Self::State)>;

    fn expand(&self, state: &Self::State) -> Self::Iterator;

    fn dfs<G>(&self, start: Self::State, goal: G) -> Option<Vec<Self::Action>>
    where G: SearchGoal<Self::State> {
        if goal.is_goal(&start) {
            return Some(vec![]);
        }

        let mut path = Vec::new();
        let mut visited = HashVisitor::new();
        let mut frontier = vec![self.expand(&start)];

        loop {
            let result = match frontier.last_mut() {
                None => {
                    return None
                },
                Some(&mut ref mut iter) => {
                    match iter.next() {
                        None => {
                            path.pop();
                            None
                        },
                        Some((action, state)) => {
                            if let Visit::Revisit = visited.visit(&state) {
                                continue;
                            }
                            path.push(action);
                            if goal.is_goal(&state) {
                                return Some(path);
                            }
                            Some(self.expand(&state))
                        }
                    }
                }
            };
            match result {
                None => {
                    frontier.pop();
                }
                Some(iter) => {
                    frontier.push(iter);
                }
            };
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
