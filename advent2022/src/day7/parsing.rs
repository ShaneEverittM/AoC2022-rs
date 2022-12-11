use anyhow::Result;
use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alphanumeric1, digit1, line_ending, multispace0, multispace1, not_line_ending,
    },
    combinator::{all_consuming, map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub(super) enum Command {
    Cd { directory: Utf8PathBuf },
    Ls { listings: Vec<Listing> },
}

#[derive(Debug)]
pub(super) struct DirectoryEntry {
    // Nice for showing solution
    #[allow(unused)]
    pub name: Utf8PathBuf,
}

#[derive(Debug)]
pub(super) struct FileEntry {
    pub size: usize,
    pub path: Utf8PathBuf,
}

#[derive(Debug)]
pub(super) enum Listing {
    Directory(DirectoryEntry),
    File(FileEntry),
}

fn prompt(i: &str) -> IResult<&str, &str> {
    terminated(tag("$"), multispace1)(i)
}

fn cd(i: &str) -> IResult<&str, Command> {
    let (i, _) = preceded(tag("cd"), multispace1)(i)?;
    let (i, directory) = alt((alphanumeric1, tag(".."), tag("/")))(i)?;

    Ok((
        i,
        Command::Cd {
            directory: directory.into(),
        },
    ))
}

fn directory(i: &str) -> IResult<&str, DirectoryEntry> {
    map(
        preceded(tuple((tag("dir"), multispace1)), alphanumeric1),
        |s: &str| DirectoryEntry { name: s.into() },
    )(i)
}

fn file_entry(i: &str) -> IResult<&str, FileEntry> {
    map_res(
        separated_pair(digit1, multispace1, not_line_ending),
        |(size, name): (&str, &str)| {
            size.parse::<usize>().map(|size| FileEntry {
                size,
                path: name.into(),
            })
        },
    )(i)
}

fn listing(i: &str) -> IResult<&str, Listing> {
    alt((
        map(directory, Listing::Directory),
        map(file_entry, Listing::File),
    ))(i)
}

fn ls(i: &str) -> IResult<&str, Command> {
    let (i, _) = preceded(tag("ls"), multispace1)(i)?;
    let (i, listings) = separated_list1(line_ending, listing)(i)?;

    Ok((i, Command::Ls { listings }))
}

fn command(i: &str) -> IResult<&str, Command> {
    alt((cd, ls))(i)
}

fn command_line(i: &str) -> IResult<&str, Command> {
    delimited(prompt, command, multispace0)(i)
}

pub(super) fn parse_input() -> Result<Vec<Command>> {
    let input = include_str!("../inputs/day7.txt");

    let commands = all_consuming(many1(command_line))(input)
        .finish()
        .map(|(_, commands)| commands)?;

    Ok(commands)
}
