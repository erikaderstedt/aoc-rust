// https://adventofcode.com/2022/day/20

use itertools::Itertools;
use crate::common::Solution;

fn mix(unmixed: Vec<i64>, num_iterations: usize) -> i64 {
    let mut labels: Vec<u16> = (0..unmixed.len()).map(|v| v as u16).collect(); 
    let mut values = unmixed.clone();
    let len = values.len() - 1;
    
    for _i in 0..num_iterations {
        for label in 0u16..(unmixed.len() as u16) {
            let mut index = 0;
            while labels[index] != label {
                index += 1;
            }
            let l = labels.remove(index);
            let v = values.remove(index);
            let new_index = (((index as i64) + v + (2 * 811589153)*(len as i64)) as usize) % len;
            labels.insert(new_index, l);
            values.insert(new_index, v);
        }    
    }

    let i0 = values.iter().find_position(|&j| *j == 0i64).unwrap().0;
    [1000,2000,3000].iter()
        .map(|j| values[(i0 + *j) % values.len()])
        .sum()
}

pub fn solve(input: &str) -> Solution {
    let list: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let decrypted: Vec<i64> = list
        .iter()
        .map(|n| n * 811589153)
        .collect();

    let p1 = mix(list, 1);
    let p2 = mix(decrypted, 10);

    Solution::new(p1,p2)
}
