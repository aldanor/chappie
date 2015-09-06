use std::iter::Iterator;
use std::collections::HashSet;
use std::hash::{Hash, Hasher, SipHasher};

pub trait SearchSpace {
    type State: Hash;
    type Action;
    type Iterator: Iterator<Item=(Self::Action, Self::State)>;

    fn expand(&self, state: &Self::State) -> Self::Iterator;
    fn is_goal(&self, state: &Self::State) -> bool;
}

pub fn depth_first_search<S>(space: &S, start: S::State) -> Option<Vec<S::Action>>
where S: SearchSpace {
    if space.is_goal(&start) {
        return Some(vec![]);
    }

    let mut path = Vec::new();
    let mut visited = HashSet::new();
    let mut frontier = vec![space.expand(&start)];

    loop {
        let result = match frontier.last_mut() {
            None => return None,
            Some(&mut ref mut iter) => {
                match iter.next() {
                    None => {
                        path.pop();
                        None
                    },
                    Some((action, state)) => {
                        let mut hasher = SipHasher::new();
                        state.hash(&mut hasher);
                        let hash = hasher.finish();
                        if visited.contains(&hash) {
                            continue;
                        }
                        visited.insert(hash);

                        path.push(action);
                        if space.is_goal(&state) {
                            return Some(path);
                        }
                        Some(space.expand(&state))
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

#[cfg(test)]
pub mod tests {
    use std::vec::IntoIter;
    use super::*;

    #[test]
    pub fn test_depth_first_search() {
        struct S {
            goal: i32,
        }

        #[derive(Debug, PartialEq)]
        enum Direction {
            Left,
            Right
        }

        impl SearchSpace for S {
            type State = i32;
            type Action = Direction;
            type Iterator = IntoIter<(Self::Action, Self::State)>;

            fn expand(&self, state: &Self::State) -> Self::Iterator {
                if *state == 0 {
                    vec![(Direction::Left, 1), (Direction::Right, 2)].into_iter()
                } else if *state == 1 {
                    vec![(Direction::Left, 3), (Direction::Right, 4)].into_iter()
                } else if *state == 2 {
                    vec![(Direction::Left, 2)].into_iter()
                } else {
                    vec![].into_iter()
                }
            }

            fn is_goal(&self, state: &i32) -> bool {
                return self.goal == *state
            }
        }

        impl S {
            fn solve(start: i32, goal: i32) -> Option<Vec<Direction>> {
                let space = S { goal: goal };
                depth_first_search(&space, start)
            }
        }

        assert_eq!(S::solve(0, 0).unwrap(), vec![]);
        assert_eq!(S::solve(0, 1).unwrap(), vec![Direction::Left]);
        assert_eq!(S::solve(0, 2).unwrap(), vec![Direction::Right]);
        assert_eq!(S::solve(0, 3).unwrap(), vec![Direction::Left, Direction::Left]);
        assert_eq!(S::solve(0, 4).unwrap(), vec![Direction::Left, Direction::Right]);
        assert_eq!(S::solve(2, 2).unwrap(), vec![]);
        assert!(S::solve(2, 0).is_none());
        assert!(S::solve(5, 0).is_none());
    }
}
