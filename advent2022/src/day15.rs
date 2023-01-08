use std::{
    cmp::Ordering,
    collections::BTreeMap,
    ops::{Range, RangeInclusive},
};

use anyhow::{Context, Result};
use derive_more::Deref;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    *,
};

use crate::utils::InclusiveRangeExt;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Deref, Clone, PartialEq, Eq)]
struct Coverage(RangeInclusive<i64>);

impl IntoIterator for Coverage {
    type Item = <RangeInclusive<i64> as IntoIterator>::Item;
    type IntoIter = <RangeInclusive<i64> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}

impl PartialOrd<Self> for Coverage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coverage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start().cmp(other.start())
    }
}

struct Zone {
    sensor: Point,
    range: i64,
}

impl Zone {
    fn coverage_at_y(&self, y: i64) -> Coverage {
        let Zone { sensor, range } = self;

        let walk_on_y = range - (sensor.y - y).abs();

        Coverage((sensor.x - walk_on_y)..=sensor.x + walk_on_y)
    }

    fn vertical_coverage(&self) -> Range<i64> {
        let Zone { sensor, range } = self;
        (sensor.y - range)..(sensor.y + range)
    }
}

fn position(input: &str) -> IResult<&str, Point> {
    let (i, (x, y)) = separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)?;

    Ok((i, Point { x, y }))
}

fn zones(input: &str) -> IResult<&str, Vec<Zone>> {
    let (input, list) = separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(position, tag(": closest beacon is at "), position),
        ),
    )(input)?;

    Ok((
        input,
        list.into_iter()
            .map(|(sensor, beacon)| Zone {
                sensor,
                range: sensor.manhattan_distance(&beacon),
            })
            .collect(),
    ))
}

fn parse_input() -> Result<Vec<Zone>> {
    let (_, zones) = all_consuming(zones)(include_str!("inputs/day15.txt")).finish()?;

    Ok(zones)
}

pub fn part1() -> Result<usize> {
    const ROW: i64 = 2000000;
    let zones = parse_input()?;

    let coverage = zones
        .iter()
        // Filter to just the sensors whose exclusion zone contains the row in question
        .filter(|zone| zone.vertical_coverage().contains(&ROW))
        // Compute how much of the sensor's range covers the row in question
        .flat_map(|zone| zone.coverage_at_y(ROW))
        // Deduplicate since sensors coverage can overlap
        .unique()
        .count()
        - 1;

    Ok(coverage)
}

pub fn part2() -> Result<i64> {
    const LIMIT: i64 = 4_000_000;
    let zones = parse_input()?;

    // Get mapping of y to the coverage along y-axis at x of sensor
    let coverages = zones
        .iter()
        .flat_map(|zone| zone.vertical_coverage().map(|y| (y, zone.coverage_at_y(y))));

    // Compute mapping of y indices to list of coverages at that row
    let mut ordered_coverages: BTreeMap<i64, Vec<Coverage>> = BTreeMap::new();
    for (y, coverage_at_y) in coverages {
        if (0..=LIMIT).contains(&y) {
            ordered_coverages
                .entry(y)
                .and_modify(|coverages_at_y| coverages_at_y.push(coverage_at_y.clone()))
                .or_insert_with(|| vec![coverage_at_y]);
        }
    }

    // For every row, search for a gap in the coverage
    let (x, y) = ordered_coverages
        .into_iter()
        .find_map(|(y, coverages)| {
            // Find gap by trying to build up a contiguous range from the coverages
            let mut range = 0..=0;
            for coverage in coverages.iter().sorted() {
                // No gap, range is still contiguous
                if range.precedes(coverage) || range.overlaps(coverage) {
                    range = range.extend_by(coverage);
                }
                // There is a gap in the coverage, that's the beacon!
                else {
                    return Some((range.end() + 1, y));
                }
            }

            None
        })
        .context("Could not find distress beacon!")?;

    // Convert location to tuning frequency
    Ok(x * 4000000 + y)
}
