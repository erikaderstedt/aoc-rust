use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let mut num_valid_according_to_old_policy = 0;
    let mut num_valid_according_to_new_policy = 0;

    for line in input.lines() {
        let (at_least,at_most) = line.split(|c| c == '-' || c == ' ').take(2).map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();

        let p = line.find(':').unwrap();
        let b = line.as_bytes();
        let chr = b[p-1];
        let pwd = b[(p+2)..].to_vec();

        let cnt = bytecount::count(&pwd, chr);
        if at_least <= cnt && cnt <= at_most { 
            num_valid_according_to_old_policy += 1; 
        }
        if (pwd[at_least-1] == chr) != (pwd[at_most-1] == chr) { 
            num_valid_according_to_new_policy += 1; 
        }
    }

    Solution::new(num_valid_according_to_old_policy, num_valid_according_to_new_policy)
}