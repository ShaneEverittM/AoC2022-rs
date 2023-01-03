pub mod wasm;

use anyhow::Result;

use wasm::Grid;

fn parse_input(floor: bool) -> Grid {
    Grid::parse(include_str!("../inputs/day14.txt"), floor)
}

fn run_sim(floor: bool) -> Result<usize> {
    let mut grid = parse_input(floor);
    loop {
        let done = grid.step();
        if done {
            break;
        }
    }

    Ok(grid.num_settled())
}

pub fn part1() -> Result<usize> {
    run_sim(false)
}

pub fn part2() -> Result<usize> {
    run_sim(true)
}
