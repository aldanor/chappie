use std::iter::Iterator;
use std::collections::HashSet;
use std::hash::{Hash, Hasher, SipHasher};

pub trait SearchSpace {
    type State: Hash;
    type Action;
    type Iterator: Iterator<Item=Self::Action>;

    fn actions(&self, state: &Self::State) -> Self::Iterator;
    fn next_state(&self, state: &Self::State, action: &Self::Action) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}

pub fn depth_first_search<S>(space: &S, start: S::State) -> Option<Vec<S::Action>>
where S: SearchSpace {
    let mut path = Vec::new();
    let actions = space.actions(&start);
    let mut frontier = vec![(start, actions)];
    let mut visited = HashSet::new();

    loop {
        let result = match frontier.last_mut() {
            None => return None,
            Some(&mut (ref mut state, ref mut actions)) => {
                if space.is_goal(&state) {
                    return Some(path);
                }
                match actions.next() {
                    None => None,
                    Some(action) => {
                        let next_state = space.next_state(&state, &action);

                        let mut hasher = SipHasher::new();
                        next_state.hash(&mut hasher);
                        let hash = hasher.finish();
                        if visited.contains(&hash) {
                            continue;
                        }
                        visited.insert(hash);

                        let next_actions = space.actions(&next_state);
                        path.push(action);
                        Some((next_state, next_actions))
                    }
                }
            }
        };
        match result {
            None => {
                path.pop();
                frontier.pop();
            },
            Some((next_state, next_actions)) => {
                frontier.push((next_state, next_actions));
            },
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

        #[derive(Debug, Clone, PartialEq)]
        enum Direction {
            Left,
            Right
        }

        impl SearchSpace for S {
            type State = i32;
            type Action = Direction;
            type Iterator = IntoIter<Self::Action>;

            fn actions(&self, state: &i32) -> Self::Iterator {
                if *state == 0 || *state == 1 {
                    vec![Direction::Left, Direction::Right].into_iter()
                } else if *state == 2 {
                    vec![Direction::Left].into_iter()
                }
                else {
                    vec![].into_iter()
                }
            }

            fn next_state(&self, state: &i32, action: &Direction) -> i32 {
                match *state {
                    0 => match *action {
                        Direction::Left => 1,
                        Direction::Right => 2,
                    },
                    1 => match *action {
                        Direction::Left => 3,
                        Direction::Right => 4,
                    },
                    2 => 2,
                    node => node
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
