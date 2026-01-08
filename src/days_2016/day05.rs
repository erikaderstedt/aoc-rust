// https://adventofcode.com/2016/day/5

use crate::common::Solution;
use itertools::Itertools;

const BATCH: usize = 2_000_000;
const NUM_THREADS: usize = 16;

fn find_password(input: String, start: usize, step: usize) -> Vec<(usize, u8, u8)> {
    (start..)
        .step_by(step)
        .take(BATCH)
        .filter_map(|counter| {
            let digest = md5::compute(format!("{}{}", input, counter));
            if digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & 0xf0) == 0 {
                Some((counter, digest.0[2] & 0xf, digest.0[3] >> 4))
            } else {
                None
            }
        })
        .collect()
}

pub fn solve(input: &str) -> Solution {
    // Keep going until all p2 have been found
    let input = input.trim().to_string();
    let mut start = 0;
    let mut results: Vec<(usize, u8, u8)> = vec![];
    loop {
        let mut handles = vec![];
        for i in 0..NUM_THREADS {
            let j = input.clone();
            let s = start.clone() + i;
            handles.push(std::thread::spawn(move || find_password(j, s, NUM_THREADS)));
        }

        results.extend(handles.into_iter().map(|h| h.join().unwrap()).flatten());
        if results
            .iter()
            .map(|r| r.1)
            .filter(|v| *v < 8)
            .sorted()
            .dedup()
            .count()
            == 8
        {
            break;
        }
        start += NUM_THREADS * BATCH;
    }

    results.sort_by_key(|r| r.0);
    let p1 = results
        .iter()
        .map(|r| format!("{:x}", r.1))
        .take(8)
        .join("");

    let mut p = [' '; 8];
    for r in results.into_iter() {
        if r.1 < 8 && p[r.1 as usize] == ' ' {
            p[r.1 as usize] = if r.2 < 10 {
                (r.2 + ('0' as u8)) as char
            } else {
                (r.2 - 10 + ('a' as u8)) as char
            }
        }
    }
    let p2: String = p.iter().collect();

    Solution::new(p1, p2)
}
