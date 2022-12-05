use std::collections::HashSet;

use itertools::Itertools;
use substring::Substring;

fn score(c: &char) -> u32 {
    if c.is_uppercase() {
        (*c as u32) - 38
    } else {
        (*c as u32) - 96
    }
}

pub fn part1() -> u32 {
    include_str!("inputs/day3.txt")
        .lines()
        .map(|line| {
            let len = line.len();
            let mid = len / 2;
            let compartment1 = line.substring(0, mid).chars().collect::<HashSet<_>>();
            let compartment2 = line.substring(mid, len).chars().collect::<HashSet<_>>();

            let mut intersection = compartment1.intersection(&compartment2);
            score(intersection.next().expect("Compartments have overlap"))
        })
        .sum()
}

pub fn part2() -> u32 {
    include_str!("inputs/day3.txt")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let (bag1, bag2, bag3) = lines
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect_tuple()
                .expect("Expected 3 bags");

            let bag_1_and_2 = bag1.intersection(&bag2).copied().collect::<HashSet<_>>();
            let mut intersection = bag_1_and_2.intersection(&bag3);

            score(intersection.next().expect("Compartments have overlap"))
        })
        .sum()
}
