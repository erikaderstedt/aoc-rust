use itertools::Itertools;

// https://adventofcode.com/2022/day/16
use crate::common::{Solution, parsed_from_each_line};
use std::str::FromStr;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct Room {
    name: String,
    flow_rate: usize,
    tunnels_to_rooms: Vec<String>,
}
impl FromStr for Room {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s[6..8].to_string();
        let flow_rate = s[23..].split(";").next().unwrap().parse::<usize>().unwrap();
        let tunnels_to_rooms = if s.contains("valves") {
            s.split("valves ").skip(1).next().unwrap().split(", ").map(|s| s.to_string()).collect()
        } else {
            let valve = s.split("valve ").skip(1).next().unwrap().to_string();
            vec![valve]
        };
        Ok(Room { name, flow_rate, tunnels_to_rooms })
    }
}

#[derive(Debug,Hash,PartialEq, Eq)]
struct State {
    location: String,
    flow_rate: usize,
    open_valves: Vec<String>,
    // minutes_left: usize,
    total_released_pressure: usize
}

impl State {
    fn new() -> State { State { location: "AA".to_string(), open_valves: vec![], flow_rate: 0, 
    // minutes_left: 30, 
    total_released_pressure: 0 }}

    fn all_valves_are_open(&self, rooms: &HashMap<String,Room>) -> bool {
        rooms.iter().all(|room| room.1.flow_rate == 0 || self.open_valves.contains(room.0))
    }

    fn successor_states(&self, rooms: &HashMap<String,Room>) -> Vec<State> {
        // if self.minutes_left == 0 { return vec![] };

        let mut v = vec![];
        let location = rooms.get(&self.location).unwrap();
        if location.flow_rate > 0 && !self.open_valves.contains(&self.location) {
            let mut valves = self.open_valves.clone();
            valves.push(self.location.clone());
            v.push(State { 
                location: self.location.clone(),
                flow_rate: self.flow_rate + location.flow_rate,
                // minutes_left: self.minutes_left - 1,
                open_valves: valves,
                total_released_pressure: self.total_released_pressure + self.flow_rate,  
            })
        }
        for tunnel in location.tunnels_to_rooms.iter() {
            v.push(State { 
                location: tunnel.clone(), 
                flow_rate: self.flow_rate, 
                open_valves: self.open_valves.clone(), 
                // minutes_left: self.minutes_left - 1, 
                total_released_pressure: self.total_released_pressure + self.flow_rate, 
            })
        }
        v
    }

    fn move_through(rooms: &HashMap<String,Room>) -> usize {

        let mut states: Vec<State> = vec![State::new()];
        for _minute in 1..=30 {
            // Get all possible states
            // Replace all states by their successor states.
            states = states.into_iter()
                .map(|state| state.successor_states(rooms).into_iter())
                .flatten()
                .sorted_by_key(|state| state.total_released_pressure)
                .rev()
                .take(400)
                .collect();

            // if states.iter().any(|state| state.all_valves_are_open(rooms)) {
            //     states = states.into_iter().filter(|state| state.all_valves_are_open(rooms)).collect();
            // }
    
            // println!("Minute: {}, States: {:?}", minute, states);
        }
    
        let best = states.into_iter().max_by_key(|state| state.total_released_pressure).unwrap();
        println!("Best state;: {:?}", best);
        best.total_released_pressure
    }
}


#[derive(Debug,Hash,PartialEq, Eq,Clone)]
struct ElephantHelperState {
    location: String,
    elephant_location: String,
    flow_rate: usize,
    open_valves: Vec<String>,
    last_move_was_to_open_current_valve: bool,
    // minutes_left: usize,
    total_released_pressure: usize
}

impl ElephantHelperState {
    fn new() -> ElephantHelperState { ElephantHelperState { location: "AA".to_string(), 
    elephant_location: "AA".to_string(), open_valves: vec![], flow_rate: 0, 
    last_move_was_to_open_current_valve: false,
    // minutes_left: 30, 
    total_released_pressure: 0 }}

    fn all_valves_are_open(&self, rooms: &HashMap<String,Room>) -> bool {
        rooms.iter().all(|room| room.1.flow_rate == 0 || self.open_valves.contains(room.0))
    }

