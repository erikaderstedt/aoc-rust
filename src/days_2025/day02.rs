// https://adventofcode.com/2025/day/2

use std::ops::{Div, DivAssign};

use crate::common::Solution;

struct ChunkConfig {
    length: usize,
    divisor: usize,
    mask: usize,
}

impl ChunkConfig {
    fn from(total_length: usize, chunk_length: usize) -> ChunkConfig {
        let divisor = match chunk_length {
            1 => 10,
            2 => 100,
            3 => 1000,
            4 => 10000,
            5 => 100000,
            _ => panic!("not implemented")
        };
        let num_chunks = total_length / chunk_length;
        let mask = (1..num_chunks).fold(1, |acc, _| acc * divisor + 1);
        ChunkConfig { length: chunk_length, divisor, mask }
    }
}

pub fn solve(input: &str) -> Solution {

    let mut p1 = 0;
    let mut p2 = 0;
    let factors: Vec<Vec<ChunkConfig>> = (0..=10).map(|l| 
        match l {
            0 | 1 => vec![],
            2 | 3 | 5 | 7 => vec![ChunkConfig::from(l, 1)],
            4 => vec![ChunkConfig::from(4, 2), ChunkConfig::from(4, 1)],
            6 => vec![ChunkConfig::from(6, 3), ChunkConfig::from(6, 2), ChunkConfig::from(6, 1)],
            8 => vec![ChunkConfig::from(8, 4), ChunkConfig::from(8, 2), ChunkConfig::from(8, 1)],
            9 => vec![ChunkConfig::from(9, 1), ChunkConfig::from(9, 3)],
            10 => vec![ChunkConfig::from(10, 5), ChunkConfig::from(10, 2), ChunkConfig::from(10, 1)],
            _ => panic!("unsupported length {}", l),
        }).collect();

    for chunk in input.split(',') {
        let (start, stop) = chunk.split_once('-').unwrap();
        let start: usize = start.parse::<usize>().unwrap();
        let stop: usize = stop.parse::<usize>().unwrap();

        for i in start..=stop {
            let length = (i.ilog10() + 1) as usize;
            for chunk_config in factors[length].iter() {
                let num_chunks = length / chunk_config.length;

                let first_chunk = i.rem_euclid(chunk_config.divisor);
                if first_chunk * chunk_config.mask == i {
                    p2 = p2 + i;
                    if num_chunks == 2 {                        
                        p1 = p1 + i;
                    }
                    break;
                }
            }
        }
    }
    
    Solution::new(p1, p2)
}
