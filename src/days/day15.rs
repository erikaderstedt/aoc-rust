use crate::common::Solution;

fn determine_nth_number_spoken(starting_nums: &[u32], n: u32) -> u32 {
    let mut spoken = vec![0u32; n as usize];
    for i in 0..(starting_nums.len()-1) {
        spoken[starting_nums[i] as usize] = (i+1)  as u32;
    }

    ((starting_nums.len())..(n as usize)).fold(starting_nums[starting_nums.len()-1], |last_spoken, turn_number| {
        let speak = match spoken[last_spoken as usize] {
            0 => 0,
            n => (turn_number as u32) - n,
        };
        spoken[last_spoken as usize] = turn_number as u32;
        speak
    })
}

pub fn solve(_input: &str) -> Solution {
    let nums: [u32;6] = [1,0,15,2,10,13];

    let p1 = determine_nth_number_spoken(&nums, 2020u32);
    let p2 = determine_nth_number_spoken(&nums, 30000000u32);

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}