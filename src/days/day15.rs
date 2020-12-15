use crate::common::Solution;
// use crate::common::parsed_from_each_line;
// use regex::Regex;
// use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution {
    let nums: [usize;6] = [1,0,15,2,10,13];
//    let nums: [usize;3] = [0,3,6];

    let p1 = {
        let mut spoken: HashMap<usize,usize> = nums.iter().cloned().enumerate().map(|(i,v)| (v,i+1)).collect();

        let mut last_was_previously_spoken: Option<usize> = None; 
        let mut p1 = 0;
        
        for turn_number in (nums.len()+1)..=2020 {
            let new_number = match last_was_previously_spoken {
                None => 0,
                Some(v) => v, 
            };
            last_was_previously_spoken = match spoken.get(&new_number) {
                Some(previous_turn) => Some(turn_number - previous_turn),
                None => None,
            };
    //        println!("{} {} {:?}",new_number, turn_number, last_was_previously_spoken);
            spoken.insert(new_number, turn_number);
            p1 = new_number;
        }
        p1
    };

    let p2 = {
        let mut spoken: HashMap<usize,usize> = nums.iter().cloned().enumerate().map(|(i,v)| (v,i+1)).collect();

        let mut last_was_previously_spoken: Option<usize> = None; 
        let mut p1 = 0;
        
        for turn_number in (nums.len()+1)..=30000000 {
            let new_number = match last_was_previously_spoken {
                None => 0,
                Some(v) => v, 
            };
            last_was_previously_spoken = match spoken.get(&new_number) {
                Some(previous_turn) => Some(turn_number - previous_turn),
                None => None,
            };
    //        println!("{} {} {:?}",new_number, turn_number, last_was_previously_spoken);
            spoken.insert(new_number, turn_number);
            p1 = new_number;
        }
        p1
    };

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}