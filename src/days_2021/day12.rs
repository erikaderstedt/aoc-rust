// https://adventofcode.com/2021/day/12
use crate::common::Solution;
use itertools::Itertools;

enum Cave<'a> {
    Start,
    End,
    Large(&'a str),
    Small(&'a str),
}

impl<'a> Cave<'a> {
    fn from(s: &str) -> Cave {
        match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => { if s.as_bytes()[0] >= ('a' as u8) { 
                Cave::Small(s)
            } else {
                Cave::Large(s)
            }}
        }
    }
}

struct CaveSystem<'a> {
    caves: Vec<(Cave<'a>,Vec<usize>)>,
}

impl<'a> CaveSystem<'a> {
    fn from(s: &str) -> CaveSystem {
        // Build vec of caves
        let cave_names: Vec<&str> = s.lines()
            .flat_map(|line| line.split('-'))
            .unique()
            .collect();
        CaveSystem { 
            caves: cave_names.iter().map(|&name| {
                let destination_indices = s.lines()
                    .filter_map(|line| {
                        let (start, end) = line.split_once('-').unwrap();
                        if name == start {
                            Some(end)
                        } else if name == end && name != "end" && start != "start" {
                            Some(start)
                        } else {
                            None
                        }})
                    .map(|n| cave_names.iter().position(|&c| c == n).unwrap())
                    .collect();
                (Cave::from(name), destination_indices)
            }).collect()
        }
    }

    fn explore(&self, allows_revisits_of_small_caves: bool) -> usize {
        let mut available_actions: Vec<(usize, bool,usize)> = self.caves[0].1.iter().map(
            |destination_index| {
                (1 << 0, allows_revisits_of_small_caves, *destination_index) 
            }
        ).collect();
        
        let mut num_paths = 0;
        while let Some((visited, allows_revisits, next_index)) = available_actions.pop() {
            let havent_been_here = visited & (1 << next_index) == 0;
            match self.caves[next_index].0 {
                Cave::Start => { panic!("We should not have a path leading back to start.") },
                Cave::End => { num_paths += 1; },
                Cave::Large(_) => {
                    available_actions.extend(self.caves[next_index].1.iter().map(|destination_index|{
                        (visited | (1 << next_index), allows_revisits, *destination_index) 
                    })); },
                Cave::Small(_) if allows_revisits || havent_been_here => {
                    available_actions.extend(self.caves[next_index].1.iter().map(|destination_index|{
                        (visited | (1 << next_index), 
                        if havent_been_here { allows_revisits } else { false }, 
                        *destination_index) 
                    }));
                },
                _ => {},
            }
        }
        num_paths
    }
}

pub fn solve(input: &str) -> Solution {
    let cave_system = CaveSystem::from(input);

    let m1 = cave_system.explore(false);
    let m2 = cave_system.explore(true);

    Solution::new(m1,m2)
}


