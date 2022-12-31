use std::{cmp::Ordering, fmt, iter::once};

use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Entry {
    Number(u64),
    List(Vec<Entry>),
}

impl Entry {
    fn with_slice<T>(&self, f: impl FnOnce(&[Entry]) -> T) -> T {
        match self {
            Entry::List(n) => f(&n[..]),
            Entry::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Number(n) => write!(f, "{n}"),
            Entry::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Entry::Number(a), Entry::Number(b)) => a.cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.cmp(r))),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input() -> Result<impl Iterator<Item = (Entry, Entry)>> {
    let iter = include_str!("inputs/day13.txt")
        .split("\r\n\r\n")
        .map(|pairs| {
            pairs
                .lines()
                .map(|line| serde_json::from_str::<Entry>(line).unwrap())
                .collect_tuple()
                .unwrap()
        });

    Ok(iter)
}

pub fn part1() -> Result<usize> {
    let pairs = parse_input()?;
    let mut sum = 0;
    for (i, (l, r)) in pairs.enumerate() {
        println!("\n== Pair {i} ==");
        println!("l = {l:?}");
        println!("r = {r:?}");
        println!("l < r = {}", l < r);
        if l < r {
            sum += i + 1;
        }
    }

    Ok(sum)
}

pub fn part2() -> Result<usize> {
    let divider_packets = vec![
        Entry::List(vec![Entry::Number(2)]),
        Entry::List(vec![Entry::Number(6)]),
    ];

    let mut packets = parse_input()?
        .flat_map(|tup| once(tup.0).chain(once(tup.1)))
        .chain(divider_packets.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    let decoder = divider_packets
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    Ok(decoder)
}
