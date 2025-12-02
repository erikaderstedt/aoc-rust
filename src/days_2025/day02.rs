// https://adventofcode.com/2025/day/2

use crate::common::Solution;

struct Range {
    start: usize,
    stop: usize,
}

impl Range {

    fn num_invalid_ids(&self) -> (usize, usize) {
        let factors: Vec<Vec<usize>> = (0..=10).map(|l| 
            match l {
                0 | 1 => vec![],
                2 | 3 | 5 | 7 => vec![1],
                4 => vec![2,1],
                6 => vec![3,2,1],
                8 => vec![4,2,1],
                9 => vec![1,3],
                10 => vec![5,2,1],
                _ => panic!("unsupported length {}", l),
            }).collect();

        let mut p1 = 0;
        let mut p2 = 0;
        for i in self.start..=self.stop {
            
            let s = i.to_string();
            let l = s.len();

            for k in factors[l].iter() {
                let m = l / k; // number of chunks
                let p0 = &s[0..*k];
                if (1..m).all(|q| s[q*(*k)..(q+1)*(*k)] == *p0) {
                    p2 = p2 + i;
                    if m == 2 {                        
                        p1 = p1 + i;
                    }
                    break;
                }
            }
        }
        (p1, p2)
    }
}

pub fn solve(input: &str) -> Solution {
    let ranges: Vec<Range> = input.split(',').map(|s| {
        let (start, stop) = s.split_once('-').unwrap();
        let start = start.parse::<usize>().unwrap();
        let stop = stop.parse::<usize>().unwrap();
        Range { start, stop }
    }).collect();

    let num_valids: Vec<(usize, usize)> = ranges.iter().map(|r| r.num_invalid_ids()).collect();
    let p1: usize = num_valids.iter().map(|r| r.0).sum();
    let p2: usize = num_valids.iter().map(|r| r.1).sum();
    
    Solution::new(p1, p2)
}
