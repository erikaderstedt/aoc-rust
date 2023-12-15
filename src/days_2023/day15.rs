// https://adventofcode.com/2023/day/15

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {

    let p1: u64 = input.split(',').map(|step| {
        step.as_bytes().iter().fold(0u64, |current_value, b| {
            ((current_value + *b as u64) * 17) & 0xff
        })
    }).sum();

    let mut boxes: Vec<Vec<(&str, usize)>> = (0..256).map(|_| Vec::new()).collect();

    for operation in input.split(',') {
        let label: String = operation.chars().take_while(|c| c.is_alphabetic()).collect();
        let id: usize = label.as_bytes().iter().fold(0u64, |current_value, b| {
            ((current_value + *b as u64) * 17) & 0xff
        }) as usize;

        if operation.contains('-') {
            if let Some(i) = boxes[id].iter().position(|l| l.0 == label) {
                boxes[id].remove(i);
            }
        } else if operation.contains('=') {
            let (l2, n) = operation.split_once('=').unwrap();
            let nn = n.parse::<usize>().unwrap();
            if let Some(i) = boxes[id].iter().position(|l| l.0 == label) {
                boxes[id][i].1 = nn;
            } else {
                boxes[id].push((&l2, nn))
            }
        }        
    }

    let p2 = boxes.into_iter().enumerate().fold(0, |total, (index,b)| {
        total + b.iter().enumerate().map(|(slot, lens)| {
            (index + 1) * (slot + 1) * lens.1
        }).sum::<usize>()
    });
    
    Solution::new(p1, p2)
}
