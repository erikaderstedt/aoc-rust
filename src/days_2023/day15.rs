// https://adventofcode.com/2023/day/15

use crate::common::Solution;

enum Operation {
    InsertOrReplace(usize),
    Remove
}

const EMPTY : Vec<(&str, usize)> = Vec::new();

pub fn solve(input: &str) -> Solution {

    let mut p1 = 0;
    let mut boxes: [Vec<(&str, usize)>;256] = [EMPTY; 256];

    for operation_description in input.split(',') {
        let len = operation_description.len();
        let (label, operation) = if operation_description.contains('-') {
            (&operation_description[0..len-1], Operation::Remove)
        } else if operation_description.contains('=') {
            (&operation_description[0..len-2], Operation::InsertOrReplace(operation_description[len-1..len].parse::<usize>().unwrap()))
        } else {
            panic!("Invalid operation")
        };

        let id: usize = label.as_bytes().iter().fold(0u64, |current_value, b| {
            ((current_value + *b as u64) * 17) & 0xff
        }) as usize;

        p1 += id;

        match operation {
            Operation::Remove => {
                if let Some(i) = boxes[id].iter().position(|l| l.0 == label) {
                    boxes[id].remove(i);
                }
            },
            Operation::InsertOrReplace(focal_length) => {
                if let Some(i) = boxes[id].iter().position(|l| l.0 == label) {
                    boxes[id][i].1 = focal_length;
                } else {
                    boxes[id].push((label, focal_length))
                }
            }
        }
    }

    let p2 = boxes.iter().enumerate().fold(0, |total, (index,b)| {
        total + (index + 1) * b.iter().enumerate().map(|(slot, lens)| (slot + 1) * lens.1 ).sum::<usize>()
    });
    
    Solution::new(p1, p2)
}
