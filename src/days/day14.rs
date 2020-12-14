use crate::common::Solution;
use std::collections::HashMap;

const UNTOUCHED: u8 = 'X' as u8;
const FLOATING: u8 = 'X' as u8;
const FLOATING_SET_TO_1: u8 = '2' as u8;
const FLOATING_SET_TO_0: u8 = '3' as u8;

pub fn solve(input: &str) -> Solution {
    let mut current_mask: Vec<u8> = Vec::new();
    let mut mem_space_p1: HashMap<u64,u64> = HashMap::new();
    let mut mem_space_p2: HashMap<u64,u64> = HashMap::new();
    let mut x_locations: Vec<usize> = Vec::new();
    
    for line in input.lines() {
        if line.starts_with("mask") {
            current_mask = line[7..].as_bytes().iter().rev().cloned().collect();
            x_locations = current_mask.iter()
                .enumerate()
                .filter_map(|(i,c)| match *c { FLOATING => Some(i), _ => None })
                .collect();
        } else {
            let i = line.find(']').unwrap();
            let address = line[4..i].parse::<u64>().unwrap();
            let value = line[(i+4)..].parse::<u64>().unwrap();

            let p1_value = current_mask.iter().enumerate().fold(value, |value, (i, c)| {
                if *c != UNTOUCHED {
                    (value - (value & (1 << i))) + ((if *c == '1' as u8 { 1 } else { 0 }) << i)
                } else {
                    value
                }
            });
            mem_space_p1.insert(address, p1_value);

            
            let number_of_x = x_locations.len();
            let number_of_addresses = 1 << number_of_x;
            let mut edited_mask = current_mask.clone();
            for address_index in 0..number_of_addresses {
                for j in 0..number_of_x {
                    edited_mask[x_locations[j]] = if address_index & (1 << j) > 0 { FLOATING_SET_TO_1 } else { FLOATING_SET_TO_0 }                    
                }
                let p2_address = edited_mask.iter().enumerate().fold(address, |address, (i,c)| {
                    if *c == '0' as u8 { 
                        address 
                    } else {
                        (address - (address & (1 << i))) + ((if *c == FLOATING_SET_TO_0 { 0 } else { 1 }) << i)
                    }
                });
               mem_space_p2.insert(p2_address, value);
            }
        }
    }

    let p1:u64 = mem_space_p1.values().sum();
    let p2:u64 = mem_space_p2.values().sum();

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}