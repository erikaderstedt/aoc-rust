// https://adventofcode.com/2022/day/20

use itertools::Itertools;
use crate::common::Solution;
use std::collections::VecDeque;

type List = VecDeque<(usize,i64)>;

fn mix(ordering: List, num_iterations: usize) -> i64 {
    let mut mixed = ordering.clone();
    let len = mixed.len() - 1;

    for _i in 0..num_iterations {
        for (label, num) in ordering.iter() {
            let index: usize = mixed.iter().find_position(|j| j.0 == *label).unwrap().0;
            let value = mixed.remove(index).unwrap();
            let new_index = (((index as i64) + *num + (2811589153)*(len as i64)) as usize) % len;
            mixed.insert(new_index, value);
        }    
    }

    let i0 = mixed.iter().find_position(|&j| j.1 == 0i64).unwrap().0;
    [1000,2000,3000].iter()
        .map(|j| mixed[(i0 + *j) % mixed.len()].1)
        .sum()
}

pub fn solve(input: &str) -> Solution {
    let list: List = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .collect();
    let decrypted: List = list
        .iter()
        .map(|(i, n)| (i.clone(), n * 811589153))
        .collect();

    let p1 = mix(list, 1);
    let p2 = mix(decrypted, 10);

    Solution::new(p1,p2)
}
