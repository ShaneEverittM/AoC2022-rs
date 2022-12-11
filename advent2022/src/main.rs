use anyhow::Result;

use advent2022::*;

fn main() -> Result<()> {
    // println!("Day 1:");
    // println!("\tPart 1, max calories: {}", day1::part1()?);
    // println!("\tPart 2, total calories of top 3: {}", day1::part2()?);
    //
    // println!("Day 2:");
    // println!("\tPart 1, total score: {}", day2::part1()?);
    // println!("\tPart 2, total score: {}", day2::part2()?);
    //
    // println!("Day 3:");
    // println!("\tPart 1, total priority: {}", day3::part1());
    // println!("\tPart 2, total priority: {}", day3::part2());
    //
    // println!("Day 4:");
    // println!("\tPart 1, total overlaps: {}", day4::part1());
    // println!("\tPart 2, partial overlaps: {}", day4::part2());
    //
    // println!("Day 5:");
    // println!("\tPart 1, top of each stack: {}", day5::part1()?);
    // println!("\tPart 2, top of each stack: {}", day5::part2()?);
    //
    // println!("Day 6:");
    // println!(
    //     "\tPart 1, index of 4 unique: {}",
    //     day6::part1().context("Could not find run of 4")?
    // );
    // println!(
    //     "\tPart 2, index of 14 unique: {}",
    //     day6::part2().context("Could not find run of 4")?
    // );

    println!("Day 7:");
    println!(
        "\tPart 1, sum of directories smaller than 100000: {}",
        day7::part1()?
    );
    println!(
        "\tPart 2, size of directory to delete: {}",
        day7::part2()?
    );

    Ok(())
}
