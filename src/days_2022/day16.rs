
// https://adventofcode.com/2022/day/16
use crate::common::Solution;
use std::collections::HashMap;
use itertools::Itertools; 
use pathfinding::prelude::bfs;
use std::cmp::Ordering;
use std::str::FromStr;

type LocationIndex = u8;

const CAVE_COLLAPSES: usize = 30;

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

#[derive(Debug,Clone,PartialEq,Eq,Ord)]
struct SingleOperatorStateInPlay {
    location: LocationIndex,
    open_valves: u64,
    time_expended: usize,
    total_released_pressure: usize
}

impl PartialOrd for SingleOperatorStateInPlay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_released_pressure.partial_cmp(&other.total_released_pressure)
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Ord)]
struct DualOperatorStateInPlay {
    ego: SingleOperatorStateInPlay,
    elephant: LocationIndex,
    elephant_time_expended: usize,
}

impl PartialOrd for DualOperatorStateInPlay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ego.total_released_pressure.partial_cmp(&other.ego.total_released_pressure)
    }
}
trait OperatorStateMachine {
    type T;

    fn successor_states(self, graph: &HashMap<LocationIndex, Room>) -> Vec<OperatorState<Self::T>>; //where T: OperatorStateMachine + PartialOrd + PartialEq + Eq + Sized;
    fn all_valves_are_open(&self) -> bool;
    fn valve_open(&self, valve: &LocationIndex) -> bool;
    fn total_released_pressure(&self) -> usize;
    fn initial(start_from: &LocationIndex, graph: &HashMap<LocationIndex, Room>) -> Self::T;
    fn dedup(&self, other: &Self::T) -> bool;
}

impl OperatorStateMachine for SingleOperatorStateInPlay {

    fn dedup(&self, _other: &SingleOperatorStateInPlay) -> bool { false }

    fn valve_open(&self, valve: &LocationIndex) -> bool { (self.open_valves >> (*valve as u64)) & 1 == 1 }

    fn all_valves_are_open(&self) -> bool {
        self.open_valves == u64::MAX
    }

    fn successor_states(self, graph: &HashMap<LocationIndex, Room>) -> Vec<OperatorState<Self>> {
        if self.all_valves_are_open() {
            vec![OperatorState::Finished(self.total_released_pressure)]
        } else {
            graph[&self.location]
                .tunnels
                .iter()
                .filter(|t| !self.valve_open(&t.destination))
                .map(|t|                            
                    if self.time_expended + (t.length as usize) + 1 >= CAVE_COLLAPSES {
                        OperatorState::Finished(self.total_released_pressure)
                    } else {
                        let valve_turns_on_at = self.time_expended + (t.length as usize) + 1;
                        OperatorState::Ongoing( Self { 
                            location: t.destination,
                            open_valves: self.open_valves + (1 << (t.destination as u64)),
                            time_expended: valve_turns_on_at,
                            total_released_pressure: self.total_released_pressure + graph[&t.destination].flow_rate * (CAVE_COLLAPSES - valve_turns_on_at)
                        })
                    })
                .collect()
        }
    }

    fn initial(start_from: &LocationIndex, graph: &HashMap<LocationIndex, Room>) -> Self {
        let mut valves = u64::MAX; // All valves open.
        // Close all valves with non-zero flow rate
        for (index, room) in graph.iter() {
            if room.flow_rate > 0 {
                valves ^= (1u64 << (*index as u64)) as u64;
            }
        }

        SingleOperatorStateInPlay { 
            location: *start_from, 
            open_valves: valves,
            time_expended: 0,
            total_released_pressure: 0 
        }
    }

    fn total_released_pressure(&self) -> usize { self.total_released_pressure }

    type T = Self;
}

impl OperatorStateMachine for DualOperatorStateInPlay {

    fn dedup(&self, other: &DualOperatorStateInPlay) -> bool { 
        (self.ego.location == other.elephant && self.ego.time_expended == other.elephant_time_expended) ||
        (self.elephant == other.ego.location && self.elephant_time_expended == other.ego.time_expended)
     }
    
    fn valve_open(&self, valve: &LocationIndex) -> bool { self.ego.valve_open(valve) }
    fn all_valves_are_open(&self) -> bool { self.ego.all_valves_are_open() }

