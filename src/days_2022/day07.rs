// https://adventofcode.com/2022/day/7

use crate::common::Solution;

const DISK_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn merge_last_two(v: &mut Vec<usize>) -> usize {
    if let (Some(sz), Some(sz_parent)) = (v.pop(), v.pop()) {
        v.push(sz_parent + sz);
        sz
    } else { 0 }
}

pub fn solve(input: &str) -> Solution {
    // Our input data does not revisit directories. All directories
    // are completely explored before cd:ing up from them.
    let mut current: Vec<usize> = vec![0];
    let mut completed_directories = vec![];
    for line in input.lines() {
        match &line[0..4] {
            "$ cd" => match &line[5..] {
                "/" => {},
                ".." => completed_directories.push(merge_last_two(&mut current)),
                _ => current.push(0),
            },
            "$ ls" | "dir " => {},
            _ => { // Regular file.
                let sz = line.split_once(" ").unwrap().0.parse::<usize>().unwrap();
                *current.last_mut().unwrap() += sz;
            },                        
        }
    }
    // Step back out
    completed_directories.extend(current
            .into_iter()
            .rev()
            .scan(0, |sum, x| { *sum += x; Some(*sum) }));

    let p1: usize = completed_directories.iter().filter(|d| **d <= 100000).sum();
    let used_space = completed_directories.last().unwrap();
    let free_space = DISK_SIZE - used_space;
    let required_space = UPDATE_SIZE - free_space;
    let p2: usize = *completed_directories
        .iter()
        .filter(|d| **d >= required_space)
        .min()
        .unwrap();

    Solution::new(p1,p2)
}

