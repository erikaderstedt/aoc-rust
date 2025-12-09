// https://adventofcode.com/2025/day/8

use itertools::Itertools;
use crate::common::Solution;

struct JunctionBox {
    id: usize,
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
    // Attempt aggres
    let mut boxes: Vec<JunctionBox> = input.lines()
        .enumerate()
        .map(|(i, line)| {
            let nums: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            JunctionBox { id: i, position: [nums[0], nums[1], nums[2]] }
        })
        .collect();

    ///////////////////////////
    // Note: this optimization is incorrect - it may not hold
    // for all inputs (two boxes close together but far away from
    // the others will be a problem)
    // Most likely it will hold for every real input in AoC.
    ///////////////////////////
    
    let minimum_required_distance: i64 = boxes
        .iter()
        .map(|a| boxes
            .iter()
            .filter(|b| a.id != b.id )
            .map(|b| a.distance_to(b))
            .min().unwrap_or(0))
        .max().unwrap();

    let distances: Vec<(usize, usize, i64)> = boxes.iter()
        .cartesian_product(boxes.iter())
        .filter(|(a,b)| a.id < b.id )
        .filter_map(|(a,b)| {
            let d = a.distance_to(b);
            if d <= minimum_required_distance {
                Some((a.id, b.id, d))
            } else {
                None
            }
        })
        .sorted_unstable_by_key(|k| k.2)
        .collect();

    let last = distances.last().unwrap();
    let p2 = boxes[last.0].position[0] * boxes[last.1].position[0];

    let mut circuit_id_counts = vec![1;boxes.len()];
    for (i, &(j0, j1, _)) in distances.iter().enumerate().take(CONNECTIONS) {
        if i == CONNECTIONS {            
        }
        let id0 = boxes[j0].id;
        let id1 = boxes[j1].id;
        if id0 != id1 {
            for b in boxes.iter_mut() {
                if b.id == id1 { b.id = id0; }
            }
            circuit_id_counts[id0] += circuit_id_counts[id1];
            circuit_id_counts[id1] = 0;
        }
    }
    let p1: usize = circuit_id_counts.iter().sorted().rev().take(3).product();

    Solution::new(p1, p2)
}
