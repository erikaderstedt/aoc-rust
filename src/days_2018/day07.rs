// https://adventofcode.com/2018/day/7


use crate::common::Solution;
// use itertools::Itertools; 


pub fn solve(input: &str) -> Solution {
    // let mut dependencies = vec![0u32;26];

    // for line in input.lines() {
    //     let (start, finish) = line.split(' ').skip(1).step_by(6)
    //     .map(|s| ((s.chars().next().unwrap() as u8) - ('A' as u8)) as usize )
    //     .collect_tuple().unwrap();
    //     dependencies[finish] = dependencies[finish] | (1u32 << start);
    // }
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

    Solution::new(0, 0)
}