// https://adventofcode.com/2017/day/10

use itertools::Itertools;

use crate::common::Solution;

fn round<const N: usize>(lengths: &Vec<u8>) -> Vec<u8> {
    let mut pos = 0;
    let mut skip_size = 0;
    let mut n: Vec<u8> = (0..=255).collect();
    for _ in 0..N {
        for length in lengths.iter() {
            for i in 0..(length / 2) {
                let src = (pos + i) & 255;
                let dst = (pos + (length - 1 - i)) & 255;
                let v = n[dst as usize];
                n[dst as usize] = n[src as usize];
                n[src as usize] = v;
            }
            pos = (pos + length + skip_size) & 255;
            skip_size += 1;
        }
    }

    n
}

pub fn knot_hash(s: &str) -> [u8; 16] {
    let lengths = s
        .as_bytes()
        .iter()
        .cloned()
        .chain(vec![17, 31, 73, 47, 23])
        .collect();
    round::<64>(&lengths)
        .chunks(16)
        .map(|c| c.iter().cloned().reduce(|a, b| a ^ b).unwrap())
        .collect_array()
        .unwrap()
}

pub fn solve(input: &str) -> Solution {
    let p1 = {
        let lengths = input
            .trim()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        let v = round::<1>(&lengths);
        (v[0] as usize) * (v[1] as usize)
    };

    let p2 = knot_hash(input.trim())
        .iter()
        .map(|c| format!("{:02x}", c))
        .join("");

    Solution::new(p1, p2)
}
