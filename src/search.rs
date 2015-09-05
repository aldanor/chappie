use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashSet;

pub trait SearchSpace {
    type State: Ord + Hash + Eq + Clone + Debug;
    type Action: Clone;

    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn next_state(&self, state: &Self::State, action: &Self::Action) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
}

pub fn depth_first_search<S>(space: &S, start: S::State) -> Option<Vec<S::Action>>
where S: SearchSpace {
    let mut path = Vec::new();
    let start_actions = space.actions(&start);
    let mut frontier = vec![(start, start_actions.into_iter())];

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
                        let next_actions = space.actions(&next_state).into_iter();
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

            fn actions(&self, state: &i32) -> Vec<Direction> {
                if *state == 0 || *state == 1 {
                    vec![Direction::Left, Direction::Right]
                } else {
                    vec![]
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
        assert!(S::solve(5, 0).is_none());
    }
}
