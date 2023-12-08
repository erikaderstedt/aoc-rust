// https://adventofcode.com/2023/day/8

use std::collections::{HashMap, HashSet};
use std::str;

use crate::common::Solution;

fn location_name_to_u32(s: &str, index: usize) -> u32 {
    let mut b = [0u8;4];
    b[1] = s.as_bytes()[0 + index];
    b[2] = s.as_bytes()[1 + index];
    b[3] = s.as_bytes()[2 + index];
    
    u32::from_be_bytes(b)
}

pub fn solve(input: &str) -> Solution {
    let names: Vec<u32> = input.lines()
                            .skip(2)
                            .map(|i| location_name_to_u32(i, 0))
                            .collect();
    let name_to_index: HashMap<u32, usize> = names.iter().enumerate().map(|(index, v)| (*v, index)).collect(); 

    let index_of_aaa = name_to_index.iter().find(|(name, _)| **name == 0x414141).unwrap().1;
    let index_of_zzz = name_to_index.iter().find(|(name, _)| **name == 0x5a5a5a).unwrap().1;

    let indices_of_ghosts: Vec<usize> = name_to_index.iter().filter(|(name, _)| **name & 0xff == 'A' as u32).map(|j| j.1).cloned().collect();

    let next_index_map: Vec<(usize,usize)> = input.lines()
    .skip(2)
    .map(|line| {
        let left = location_name_to_u32(line, 7);
        let right = location_name_to_u32(line, 12);
        (name_to_index[&left], name_to_index[&right])
    })
    .collect();
    
    let dirs = input.lines().next().unwrap();

    let travel_to_zzz = |mut index: usize| -> usize {
        let mut p = 0;
        for d in dirs.chars().cycle() {
            match d {
                'L' => { index = next_index_map[index].0; },
                'R' => { index = next_index_map[index].1; },
                _ => panic!("Invalid direction")
            }
            p += 1;
            if index == *index_of_zzz { break }
        }
        p
    };

    let travel_to_ending_in_z = |mut index: usize| -> usize {
        let mut p = 0;
        for d in dirs.chars().cycle() {
            match d {
                'L' => { index = next_index_map[index].0; },
                'R' => { index = next_index_map[index].1; },
                _ => panic!("Invalid direction")
            }
            p += 1;
            if names[index] & 0xff == 'Z' as u32 { break; }
        }
        p
    };

    let p1 = travel_to_zzz(*index_of_aaa);

    let p2_parts: HashSet<usize> = indices_of_ghosts.into_iter()
        .map(travel_to_ending_in_z)
        .map(|p| prime_factorization::Factorization::run(p as u32)
                .factors.into_iter().map(|d| d as usize))
        .flatten()
        .collect();
        
    let p2: usize = p2_parts.into_iter().product();

    Solution::new(p1,p2)
}
