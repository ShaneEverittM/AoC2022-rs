use std::collections::HashSet;

use itertools::Itertools;

fn find_n_unique(n: usize) -> Option<usize> {
    include_str!("inputs/day6.txt")
        .as_bytes()
        .windows(n)
        .map(|n| HashSet::<u8>::from_iter(n.iter().copied()))
        .find_position(|set| set.len() == n)
        .map(|(i, _)| i + n)
}

pub fn part1() -> Option<usize> {
    find_n_unique(4)
}
pub fn part2() -> Option<usize> {
    find_n_unique(14)
}