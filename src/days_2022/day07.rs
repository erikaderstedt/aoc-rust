// https://adventofcode.com/2022/day/7

use crate::common::Solution;
use itertools::Itertools;

const DISK_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn merge_last_two(v: &mut Vec<usize>) -> usize {
    let sz = v.pop().unwrap();
    let sz_parent = v.pop().unwrap() + sz;
    v.push(sz_parent);
    sz
}

pub fn solve(input: &str) -> Solution {
    // Our input data does not ls an already ls:d directory
    let mut v: Vec<usize> = vec![0];
    let mut completed_directories = vec![];
    for line in input.lines().skip(1) {
        match &line[0..4] {
            "$ cd" => match &line[5..] {
                "/" => {},
                ".." => completed_directories.push(merge_last_two(&mut v)),
                _ => v.push(0),
            },
            "$ ls" | "dir " => {},
            _ => {
                let sz = line.split_once(" ").unwrap().0.parse::<usize>().unwrap();
                let i = v.pop().unwrap() + sz;
                v.push(i);
            },                        
        }
    }
    // Step back out
    while v.len() > 1 { completed_directories.push(merge_last_two(&mut v)) }
    if let Some(sz) = v.pop() {
        completed_directories.push(sz);
    }

    let p1: usize = completed_directories.iter().filter(|d| **d <= 100000).sum();
    let used_space = completed_directories.last().unwrap();
    let free_space = DISK_SIZE - used_space;
    let required_space = UPDATE_SIZE - free_space;
    let p2: usize = *completed_directories
        .iter()
        .filter(|d| **d >= required_space)
        .sorted()
        .next()
        .unwrap();

    Solution::new(p1,p2)
}

