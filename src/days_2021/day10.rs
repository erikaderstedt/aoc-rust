// https://adventofcode.com/2021/day/10
use crate::common::Solution;

enum Evaluation {
    Corrupt(usize),
    Incomplete(usize),
}

fn evaluate(line: &str) -> Evaluation {
    let mut v = [0u8;100];
    let mut n = 0;
    for &b in line.as_bytes().iter() {
        match b {
            b'(' | b'[' | b'<' | b'{' => { v[n] = b; n += 1 },
            b')' => if v[n-1] == b'(' { n -= 1; } else { return Evaluation::Corrupt(3) },
            b']' => if v[n-1] == b'[' { n -= 1; } else { return Evaluation::Corrupt(57) },
            b'}' => if v[n-1] == b'{' { n -= 1; } else { return Evaluation::Corrupt(1197) },
            b'>' => if v[n-1] == b'<' { n -= 1; } else { return Evaluation::Corrupt(25137) },
            _ => panic!("Unexpected character {} in line.", b as char),
        }
    }
    Evaluation::Incomplete(
        v[0..n].into_iter().rev().fold(0, |s, c| 
            5 * s + match c {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => unreachable!(""),
            }))
}

pub fn solve(input: &str) -> Solution {
    let line_status: Vec<Evaluation> = 
        input.lines().map(|line| evaluate(line)).collect();
    
    let m1 = line_status.iter().fold(0, |s, e|
        s + match e {
            Evaluation::Corrupt(b) => *b,
            Evaluation::Incomplete(_) => 0,
        });

    let mut scores: Vec<usize> = line_status.into_iter()
        .filter_map(|e| match e {
            Evaluation::Corrupt(_) => None,
            Evaluation::Incomplete(score) => Some(score),
        }).collect();
    scores.sort_unstable();
    let m2 = scores[scores.len()/2];
    
    Solution::new(m1, m2)
}