    fn successor_states(&self, rooms: &HashMap<String,Room>) -> Vec<ElephantHelperState> {
        // if self.minutes_left == 0 { return vec![] };
        // Outer product of self action and elephant action


        let mut v1 = vec![];
        let room = rooms.get(&self.location).unwrap();
        if room.flow_rate > 0 && !self.open_valves.contains(&self.location) {
            let mut valves = self.open_valves.clone();
            valves.push(self.location.clone());
            v1.push(ElephantHelperState { 
                location: self.location.clone(),
                elephant_location: self.elephant_location.clone(),
                flow_rate: self.flow_rate + room.flow_rate,
                // minutes_left: self.minutes_left - 1,
                open_valves: valves,
                last_move_was_to_open_current_valve: true,
                total_released_pressure: self.total_released_pressure + self.flow_rate,  
            })
        }
        for tunnel in room.tunnels_to_rooms.iter() {
            v1.push(ElephantHelperState { 
                location: tunnel.clone(), 
                elephant_location: self.elephant_location.clone(),
                flow_rate: self.flow_rate, 
                open_valves: self.open_valves.clone(), 
                last_move_was_to_open_current_valve: false,
                // minutes_left: self.minutes_left - 1, 
                total_released_pressure: self.total_released_pressure + self.flow_rate, 
            })
        }

        let mut v2 = vec![];
        let room = rooms.get(&self.elephant_location).unwrap();
        if room.flow_rate > 0 && !self.open_valves.contains(&self.elephant_location) {
            let mut valves = self.open_valves.clone();
            valves.push(self.elephant_location.clone());
            v2.push(ElephantHelperState { 
                location: self.location.clone(),
                elephant_location: self.elephant_location.clone(),
                flow_rate: self.flow_rate + room.flow_rate,
                // minutes_left: self.minutes_left - 1,
                open_valves: valves,
                last_move_was_to_open_current_valve: true,
                total_released_pressure: self.total_released_pressure + self.flow_rate,  
            })
        }
        for tunnel in room.tunnels_to_rooms.iter() {
            v2.push(ElephantHelperState { 
                location: self.location.clone(), 
                elephant_location: tunnel.clone(),
                flow_rate: self.flow_rate, 
                open_valves: self.open_valves.clone(), 
                last_move_was_to_open_current_valve: false,
                // minutes_left: self.minutes_left - 1, 
                total_released_pressure: self.total_released_pressure + self.flow_rate, 
            })
        }

        // Merge. 
        iproduct!(v1.into_iter(), v2.into_iter())
            .filter_map(|(my_move,elephant_move)| {
                if my_move.last_move_was_to_open_current_valve && elephant_move.last_move_was_to_open_current_valve {
                    if my_move.location == elephant_move.elephant_location { 
                        None
                    } else {
                        let mut valves = my_move.open_valves.clone();
                        // println!("Current {:?}", self);
                        // println!("{:?}, {:?}", valves, elephant_move.location);
                        // assert!(!valves.contains(&self.elephant_location));
                        valves.push(elephant_move.elephant_location.clone());            
                        let flow_rate_in_elephant_room = rooms.get(&self.elephant_location).unwrap().flow_rate;                        

                        Some(ElephantHelperState {
                            location: my_move.location,
                            elephant_location: elephant_move.elephant_location,
                            flow_rate: my_move.flow_rate + flow_rate_in_elephant_room,
                            open_valves: valves,
                            last_move_was_to_open_current_valve: true,
                            total_released_pressure: self.total_released_pressure + self.flow_rate,
                        }) 
                    }                     
                } else {
                    Some(ElephantHelperState {
                        location: my_move.location,
                        elephant_location: elephant_move.elephant_location,
                        flow_rate: my_move.flow_rate.max(elephant_move.flow_rate),
                        open_valves: if my_move.open_valves.len() > elephant_move.open_valves.len() { my_move.open_valves } else { elephant_move.open_valves },
                        last_move_was_to_open_current_valve: false,
                        total_released_pressure: self.total_released_pressure + self.flow_rate,
                    })        
                }
            })
            .collect()
    }

    fn move_through(rooms: &HashMap<String,Room>) -> usize {

        let mut states: Vec<ElephantHelperState> = vec![ElephantHelperState::new()];
        for _minute in 1..=26 {
            // Get all possible states
            // Replace all states by their successor states.
            states = states.into_iter()
                .map(|state| state.successor_states(rooms).into_iter())
                .flatten()
                .sorted_by_key(|state| state.total_released_pressure)
                .rev()
                .take(100000)
                .collect();
    
            // println!("Minute: {}, States: {:?}", minute, states);
        }
        // 2772 too low
    
        let best = states.into_iter().max_by_key(|state| state.total_released_pressure).unwrap();
        println!("Best state;: {:?}", best);
        best.total_released_pressure
    }
}


pub fn solve(input: &str) -> Solution {
    let rooms: HashMap<String,Room> = input.lines()
        .map(|line| line.parse::<Room>().unwrap())
        .map(|room| (room.name.clone(), room))
        .collect();

    let p1 = State::move_through(&rooms);
    let p2 = ElephantHelperState::move_through(&rooms);

    Solution::new(p1,p2)
}


