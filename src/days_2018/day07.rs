// https://adventofcode.com/2018/day/7


use crate::common::Solution;
// use itertools::Itertools; 

// const NUM_WORKERS = 5;

pub fn solve(input: &str) -> Solution {
    let mut dependencies = vec![1u32 << 31;26];

    for line in input.lines() {
        let must_be_finished = line.as_bytes()[5] - ('A' as u8);
        let can_begin = line.as_bytes()[36] - ('A' as u8);
        dependencies[can_begin as usize] |= 1 << must_be_finished;
    }

    let mut m1: Vec<char> = Vec::new();
    while dependencies.iter().any(|&v| v > 0) {
        let next_step = dependencies.iter().position(|&v| v == 0x80000000).unwrap() as u8;
        for m in dependencies.iter_mut() {
            *m &= !(1u32 << next_step);
        }
        m1.push((next_step + ('A' as u8)) as char);
        dependencies[next_step as usize] &= 0x7fffffff;
    }
    let part_1: String = m1.into_iter().collect();
    println!("{:?}", dependencies);
    // let precursors:u32 = dependencies.iter().fold(0u32, |a, b| a | (*b));
    // let all_used = dependencies.iter().enumerate().fold(precursors, |a, (i,b)| if (*b) > 0 { a | (1u32 << i) } else { a } );
    // let expected_results = all_used.count_ones() as usize;

    // let mut output: Vec<char> = vec![];
    // while output.len() < expected_results {
    //     // Grab the first zero element in dependencies which is also in all_used.

    //     // Set the corresponding bit in dependencies to zero.
    // }

    // for (i, v) in dependencies.iter().enumerate() {
    //     println!("{}, {:026b}", ((i as u8) + 65) as char, v);
    // }
    // println!("{:026b}", all_used);

    Solution { part_1, part_2: "".to_string() }
}