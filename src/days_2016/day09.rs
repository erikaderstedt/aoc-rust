// https://adventofcode.com/2016/day/9

use crate::common::Solution;

fn get_decompressed_length(s: &str, multi_decompression: bool) -> usize {
    let b = s.as_bytes();

    let mut decompressed = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == '(' as u8 {
            i += 1;
            let mut j = i;
            while b[j] != ')' as u8 {
                j += 1;
            }
            let (length, multiplier) = s[i..j].split_once("x").unwrap();
            let length = length.parse::<usize>().unwrap();
            let multiplier = multiplier.parse::<usize>().unwrap();
            j += 1;
            decompressed += multiplier
                * if multi_decompression {
                    get_decompressed_length(&s[j..(j + length)], multi_decompression)
                } else {
                    length
                };
            i = j + length;
        } else {
            decompressed += 1;
            i += 1;
        }
    }
    decompressed
}

pub fn solve(input: &str) -> Solution {
    let p1 = get_decompressed_length(input.trim(), false);
    let p2 = get_decompressed_length(input.trim(), true);

    Solution::new(p1, p2)
}
