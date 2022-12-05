use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1},
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::utils::RangeExt;

fn decimal(input: &str) -> IResult<&str, u32> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.parse::<u32>(),
    )(input)
}

fn range(input: &str) -> IResult<&str, Range<u32>> {
    map(
        separated_pair(decimal, tag("-"), decimal),
        |(lower, upper): (u32, u32)| lower..upper,
    )(input)
}

fn parse_line(input: &str) -> (Range<u32>, Range<u32>) {
    let (_, (r1, r2)) = separated_pair(range, tag(","), range)(input).expect("Failed to parse");
    (r1, r2)
}

pub fn count_ranges(predicate: fn(r1: Range<u32>, r2: Range<u32>) -> bool) -> usize {
    include_str!("inputs/day4.txt")
        .lines()
        .filter(|line| {
            let (r1, r2) = parse_line(line);
            predicate(r1, r2)
        })
        .count()
}

pub fn part1() -> usize {
    count_ranges(|r1, r2| r1.contains_range(&r2) || r2.contains_range(&r1))
}

pub fn part2() -> usize {
    count_ranges(|r1, r2| r1.overlaps(&r2))
}
