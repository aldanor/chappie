#![feature(test)]

extern crate chappie;
extern crate test;

use chappie::search::SearchSpace;
use test::{Bencher, black_box};
use std::vec::IntoIter;

enum Dir { Left, Right}

struct BinaryTree;

const MAX_DEPTH: u64 = 16;
const MAX_OFFSET: u64 = 1 << (MAX_DEPTH + 1);

impl SearchSpace for BinaryTree {
    type State = u64;
    type Action = Dir;
    type Iterator = IntoIter<(Self::Action, Self::State)>;

    fn expand(&self, state: &Self::State) -> Self::Iterator {
        let offset = (*state + 2).next_power_of_two();
        if offset >= MAX_OFFSET {
            return vec![].into_iter();
        }
        let right = *state + offset;
        let left = *state + offset / 2;
        vec![(Dir::Left, left), (Dir::Right, right)].into_iter()
    }
}

#[bench]
fn dfs(b: &mut Bencher) {
    let tree = BinaryTree;
    b.iter(|| { black_box(tree.dfs(&0, &2)) });
}
