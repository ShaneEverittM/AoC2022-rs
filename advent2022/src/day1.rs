use std::cmp::Reverse;

use anyhow::{Context, Result};
use itertools::{process_results, Itertools};

fn for_each_chunk<R>(f: fn(&mut dyn Iterator<Item = u64>) -> R) -> Result<R> {
    process_results(
        include_str!("inputs/day1.txt").lines().map(|v| {
            if v.is_empty() {
                Ok(None)
            } else {
                v.parse::<u64>()
                    .context("Failed to parse line as calorie count")
                    .map(Some)
            }
        }),
        |iter| f(&mut iter.batching(|it| it.map_while(|it| it).sum1::<u64>())),
    )
}

pub fn part1() -> Result<u64> {
    for_each_chunk(|it| it.max().expect("Puzzle input is not empty"))
}

pub fn part2() -> Result<u64> {
    for_each_chunk(|it| it.map(Reverse).k_smallest(3).map(|x| x.0).sum())
}
