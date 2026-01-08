// https://adventofcode.com/2016/day/2

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let mut position = '5';

    let mut p1_keys = vec![];
    for line in input.lines() {
        for c in line.chars() {
            position = match c {
                'D' => match position {
                    '1' => '4',
                    '2' => '5',
                    '3' => '6',
                    '4' => '7',
                    '5' => '8',
                    '6' => '9',
                    _ => position,
                },
                'U' => match position {
                    '4' => '1',
                    '5' => '2',
                    '6' => '3',
                    '7' => '4',
                    '8' => '5',
                    '9' => '6',
                    _ => position,
                },
                'L' => match position {
                    '2' => '1',
                    '3' => '2',
                    '5' => '4',
                    '6' => '5',
                    '8' => '7',
                    '9' => '8',
                    _ => position,
                },
                'R' => match position {
                    '1' => '2',
                    '2' => '3',
                    '4' => '5',
                    '5' => '6',
                    '7' => '8',
                    '8' => '9',
                    _ => position,
                },
                _ => panic!("Unknown movement."),
            };
        }
        p1_keys.push(position.clone());
    }

    position = '5';
    let mut p2_keys = vec![];
    for line in input.lines() {
        for c in line.chars() {
            position = match c {
                'D' => match position {
                    '1' => '3',
                    '2' => '6',
                    '3' => '7',
                    '4' => '8',
                    '6' => 'A',
                    '7' => 'B',
                    '8' => 'C',
                    'B' => 'D',
                    _ => position,
                },
                'U' => match position {
                    '3' => '1',
                    '6' => '2',
                    '7' => '3',
                    '8' => '4',
                    'A' => '6',
                    'B' => '7',
                    'C' => '8',
                    'D' => 'B',
                    _ => position,
                },
                'L' => match position {
                    '3' => '2',
                    '4' => '3',
                    '6' => '5',
                    '7' => '6',
                    '8' => '7',
                    '9' => '8',
                    'B' => 'A',
                    'C' => 'B',
                    _ => position,
                },
                'R' => match position {
                    '2' => '3',
                    '3' => '4',
                    '5' => '6',
                    '6' => '7',
                    '7' => '8',
                    '8' => '9',
                    'A' => 'B',
                    'B' => 'C',
                    _ => position,
                },
                _ => panic!("Unknown movement."),
            };
        }
        p2_keys.push(position.clone());
    }

    let p1: String = p1_keys.into_iter().collect();
    let p2: String = p2_keys.into_iter().collect();

    Solution::new(p1, p2)
}
