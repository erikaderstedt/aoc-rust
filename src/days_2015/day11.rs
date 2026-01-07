// https://adventofcode.com/2015/day/11

use crate::common::Solution;

fn is_valid(pwd: &Vec<u8>) -> bool {
    if pwd.contains(&('o' as u8))
        || pwd.contains(&('i' as u8))
        || pwd.contains(&('l' as u8))
        || !(0..(pwd.len() - 2)).any(|i| pwd[i] + 2 == pwd[i + 2] && pwd[i] + 1 == pwd[i + 1])
    {
        return false;
    }

    if let Some(first_pair_index) = (0..(pwd.len() - 1)).find(|&i| pwd[i] == pwd[i + 1]) {
        if ((first_pair_index + 2)..(pwd.len() - 1))
            .any(|i| pwd[i] != pwd[first_pair_index] && pwd[i] == pwd[i + 1])
        {
            return true;
        } else {
            return false;
        }
    }

    false
}

fn increment(pwd: &mut Vec<u8>) {
    // Find first non-z from end
    // Increase that and set others to a.
    let i = (0..pwd.len()).rev().find(|i| pwd[*i] != 'z' as u8).unwrap();
    pwd[i] += 1;
    for n in (i + 1)..pwd.len() {
        pwd[n] = 'a' as u8;
    }
}

pub fn solve(input: &str) -> Solution {
    let mut pwd: Vec<u8> = input.trim().chars().map(|v| v as u8).collect();

    while !is_valid(&pwd) {
        increment(&mut pwd);
    }
    let p1: String = String::from_utf8(pwd.clone()).unwrap();
    increment(&mut pwd);

    while !is_valid(&pwd) {
        increment(&mut pwd);
    }
    let p2: String = String::from_utf8(pwd).unwrap();
    Solution::new(p1, p2)
}
