use crate::common::Solution;

const DIFF: u8 = 97-65;
fn react(b: &mut Vec<u8>) {
    let mut i = 0;
    while i < b.len() - 1 {
        if b[i] + DIFF == b[i+1] || b[i] == b[i+1] + DIFF {
            b.remove(i);
            b.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
}

pub fn solve(input: &str) -> Solution { 

    let p1 = {
        let mut i2 = input.as_bytes().to_vec();
        react(&mut i2);
        i2.len() - 1
    };

    let p2 = ('A'..='Z').map(|r| -> usize {
        let mut i2 = input.as_bytes().iter().filter(|&v| *v != (r as u8) && *v != (r as u8) + DIFF).cloned().collect();
        react(&mut i2);
        i2.len()
    }).min().unwrap() - 1;

    Solution::new(p1, p2)
}