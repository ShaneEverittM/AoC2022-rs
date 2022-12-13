use anyhow::{Context, Result};
use nom::{
    branch::alt,
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, eof, map_res},
    multi::many1,
    sequence::terminated,
    Finish, IResult,
};

fn parse_line(i: &str) -> IResult<&str, Vec<u8>> {
    terminated(
        many1(map_res(one_of("0123456789"), |num| {
            num.to_string().parse::<u8>()
        })),
        alt((line_ending, eof)),
    )(i)
}

fn parse_input() -> Result<Vec<Vec<u8>>> {
    let input = include_str!("inputs/day8.txt");
    let result = all_consuming(many1(parse_line))(input)
        .finish()
        .map(|(_, grid)| grid)?;

    Ok(result)
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn plane(grid: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..grid.len())
        .into_iter()
        .flat_map(|y| (0..grid[0].len()).into_iter().map(move |x| (x, y)))
}

fn walk_towards_edges(
    grid: &[Vec<u8>],
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> impl Iterator<Item = &u8> {
    (1..).into_iter().map_while(move |i| {
        let x = x.checked_add_signed(dx * i)?;
        let y = y.checked_add_signed(dy * i)?;
        grid.get(x)?.get(y)
    })
}

fn visible_trees_in_direction(
    grid: &[Vec<u8>],
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> usize {
    let line = walk_towards_edges(grid, (x, y), (dx, dy));

    let mut total = 0;
    let treehouse_height = grid[x][y];
    for height in line {
        total += 1;
        if height >= &treehouse_height {
            break;
        }
    }

    total
}

fn scenic_score(grid: &[Vec<u8>], (x, y): (usize, usize)) -> usize {
    DIRECTIONS
        .into_iter()
        .map(|(dx, dy)| visible_trees_in_direction(grid, (x, y), (dx, dy)))
        .product()
}

pub fn part1() -> Result<usize> {
    let grid = parse_input()?;

    let visible = plane(&grid)
        .filter(|&(x, y)| {
            let tree_height = grid[x][y];
            DIRECTIONS.iter().any(|&(dx, dy)| {
                let mut trees_towards_edge = walk_towards_edges(&grid, (x, y), (dx, dy));
                trees_towards_edge.all(|height| height < &tree_height)
            })
        })
        .count();

    Ok(visible)
}

pub fn part2() -> Result<usize> {
    let grid = parse_input()?;

    let plane = plane(&grid);

    let spot = plane.map(|(x, y)| scenic_score(&grid, (x, y))).max();

    spot.context("Could not find perfect spot!")
}
