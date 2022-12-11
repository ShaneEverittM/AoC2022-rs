use itertools::Itertools;

fn find_n_unique(n: usize) -> Option<usize> {
    include_str!("inputs/day6.txt")
        .as_bytes()
        .windows(n)
        .position(|window| window.iter().unique().count() == n)
        .map(|i| i + n)
}

pub fn part1() -> Option<usize> {
    find_n_unique(4)
}
pub fn part2() -> Option<usize> {
    find_n_unique(14)
}