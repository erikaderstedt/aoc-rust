// https://adventofcode.com/2015/day/4

use crate::common::Solution;
use md5;

const NUM_THREADS: usize = 10;
const BATCH: usize = 1_000_000;

pub fn solve(input: &str) -> Solution {
    let input = input.trim().to_string();
    let mut start = 0;
    let p1 = loop {
        if let Some(counter) = find_adventcoin::<5>(input.clone(), start, 1) {
            break counter;
        }
    };
    // The single-thread runtime is about 1.5 seconds, which is by
    // far the longest this year. I normally don't multi-thread but
    // for this problem I'll make an exception.
    // With this, the runtime is around 240 ms. This is quite a bit more than
    // 1/10th but that is expected since we will do more total work (keep
    // checking numbers above the answer, on threads that did not catch the answer)
    start = p1;
    let p2 = loop {
        let mut handles = vec![];
        for i in 0..NUM_THREADS {
            let j = input.clone();
            let s = start.clone() + i;
            handles.push(std::thread::spawn(move || {
                find_adventcoin::<6>(j, s, NUM_THREADS)
            }));
        }

        if let Some(counter) = handles.into_iter().filter_map(|h| h.join().unwrap()).min() {
            break counter;
        }
        start += BATCH * NUM_THREADS;
    };

    Solution::new(p1, p2)
}

fn find_adventcoin<const N: usize>(input: String, start: usize, step: usize) -> Option<usize> {
    (start..).step_by(step).take(BATCH).find(|counter| {
        let digest = md5::compute(format!("{}{}", input, counter));
        digest.0[0] == 0
            && digest.0[1] == 0
            && ((N == 5 && (digest.0[2] & 0xf0) == 0) || (N == 6 && digest.0[2] == 0))
    })
}
