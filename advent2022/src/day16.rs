use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap},
    fmt,
    hash::Hash,
    iter,
};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    combinator::all_consuming,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Name([u8; 2]);

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Name {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(take(2usize), |slice: &str| {
            Self(
                slice
                    .as_bytes()
                    .try_into()
                    .expect("Took two bytes, cannot fail"),
            )
        })(i)
    }
}

#[derive(Debug)]
struct Valve {
    name: Name,
    flow: u64,
    links: Vec<Name>,
}

impl Valve {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(tag("Valve "), Name::parse),
                preceded(tag(" has flow rate="), complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), Name::parse),
                ),
            )),
            |(name, flow, links)| Self { name, flow, links },
        )(i)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Flow(u64);

type Connections = HashMap<Name, (Path, Flow)>;

struct Network {
    valves: HashMap<Name, (Valve, Connections)>,
}

type Path = Vec<(Name, Name)>;
type Best = HashMap<BTreeSet<Name>, u64>;

impl Network {
    fn new() -> Self {
        let mut net = Self {
            valves: include_str!("inputs/day16.txt")
                .lines()
                .map(|l| all_consuming(Valve::parse)(l).finish().unwrap().1)
                // start off with zero connections (since we're still parsing)
                .map(|valve| (valve.name, (valve, Connections::default())))
                .collect(),
        };
        let names = net.valves.keys().copied().collect::<Vec<_>>();
        for name in names {
            // fill in the connections as needed
            let conns = net.connections(name);
            net.valves.get_mut(&name).unwrap().1 = conns;
        }
        net
    }

    fn connections(&self, start: Name) -> Connections {
        // this used to be just `HashMap<Name, Path>`, it's a bit hairier now
        let mut current: HashMap<Name, (Path, Flow)> = Default::default();
        {
            let valve = &self.valves[&start].0;
            current.insert(start, (vec![], Flow(valve.flow)));
        }

        let mut connections = current.clone();

        while !current.is_empty() {
            let mut next: HashMap<Name, (Path, Flow)> = Default::default();
            for (name, (path, _flow)) in current {
                for link in self.valves[&name].0.links.iter().copied() {
                    let valve = &self.valves[&link].0;
                    if let Entry::Vacant(e) = connections.entry(link) {
                        let conn_path: Path = path
                            .iter()
                            .copied()
                            .chain(iter::once((name, link)))
                            .collect();
                        let item = (conn_path.clone(), Flow(valve.flow));
                        e.insert(item.clone());
                        next.insert(link, item);
                    }
                }
            }
            current = next;
        }

        connections
    }
}

#[derive(Debug, Clone)]
struct Move<'a> {
    reward: u64,
    target: Name,
    path: &'a Path,
}

impl Move<'_> {
    fn cost(&self) -> u64 {
        let time_to_travel = self.path.len();
        let time_to_open = 1;

        (time_to_travel + time_to_open) as _
    }
}

#[derive(Clone)]
struct State<'a> {
    network: &'a Network,
    position: Name,
    max_turns: u64,
    turn: u64,
    pressure: u64,
    opened_valves: BTreeSet<Name>,
}

impl State<'_> {
    fn turns_remaining(&self) -> u64 {
        self.max_turns - self.turn
    }

    fn apply(&self, mv: &Move) -> Self {
        let mut next = self.clone();
        next.position = mv.target;
        next.turn += mv.cost();
        next.pressure += mv.reward;
        next.opened_valves.insert(mv.target);
        next
    }

    fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        let (_valve, connections) = &self.network.valves[&self.position];
        connections.iter().filter_map(|(name, (path, flow))| {
            if self.opened_valves.contains(name) {
                return None;
            }

            if flow.0 == 0 {
                return None;
            }

            let travel_turns = path.len() as u64;
            let open_turns = 1_u64;
            let turns_spent_open = self
                .turns_remaining()
                .checked_sub(travel_turns + open_turns)?;
            let reward = flow.0 * turns_spent_open;
            Some(Move {
                reward,
                target: *name,
                path,
            })
        })
    }

    fn apply_best_moves(&self, best: &mut Best) -> Self {
        let mut best_state = self.clone();

        best.entry(self.opened_valves.clone())
            .and_modify(|v| {
                if self.pressure > *v {
                    *v = self.pressure
                }
            })
            .or_insert(self.pressure);

        for mv in self.moves() {
            let next = self.apply(&mv).apply_best_moves(best);
            if next.pressure > best_state.pressure {
                best_state = next;
            }
        }
        best_state
    }
}

fn run_simulation() -> (u64, u64) {
    let part1_pressure;
    let part2_pressure;

    let net = Network::new();
    let p1_state = State {
        network: &net,
        position: Name(*b"AA"),
        max_turns: 30,
        turn: 0,
        pressure: 0,
        opened_valves: Default::default(),
    };

    // Part 1
    {
        let mut best = Best::default();
        let state = p1_state.apply_best_moves(&mut best);
        part1_pressure = state.pressure;
    }

    // Part 2
    {
        let mut best = Best::default();
        let mut p2_state = p1_state.clone();
        p2_state.max_turns = 26;
        p2_state.apply_best_moves(&mut best);

        let best_pressure = best
            .iter()
            .tuple_combinations()
            .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
            .map(|(human, elephant)| human.1 + elephant.1)
            .max()
            .unwrap();

        part2_pressure = best_pressure;
    }

    (part1_pressure, part2_pressure)
}

pub fn part1() -> Result<u64> {
    Ok(run_simulation().0)
}

pub fn part2() -> Result<u64> {
    Ok(run_simulation().1)
}
