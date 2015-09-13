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
    use rand::chacha::ChaChaRng;
    use rand::Rng;
    use super::SearchSpace;
    use std::iter::Enumerate;

    struct RandomGraph {
        nodes: Vec<Vec<usize>>
    }

    impl RandomGraph {
        fn new(n_nodes: usize, max_edges: usize) -> RandomGraph {
            let mut rng_e = ChaChaRng::new_unseeded();
            let mut rng_n = ChaChaRng::new_unseeded();

            RandomGraph {
                nodes: rng_e.gen_iter::<usize>().take(n_nodes)
                    .map(|e| rng_n.gen_iter::<usize>().take(e % max_edges)
                        .map(|n| n % n_nodes).collect()).collect()
            }
        }

        #[allow(dead_code)]
        fn to_graphviz(&self) -> String {
            let mut buf = "digraph G {".to_owned();
            for (node_from, nodes) in self.nodes.iter().enumerate() {
                for (action, node_to) in nodes.iter().enumerate() {
                    buf.push_str(format!("\n\t{} -> {} [ label=\"{}\" ];",
                                         node_from, node_to, action).as_ref());
                }
            }
            buf.push_str("\n}");
            buf
        }
    }

    impl SearchSpace for RandomGraph {
        type State = usize;
        type Action = usize;
        type Iterator = Enumerate<IntoIter<usize>>;

        fn expand(&self, state: &usize) -> Self::Iterator {
            if *state < self.nodes.len() {
                self.nodes[*state].clone()
            } else {
                vec![]
            }.into_iter().enumerate()
        }
    }

    #[test]
    pub fn test_dfs_simple() {
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
        const N_NODES: usize = 24;
        const MAX_EDGES: usize = 6;

        let g = RandomGraph::new(N_NODES, MAX_EDGES);

        assert!(g.dfs(N_NODES, 0).is_none());
        assert!(g.dfs(0, N_NODES).is_none());

        for start in (0..N_NODES) {
            for goal in (0..N_NODES) {
                if let Some(path) = g.dfs(start, goal) {
                    let mut state = start;
                    for action in path {
                        state = g.expand(&state).skip(action).next().unwrap().1;
                    }
                    assert_eq!(state, goal);
                }
            }
        }
    }
}
