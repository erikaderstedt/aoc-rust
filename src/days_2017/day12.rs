// https://adventofcode.com/2017/day/12

use std::collections::HashSet;
use itertools::Itertools;
use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let data: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (left_program, programs) = line.split(" <-> ").collect_tuple().unwrap();
            let left = left_program.parse::<usize>().unwrap();
            assert!(left == i);
            programs
                .split(", ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut groups: Vec<usize> = (0..data.len());
    groups[0] = Some(0);

    for (i, p) in data.iter().enumerate() {
        let group_id = p.iter()
            .cloned()
            .chain(vec![i])
            .filter_map(|n| groups[n])
            .min()
            .unwrap_or(i);

        // For each group id in these, set 

        // If any is part of a group, 

        // set the 
    }

    for line in input.lines() {
        let (left_program, programs) = line.split(" <-> ").collect_tuple().unwrap();
        let left = left_program.parse::<usize>().unwrap();
        let programs: Vec<usize> = programs
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        if let Some(group_index) = groups.iter().position(|group| {
            group.contains(&left) || programs.iter().any(|p| group.contains(p))
        }) {
            groups[group_index].insert(left);
            groups[group_index].extend(programs.into_iter());
        } else {
            let mut new_group = HashSet::new();
            new_group.insert(left);
            new_group.extend(programs.into_iter());
            groups.push(new_group);
        }
    }
    let p1 = groups.iter().find(|g| g.contains(&0)).unwrap().len();
    let p2 = groups.len();

    Solution::new(p1,p2)
}
