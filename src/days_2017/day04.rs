// https://adventofcode.com/2017/day/4

use crate::common::Solution;

fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    if s1 == s2 { 
        return true;
    }
    let mut b1: Vec<u8> = s1.chars().map(|c| c as u8).collect();
    let mut b2: Vec<u8> = s2.chars().map(|c| c as u8).collect();
    
    if b1.iter().sum::<u8>() != b2.iter().sum::<u8>() {
        return false;
    }
    while let Some(v1) = b1.pop() {
        if let Some(i) = b2.iter().position(|v2| *v2 == v1) {
            b2.remove(i);
        } else {
            return false;
        }
    }
    true
}

pub fn solve(input: &str) -> Solution {
    let passphrases: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(' ').collect())
        .collect();

    let p1 = passphrases
        .iter()
        .filter(|passphrase| 
            passphrase
                .iter()
                .all(|&word1| 
                    passphrase
                        .iter()
                        .filter(|word2| word1 == **word2)
                        .count() == 1))
        .count();

    let p2 = passphrases
        .iter()
        .filter(|passphrase| 
            passphrase
                .iter()
                .all(|&word1| 
                    passphrase
                        .iter()
                        .filter(|word2| is_anagram(word1, **word2))
                        .count() == 1))
        .count();
    Solution::new(p1,p2)
}
