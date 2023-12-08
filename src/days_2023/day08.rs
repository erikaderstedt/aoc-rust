// https://adventofcode.com/2023/day/8

use std::collections::HashMap;
use crate::common::Solution;

const AAA: u32 = 0x414141;
const ZZZ: u32 = 0x5a5a5a;

fn location_name_to_u32(s: &str, index: usize) -> u32 {
    let mut b = [0u8;4];
    b[1] = s.as_bytes()[0 + index];
    b[2] = s.as_bytes()[1 + index];
    b[3] = s.as_bytes()[2 + index];
    
    u32::from_be_bytes(b)
}

// Relies on two hidden properties of the input
// 1) The distance **A -> **Z is the same as **Z to another **Z.
// 2) The distance **A -> **Z is a multiple of the instruction vector length.
// #2 is easy to do away with, but #1 is not.
// I think #2 follows from #1 but I am not sure.

pub fn solve(input: &str) -> Solution {
    let names: Vec<u32> = input.lines()
                            .skip(2)
                            .map(|i| location_name_to_u32(i, 0))
                            .collect();
    let name_to_index: HashMap<u32, usize> = names.iter().enumerate().map(|(index, v)| (*v, index)).collect(); 
    let next_index_map: Vec<(usize,usize)> = input.lines()
    .skip(2)
    .map(|line| {
        let left = location_name_to_u32(line, 7);
        let right = location_name_to_u32(line, 12);
        (name_to_index[&left], name_to_index[&right])
    })
    .collect();
    let index_of_aaa = name_to_index.iter().find(|(name, _)| **name == AAA).unwrap().1;
    let indices_of_other_ghosts: Vec<usize> = name_to_index.iter()
        .filter(|(name, index)| (**name & 0xff == 'A' as u32) && *index != index_of_aaa)
        .map(|j| j.1).cloned().collect();
    let dirs = input.lines().next().unwrap();
    let instruction_len = dirs.chars().count();

    let travel = |mut index: usize, destination_mask: u32, destination: u32| -> usize {
        let mut p = 0;
        for d in dirs.chars().cycle() {
            match d {
                'L' => { index = next_index_map[index].0; },
                'R' => { index = next_index_map[index].1; },
                _ => panic!("Invalid direction")
            }
            p += 1;
            if (names[index] & destination_mask) == destination { break }
        }
        p
    };

    let p1 = travel(*index_of_aaa, 0xffffff, ZZZ);
    let p2 = indices_of_other_ghosts.into_iter()
        .map(|location| travel(location, 0xff, 'Z' as u32) / instruction_len)
        .product::<usize>() * p1;

    Solution::new(p1,p2)
}
