// https://adventofcode.com/2017/day/10

use crate::common::Solution;
use itertools::Itertools;

fn round<const N: usize>(lengths: &Vec<usize>) -> Vec<usize> {
    let mut pos = 0;
    let mut skip_size = 0;
    let mut n: Vec<usize> = (0..256).collect();
    for _ in 0..N {
        for length in lengths.iter() {
            for i in 0..(length / 2) {
                let src = (pos + i) & 255;
                let dst = (pos + (length - 1 - i)) & 255;
                let v = n[dst];
                n[dst] = n[src];
                n[src] = v;
            }
            pos = (pos + length + skip_size) & 255;
            skip_size += 1;
        }
    }

    n
}

pub fn solve(input: &str) -> Solution {
    let p1 = {
        let lengths = input
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let v = round::<1>(&lengths);
        v[0] * v[1]
    };

    let p2 = {
        let lengths = input
            .trim()
            .as_bytes()
            .iter()
            .map(|b| *b as usize)
            .chain(vec![17, 31, 73, 47, 23])
            .collect();
        round::<64>(&lengths)
            .chunks(16)
            .map(|c| {
                let value = c.iter().cloned().reduce(|a, b| a ^ b).unwrap();
                format!("{:02x}", value)
            })
            .join("")
    };

    Solution::new(p1, p2)
}
