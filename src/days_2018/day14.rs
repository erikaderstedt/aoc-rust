// https://adventofcode.com/2018/day/14

use itertools::Itertools;
use pathfinding::num_traits::Euclid;

use crate::common::Solution;

fn as_list(n: &usize) -> Vec<u8> {
    (0..=n.ilog10())
        .map(|p| (((*n as u32) / 10u32.pow(p)) % 10) as u8)
        .rev()
        .collect()
}

pub fn solve(input: &str) -> Solution {
    let n = input.trim().parse::<usize>().unwrap();
    let n_as_list = as_list(&n);

    let mut recipes: Vec<u8> = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;
    let mut correct = 0;
    let p2 = loop {
        let result = recipes[e1] + recipes[e2];
        if result >= 10 {
            let (t1, t2) = result.div_rem_euclid(&10);
            recipes.push(t1);
            if t1 == n_as_list[correct] {
                correct = correct + 1;
                if correct == n_as_list.len() {
                    break recipes.len() - n_as_list.len();
                }
            } else {
                correct = 0;
            }
            recipes.push(t2);
        } else {
            recipes.push(result);
        }
        if recipes[recipes.len() - 1] == n_as_list[correct] {
            correct = correct + 1;
            if correct == n_as_list.len() {
                break recipes.len() - n_as_list.len();
            }
        } else {
            correct = 0;
        }

        e1 = (e1 + 1 + (recipes[e1] as usize)).rem_euclid(recipes.len());
        e2 = (e2 + 1 + (recipes[e2] as usize)).rem_euclid(recipes.len());
    };
    let p1 = recipes[n..(n + 10)]
        .iter()
        .map(|v| format!("{}", v))
        .join("");

    Solution::new(p1, p2)
}
