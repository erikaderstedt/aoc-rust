
// https://adventofcode.com/2022/day/16
use crate::common::Solution;
use std::collections::HashMap;
use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::cmp::Ordering;
use std::str::FromStr;

type LocationIndex = u8;

#[derive(Debug)]
struct Tunnel {
    destination: LocationIndex,
    length: u8,
}

#[derive(Debug)]
struct Room {
    flow_rate: usize,
    tunnels: Vec<Tunnel>,
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct SingleOperatorStateInPlay {
    location: LocationIndex,
    open_valves: u64,
    time_expended: usize,
    total_released_pressure: usize
}

impl SingleOperatorStateInPlay {

    fn valve_open(&self, valve: &LocationIndex) -> bool { (self.open_valves >> (*valve as u64)) & 1 == 1 }

    fn all_valves_are_open(&self) -> bool {
        self.open_valves == u64::MAX
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Ord)]

enum SingleOperatorState {
    Ongoing(SingleOperatorStateInPlay),
    Finished(usize)
}

impl PartialOrd for SingleOperatorState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Finished(a), Self::Finished(b)) => b.partial_cmp(a),            
            (Self::Finished(_a), _) => Some(Ordering::Less),
            (_, Self::Finished(_a)) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl SingleOperatorState {
    fn total_released_pressure(&self) -> usize {
        match self {
            Self::Finished(x) => *x,
            Self::Ongoing(x) => x.total_released_pressure,
        }
    }

    fn successor_states(self, graph: &HashMap<LocationIndex, Room>) -> Vec<SingleOperatorState> {
        match self {
            Self::Finished(_x) => vec![self],
            Self::Ongoing(state) => {
                if state.all_valves_are_open() {
                    vec![Self::Finished(state.total_released_pressure)]
                } else {
                    graph[&state.location]
                        .tunnels
                        .iter()
                        .filter(|t| !state.valve_open(&t.destination))
                        .map(|t|                            
                            if state.time_expended + (t.length as usize) + 1 >= 30 {
                                Self::Finished(state.total_released_pressure)
                            } else {
                                let valve_turns_on_at = state.time_expended + (t.length as usize) + 1;
                                Self::Ongoing( SingleOperatorStateInPlay { 
                                    location: t.destination,
                                    open_valves: state.open_valves + (1 << (t.destination as u64)),
                                    time_expended: valve_turns_on_at,
                                    total_released_pressure: state.total_released_pressure + graph[&t.destination].flow_rate * (30 - valve_turns_on_at)
                                })
                            })
                        .collect()
                }
            }
        }
    }

    fn move_through(start_from: &LocationIndex, graph: &HashMap<LocationIndex, Room>) -> usize {
        let initial = {
            let mut valves = u64::MAX; // All valves open.
            // Close all valves with non-zero flow rate
            for (index, room) in graph.iter() {
                if room.flow_rate > 0 {
                    valves ^= (1u64 << (*index as u64)) as u64;
                }
            }

            SingleOperatorState::Ongoing(SingleOperatorStateInPlay { 
                location: *start_from, 
                open_valves: valves,
                time_expended: 0,
                total_released_pressure: 0 
            })
        };

        let mut states: Vec<SingleOperatorState> = vec![initial];
        // There will be as many iterations as there are states in the graph.
        for _iteration in 0..graph.len() {
            states = states.into_iter()
                .map(|state| state.successor_states(graph) )
                .flatten()
                .sorted()
                .dedup_by(|s1,s2| {
                    match (s1, s2) {
                        (SingleOperatorState::Finished(_a), SingleOperatorState::Finished(_b)) => true,
                        _ => false,
                    }
                })
                .collect();
        }
        states.iter().map(|s| s.total_released_pressure()).max().unwrap_or(0)

    }

}



type LocationName = u16;

fn name_to_usize(b: &[u8]) -> LocationName {
    (b[0] as LocationName) * 256 + (b[1] as LocationName)
}

#[allow(dead_code)]
fn usize_to_name(b: LocationName) -> String {
    let b2: [u8;2] = [(b >> 8) as u8, (b & 255) as u8];
    std::str::from_utf8(&b2[..]).unwrap().to_string()
}

#[derive(Debug,PartialEq, Eq)]
struct RawInput {
    name: LocationName,
    flow_rate: usize,
    tunnels: Vec<LocationName>,
}

const START: LocationName = (65 * 256) + 65; // AA

pub fn solve(input: &str) -> Solution {
    let input: Vec<RawInput> = input.lines().map(|line| line.parse::<RawInput>().unwrap()).collect();
    let location_indices: HashMap<LocationName,LocationIndex> = input.iter().enumerate().map(|x| (x.1.name, x.0 as LocationIndex)).collect();

    // Construct graph. For each room with a non-zero flow rate, or the initial room, calculate the shortest path to every other room.
    let graph: HashMap<LocationIndex, Room> = (0..input.len()).filter_map(|i1| {
        if input[i1].name == START || input[i1].flow_rate > 0 {
            let v: Vec<Tunnel> = (0..input.len())
                .filter(|i2| i1 != *i2 && input[*i2].flow_rate > 0)
                .map(|i2| {
                    let length: usize = 
                        bfs(&i1, 
                            |i| input[*i].tunnels.iter().map(|t| location_indices[t] as usize),
                            |i| *i == i2).expect("No path!").len() - 1;
                    Tunnel { destination: (i2 as LocationIndex), length: length as u8 }
                })
                .collect();
            Some((i1 as LocationIndex, Room { flow_rate: input[i1].flow_rate, tunnels: v }))
        } else {
            None
        }
    }).collect();

    let p1 = SingleOperatorState::move_through(&location_indices[&START], &graph);

    Solution::new(p1,0)
}


impl FromStr for RawInput {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let b = line.as_bytes();
        let name = name_to_usize(&b[6..8]);
        let flow_rate = line[23..].split(";").next().unwrap().parse::<usize>().unwrap();
        let tunnels_to_rooms = (0..b.len()).step_by(4).map_while(|i| {
            let start = b.len()-i-2;
            let end = b.len()-i;
            if b[start].is_ascii_uppercase() {
                Some(name_to_usize(&b[start..end]))
            } else {
                None
            }
        }).collect();
        Ok( RawInput { name, flow_rate, tunnels: tunnels_to_rooms })
    }
}
