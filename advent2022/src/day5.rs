use std::{
    fmt::{self, Debug, Formatter},
    ops::Sub,
    str::FromStr,
};

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{all_consuming, map, map_opt, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

#[derive(Copy, Clone, Deref)]
struct Crate(char);

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

#[derive(Deref, DerefMut)]
struct Stacks(Vec<Vec<Crate>>);

impl Debug for Stacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, stack) in self.iter().enumerate() {
            writeln!(f, "Stack {}: {:?}", i, stack)?;
        }
        Ok(())
    }
}

impl Stacks {
    fn apply(&mut self, mov: Move) {
        (0..mov.amount).for_each(|_| {
            let thing = self[mov.from].pop().unwrap();
            self[mov.to].push(thing);
        })
    }

    fn apply_bulk(&mut self, mov: Move) {
        let from = &mut self[mov.from];
        let stack = from
            .drain((from.len() - mov.amount as usize)..)
            .collect::<Vec<_>>();
        self[mov.to].extend(stack)
    }
}

#[derive(Debug)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let one_crate = delimited(tag("["), take(1_usize), tag("]"));
    map_opt(one_crate, |s: &str| s.chars().next().map(Crate))(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_slot(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_layer(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let pad = |mut layer: Vec<Option<Crate>>| {
        layer.resize(9, None);
        layer
    };
    map(separated_list1(tag(" "), parse_slot), pad)(i)
}

fn parse_number<N: FromStr>(i: &str) -> IResult<&str, N> {
    map_res(digit1, str::parse)(i)
}

fn parse_stack_number<T: FromStr + Sub<usize, Output = T>>(i: &str) -> IResult<&str, T> {
    map(parse_number::<T>, |i| i - 1usize)(i)
}

fn parse_move(i: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_stack_number),
            preceded(tag(" to "), parse_stack_number),
        )),
        |(amount, from, to)| Move { amount, from, to },
    )(i)
}

fn parse_input() -> (Stacks, Vec<Move>) {
    let lines = &mut include_str!("inputs/day5.txt").lines();

    let layers = lines
        .map_while(|line| {
            all_consuming(parse_layer)(line)
                .finish()
                .ok()
                .map(|(_, layer)| layer)
        })
        .collect();

    let stacks = transpose_reverse(layers);

    let moves = lines
        .filter_map(|line| {
            all_consuming(parse_move)(line)
                .finish()
                .map(|(_, mov)| mov)
                .ok()
        })
        .collect();

    (Stacks(stacks), moves)
}

fn transpose_reverse<T: Debug>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part1() -> Result<String> {
    let (mut stacks, moves) = parse_input();
    for mov in moves {
        println!("{stacks:?}");
        println!("{mov:?}");
        stacks.apply(mov);
    }
    println!("{stacks:?}");

    Ok(stacks
        .iter()
        .map(|stack| **stack.last().expect("Empty stack!"))
        .collect::<String>())
}

pub fn part2() -> Result<String> {
    let (mut stacks, moves) = parse_input();
    for mov in moves {
        println!("{stacks:?}");
        println!("{mov:?}");
        stacks.apply_bulk(mov);
    }
    println!("{stacks:?}");

    Ok(stacks
        .iter()
        .map(|stack| **stack.last().expect("Empty stack!"))
        .collect::<String>())
}
