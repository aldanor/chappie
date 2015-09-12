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
        self == state
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
    use std::fmt;
    use rand::chacha::ChaChaRng;
    use rand::Rng;
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
                match *state {
                    0 => vec![(Dir::Left, 1), (Dir::Right, 2)],
                    1 => vec![(Dir::Left, 3), (Dir::Right, 4)],
                    2 => vec![(Dir::Left, 2)],
                    _ => vec![]
                }.into_iter()
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

    #[test]
    pub fn test_dfs_random() {
        struct RandomTree {
            nodes: Vec<Vec<usize>>
        };

        impl fmt::Display for RandomTree {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                try!(writeln!(f, "{}", "digraph x {"));
                try!(writeln!(f, "{}",
                         self.nodes.iter()
                         .zip(0..)
                         .flat_map(|(ref edges, node_no)| edges.iter().zip(Some(node_no).into_iter().cycle()))
                         .collect::<Vec<_>>()
                         .join("\n")
                        ));
                writeln!(f, "{}", "}")
            }
        }

        impl RandomTree {
            fn new(nodes_no: usize, max_edges: usize) -> RandomTree {
                let mut rng = ChaChaRng::new_unseeded();
                let mut rng2 = ChaChaRng::new_unseeded();

                let mut nodes = (0..nodes_no).collect::<Vec<usize>>();
                rng.shuffle(&mut nodes[..]);

                assert!(max_edges < nodes_no);

                RandomTree {
                    nodes: nodes.into_iter().map(|_|
                        rng.gen_iter::<usize>()
                        .map(|rand| rand % nodes_no)
                        .take(rng2.gen_range(1, max_edges - 1))
                        .collect()
                    ).collect()
                }
            }
        }

        impl SearchSpace for RandomTree {
            type State = usize;
            type Action = usize;
            type Iterator = IntoIter<(Self::Action, Self::State)>;

            fn expand(&self, state: &Self::State) -> Self::Iterator {
                //println!("state: {}", state);
                self.nodes[*state].clone().into_iter()
                    .enumerate()
                    .map(|(action, node)|
                         (action, node)
                     )
                    //.inspect(|&(action, node)|
                        //println!("{} -> {}", action, node)
                    //)
                    .collect::<Vec<(usize, usize)>>().into_iter()
            }
        }

        let rt = RandomTree::new(24, 6);
        //println!("{}", rt);
        assert_eq!(rt.dfs(0, 0).unwrap(), vec![]);
        assert!(rt.dfs(0, 4).is_none());
        assert_eq!(rt.dfs(1, 4).unwrap(), vec![3]);
        assert_eq!(rt.dfs(1, 4).unwrap(), vec![3]);
        assert_eq!(rt.dfs(6, 13).unwrap(), vec![1, 0, 0, 0, 1, 0, 1, 0, 0]);
    }
}
