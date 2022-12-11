use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};

use super::parsing::{parse_input, Command, FileEntry, Listing};

#[derive(Debug)]
struct FsEntry {
    // Nice for showing solution
    #[allow(unused)]
    path: Utf8PathBuf,
    size: usize,
}

fn total_size(tree: &Tree<FsEntry>, root: &Node<FsEntry>) -> Result<usize> {
    let mut total = root.data().size;
    for child in root.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

pub fn part1() -> Result<usize> {
    let commands = parse_input()?;

    let tree = make_tree(commands)?;

    let sum = tree
        .traverse_pre_order(tree.root_node_id().context("Empty tree!")?)?
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node).expect("Failed to calculate size"))
        .filter(|&size| size <= 100_000)
        .sum::<usize>();

    Ok(sum)
}

pub fn part2() -> Result<usize> {
    let total_space = 70_000_000_usize;
    let needed_free_space = 30_000_000_usize;

    let commands = parse_input()?;

    let tree = make_tree(commands)?;
    let root_id = tree.root_node_id().context("Could not find root!")?;

    let used_space = total_size(&tree, tree.get(root_id)?)?;
    let free_space = total_space.checked_sub(used_space).unwrap();

    let need_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_of_doomed_directory = tree
        .traverse_pre_order(root_id)?
        .filter(|node| !node.children().is_empty())
        .map(|node| total_size(&tree, node).expect("Unable to calculate size!"))
        .filter(|&size| size > need_to_free)
        .min()
        .context("Found no suitable directories")?;

    Ok(size_of_doomed_directory)
}

fn make_tree(commands: Vec<Command>) -> Result<Tree<FsEntry>> {
    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for command in commands {
        match command {
            Command::Cd { directory } => match directory.as_str() {
                "/" => (),
                ".." => {
                    curr = tree
                        .get(&curr)?
                        .parent()
                        .context("Cannot get parent of un-inserted node, or root!")?
                        .clone();
                }
                _ => {
                    let node = Node::new(FsEntry {
                        path: directory.clone(),
                        size: 0,
                    });
                    curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
            Command::Ls { listings } => {
                for listing in listings {
                    if let Listing::File(FileEntry { size, path }) = listing {
                        let node = Node::new(FsEntry { size, path });
                        tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                }
            }
        }
    }

    // let mut s = String::new();
    // tree.write_formatted(&mut s)?;
    // println!("{s}");

    Ok(tree)
}
