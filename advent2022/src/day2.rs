use anyhow::{Context, Result};
use enum_utils::FromStr;
use itertools::{process_results, Itertools};

use crate::parse;

#[derive(Debug, FromStr, Copy, Clone)]
enum Move {
    #[enumeration(alias = "A", alias = "X")]
    Rock = 1,

    #[enumeration(alias = "B", alias = "Y")]
    Paper = 2,

    #[enumeration(alias = "C", alias = "Z")]
    Scissors = 3,
}

#[derive(Debug, FromStr, Copy, Clone)]
enum Outcome {
    #[enumeration(rename = "Z")]
    Win = 6,

    #[enumeration(rename = "Y")]
    Tie = 3,

    #[enumeration(rename = "X")]
    Lose = 0,
}

impl Move {
    fn from_requirement(requirement: &Outcome, mov: &Move) -> Move {
        match (requirement, mov) {
            (Outcome::Tie, Move::Rock) => Move::Rock,
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Tie, Move::Paper) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Tie, Move::Scissors) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
        }
    }

    fn against(&self, mov: &Move) -> Outcome {
        match (self, mov) {
            (Move::Rock, Move::Rock) => Outcome::Tie,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Paper, Move::Paper) => Outcome::Tie,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Scissors) => Outcome::Tie,
        }
    }
}

enum Input {
    Guide,
    Plan,
}

fn compute_score(input_type: Input) -> Result<u32> {
    let scores = include_str!("inputs/day2.txt")
        .lines()
        .map(|line| -> Result<u32> {
            let (left, right) = line.split(' ').collect_tuple().context("Failed to parse")?;

            let mov = parse!(left)?;
            let resp = match input_type {
                Input::Guide => parse!(right)?,
                Input::Plan => Move::from_requirement(&parse!(right)?, &mov),
            };

            Ok((resp as u32) + (resp.against(&mov) as u32))
        });

    process_results(scores, |iter| iter.sum())
}

pub fn part1() -> Result<u32> {
    compute_score(Input::Guide)
}

pub fn part2() -> Result<u32> {
    compute_score(Input::Plan)
}
