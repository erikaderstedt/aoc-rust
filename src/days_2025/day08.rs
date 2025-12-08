// https://adventofcode.com/2025/day/8

use itertools::Itertools;
use crate::common::Solution;

#[derive(Debug)]
struct JunctionBox {
    id: usize,
    circuit_id: Option<usize>,
    position: [i64;3]
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> i64 {
        (other.position[0] - self.position[0])*(other.position[0] - self.position[0]) +
        (other.position[1] - self.position[1])*(other.position[1] - self.position[1]) +
        (other.position[2] - self.position[2])*(other.position[2] - self.position[2])
    }
}

const CONNECTIONS: usize = 1000;

pub fn solve(input: &str) -> Solution {
    let mut boxes: Vec<JunctionBox> = input.lines()
        .enumerate()
        .map(|(i, line)| {
            let nums: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            JunctionBox { id: i, circuit_id: None, position: [nums[0], nums[1], nums[2]] }
        })
        .collect();

    let distances: Vec<(usize, usize, i64)> = boxes.iter()
        .cartesian_product(boxes.iter())
        .filter(|(a,b)| {
            a.id < b.id 
        })
        .map(|(a,b)| (a.id, b.id, a.distance_to(b)))
        .sorted_by_key(|k| k.2)
        .collect();

    let mut p1: usize = 0;
    let mut p2: i64 = 0;
    for (i, lowest) in distances.iter().enumerate() {

        match (boxes[lowest.0].circuit_id, boxes[lowest.1].circuit_id) {
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    for b in boxes.iter_mut() {
                        match b.circuit_id {
                            Some(c) if c == c2 => { b.circuit_id = Some(c1) }
                            _ => {},
                        }
                    }
                }
            },
            (Some(c1), None) => { boxes[lowest.1].circuit_id = Some(c1) }
            (None, Some(c2)) => { boxes[lowest.0].circuit_id = Some(c2) }
            (None, None) => {
                let next_circuit_id = match boxes.iter().filter_map(|b| b.circuit_id).max() {
                    Some(i) => i + 1,
                    None => 0
                };
                boxes[lowest.0].circuit_id = Some(next_circuit_id);
                boxes[lowest.1].circuit_id = Some(next_circuit_id);
            }            
        }

        let circuit_ids: Vec<(usize, usize)>  = boxes.iter()
            .filter_map(|b| b.circuit_id)
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|v| v.0)
            .rev()
            .collect();


        if i == CONNECTIONS - 1 {
            p1 = circuit_ids.iter().take(3).map(|x| x.0).product();
        }
        if boxes.iter().all(|b| b.circuit_id.is_some()) && circuit_ids.len() == 1 {
            p2 = boxes[lowest.0].position[0] * boxes[lowest.1].position[0];
            break
        }
    }
    
    Solution::new(p1, p2)
}
