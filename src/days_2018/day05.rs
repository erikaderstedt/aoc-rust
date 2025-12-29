use crate::common::Solution;

const DIFF: u8 = 97 - 65;
fn react(b: &Vec<u8>) -> usize {
    let mut src_index0 = 0;
    let mut num_deleted = 0;
    let mut deleted = vec![false; b.len()];

    while src_index0 < b.len() {
        let mut src_index1 = src_index0 + 1;
        while src_index1 < b.len() && deleted[src_index1] {
            src_index1 = src_index1 + 1;
        }

        if src_index1 < b.len() && b[src_index0].abs_diff(b[src_index1]) == DIFF {
            deleted[src_index0] = true;
            deleted[src_index1] = true;
            num_deleted = num_deleted + 2;
            // Go back from src_index0 to find a not deleted.
            while deleted[src_index0] && src_index0 > 0 {
                src_index0 = src_index0 - 1;
            }
            // If not found, go forward to find first not deleted
            if deleted[src_index0] {
                while deleted[src_index0] && src_index0 < b.len() - 1 {
                    src_index0 = src_index0 + 1;
                }
            }
        } else {
            src_index0 = src_index1;
        }
    }
    b.len() - num_deleted
}

pub fn solve(input: &str) -> Solution {
    let p1 = {
        let i2 = input.trim().as_bytes().to_vec();
        react(&i2)
    };

    let p2 = ('A'..='Z')
        .map(|r| -> usize {
            let mut i2 = input
                .trim()
                .as_bytes()
                .iter()
                .filter(|&v| *v != (r as u8) && *v != (r as u8) + DIFF)
                .cloned()
                .collect();
            react(&mut i2)
        })
        .min()
        .unwrap();

    Solution::new(p1, p2)
}