    fn successor_states(self, graph: &HashMap<LocationIndex, Room>) -> Vec<OperatorState<Self>> {
        if self.all_valves_are_open() {
            vec![OperatorState::Finished(self.ego.total_released_pressure)]
        } else {
            // Extend the possible ego moves with elephant moves.
            graph[&self.ego.location]
                .tunnels
                .iter()
                .filter(|t| !self.valve_open(&t.destination))
                .map(|t|                            
                    if self.ego.time_expended + (t.length as usize) + 1 >= CAVE_COLLAPSES {
                        OperatorState::Finished(self.ego.total_released_pressure)
                    } else {
                        let valve_turns_on_at = self.ego.time_expended + (t.length as usize) + 1;
                        OperatorState::Ongoing( Self { 
                            ego: SingleOperatorStateInPlay { 
                                location: t.destination,
                                open_valves: self.ego.open_valves + (1 << (t.destination as u64)),
                                time_expended: valve_turns_on_at,
                                total_released_pressure: self.ego.total_released_pressure + graph[&t.destination].flow_rate * (CAVE_COLLAPSES - valve_turns_on_at)
                            },
                            elephant: self.elephant,
                            elephant_time_expended: self.elephant_time_expended,
                        })
                    })
                .chain(
                    graph[&self.elephant]
                    .tunnels
                    .iter()
                    .filter(|t| !self.valve_open(&t.destination))
                    .map(|t|                            
                        if self.elephant_time_expended + (t.length as usize) + 1 >= CAVE_COLLAPSES {
                            OperatorState::Finished(self.ego.total_released_pressure)
                        } else {
                            let valve_turns_on_at = self.elephant_time_expended + (t.length as usize) + 1;
                            OperatorState::Ongoing( Self { 
                                ego: SingleOperatorStateInPlay { 
                                    location: self.ego.location,
                                    open_valves: self.ego.open_valves + (1 << (t.destination as u64)),
                                    time_expended: self.ego.time_expended,
                                    total_released_pressure: self.ego.total_released_pressure + graph[&t.destination].flow_rate * (CAVE_COLLAPSES - valve_turns_on_at)
                                },
                                elephant: t.destination,
                                elephant_time_expended: valve_turns_on_at,
                            })
                        })                    
                )
                .collect()
        }
    }

    fn initial(start_from: &LocationIndex, graph: &HashMap<LocationIndex, Room>) -> Self {
        let mut valves = u64::MAX; // All valves open.
        // Close all valves with non-zero flow rate
        for (index, room) in graph.iter() {
            if room.flow_rate > 0 {
                valves ^= (1u64 << (*index as u64)) as u64;
            }
        }

        DualOperatorStateInPlay {
            ego: SingleOperatorStateInPlay { 
                location: *start_from, 
                open_valves: valves,
                time_expended: 4,
                total_released_pressure: 0 
            },
            elephant: *start_from,
            elephant_time_expended: 4,
        }
    }

    fn total_released_pressure(&self) -> usize { self.ego.total_released_pressure }

    type T = Self;
}

#[derive(Debug,Clone,PartialEq,Eq,Ord)]
enum OperatorState<T>  {
    Ongoing(T),
    Finished(usize)
}

impl<T: Ord> PartialOrd for OperatorState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Finished(a), Self::Finished(b)) => b.partial_cmp(a),            
            (Self::Finished(_a), _) => Some(Ordering::Less),
            (_, Self::Finished(_a)) => Some(Ordering::Greater),
            (Self::Ongoing(a), Self::Ongoing(b)) => b.partial_cmp(a),
        }
    }
}

impl<T: OperatorStateMachine<T=T> + Ord> OperatorState<T> {
    fn total_released_pressure(&self) -> usize {
        match self {
            Self::Finished(x) => *x,
            Self::Ongoing(x) => x.total_released_pressure(),
        }
    }

    fn successor_states(self, graph: &HashMap<LocationIndex, Room>) -> Vec<OperatorState<T>> {
        match self {
            Self::Finished(_x) => vec![self],
            Self::Ongoing(state) => state.successor_states(graph),
        }
    }

    fn move_through(start_from: &LocationIndex, graph: &HashMap<LocationIndex, Room>) -> usize {
        let initial = T::initial(start_from, graph);

        let mut states: Vec<OperatorState<T>> = vec![OperatorState::Ongoing(initial)];
        // There will be as many iterations as there are states in the graph.
        for _iteration in 0..graph.len() {
            states = states.into_iter()
                .map(|state| state.successor_states(graph) )
                .flatten()
                .sorted()
                .dedup_by(|s1,s2| {
                    match (s1, s2) {
                        (OperatorState::Finished(_a), OperatorState::Finished(_b)) => true,
                        (OperatorState::Ongoing(a), OperatorState::Ongoing(b)) => a.dedup(b),
                        _ => false,
                    }
                })
                .take(500) // TODO: I would like a better condition, but this works (for my input). If in doubt, increase this 10x and see if you get the same result.               
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

    let p1 = OperatorState::<SingleOperatorStateInPlay>::move_through(&location_indices[&START], &graph);
    let p2 = OperatorState::<DualOperatorStateInPlay>::move_through(&location_indices[&START], &graph);

    Solution::new(p1,p2)
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
