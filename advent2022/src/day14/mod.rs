pub mod wasm;

use anyhow::Result;

use wasm::Grid;

fn parse_input() -> Grid {
    Grid::parse(include_str!("../inputs/day14.txt"))
}

pub fn part1() -> Result<usize> {
    let mut grid = parse_input();
    loop {
        let done = grid.step();
        if done {
            break;
        }
    }

    Ok(grid.num_settled())
}

pub fn part2() -> Result<usize> {
    Ok(0)
}
