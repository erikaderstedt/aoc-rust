use crate::common::Solution;
use std::str::FromStr;
use crate::common::parsed_from_each_line;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone,Debug,PartialEq)]
enum Mask {
    Overwrite(bool),
    LeaveUnchanged,
    Floating(bool),
}

impl Mask {

    fn from_char(c: &char) -> Option<Mask> {
        match c {
            '0' => Some(Mask::Overwrite(false)),
            '1' => Some(Mask::Overwrite(true)),
            'X' => Some(Mask::LeaveUnchanged),
            _ => None,
        }
    }
}

fn update_value_with_mask(v: &u64, mask: &Vec<Mask>) -> u64 {
    let mut m2 = v.clone();
    for (i, m) in mask.iter().enumerate() {
        match m {
            Mask::Overwrite(d) => m2 = m2 - (m2 & (1 << i)) + ((if *d { 1 } else { 0 }) << i),
            Mask::LeaveUnchanged => {},
            Mask::Floating(_) => panic!(""),
        }
    }
    m2
}

fn update_value_with_mask2(v: &u64, mask: &Vec<Mask>) -> u64 {
    let mut m2 = v.clone();
    for (i, m) in mask.iter().enumerate() {
        match m {
            Mask::Overwrite(true) => m2 = m2 - (m2 & (1 << i)) + (1 << i),
            Mask::Overwrite(false) => {},
            Mask::Floating(d) => m2 = m2 - (m2 & (1 << i)) + ((if *d { 1 } else { 0 }) << i),
            Mask::LeaveUnchanged => panic!("asdfads")
        }
    }
    m2
}


pub fn solve(input: &str) -> Solution {
    let r = Regex::new(r"^mask = ([01X]+)$").expect("d");
    let r2 = Regex::new(r"^mem\[(\d+)\] = (\d+)$").expect("d2");
    let mut current_mask: Vec<Mask> = Vec::new();
    let mut mem_space: HashMap<u64,u64> = HashMap::new();
    let mut mem_space_p2: HashMap<u64,u64> = HashMap::new();

    for line in input.lines() {
        match r.captures_iter(line).next() {
            Some(cap) => current_mask = cap[1].chars().map(|c| Mask::from_char(&c).expect("msg: &str")).rev().collect(),
            None => match r2.captures_iter(line).next() {
                Some(cap) => {
                    let address = cap[1].parse::<u64>().expect(" 3");
                    let value = cap[2].parse::<u64>().expect(" 1");

                    let m2 = update_value_with_mask(&value, &current_mask);
                    let m1 = address.clone();
                    // let initial = match mem_space.get(&m1) {
                    //     Some(v) => v.clone(),
                    //     None => 0u64,
                    // };                    


                    mem_space.insert(m1, m2);
                    let x_locations: Vec<usize> = current_mask.iter().enumerate().filter_map(|(i,m)| match m {
                        Mask::LeaveUnchanged => Some(i),
                        _ => None }).collect();
                    let num_x = x_locations.len();
                    let num_mems = 1 << num_x;
                    for i in 0..num_mems {
                        let mut mask = current_mask.clone();
                        for j in 0..num_x {
                            mask[x_locations[j]] = Mask::Floating(i & (1 << j) > 0);
                        }
//                        println!("{:?}", mask[0..8].to_vec());
                        let m3 = update_value_with_mask2(&address, &mask);
//                        println!("{} {} {}", value, address,m3);
                        mem_space_p2.insert(m3, value);
                    }
                },
                None => panic!("unknown line"),
            }
        }
    }
    let p1:u64 = mem_space.values().sum();
    let p2:u64 = mem_space_p2.values().sum();

    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}