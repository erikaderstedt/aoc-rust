// https://adventofcode.com/2025/day/2
use prime_factorization::Factorization;
use itertools::Itertools;
use crate::common::Solution;

type ChunkLengthCalculator = fn(u64) -> Vec<u64>;

pub fn solve(input: &str) -> Solution {
    let mut p1 = 0u64;
    let mut p2 = 0u64;

    for chunk in input.split(',') {
        let (start, stop) = chunk.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let stop = stop.parse::<u64>().unwrap();
    
        for (num_digits, start, stop) in split_by_num_digits(start, stop).into_iter() {
            let invalid_id_sum = |f: ChunkLengthCalculator, num_digits: u64| -> u64 {
                masks(f(num_digits), num_digits)
                .into_iter()
                .map(|mask| matches_for_mask_in_range(mask, start, stop))
                .flatten()
                .sorted()
                .dedup()
                .sum::<u64>()
            };
            p1 = p1 + invalid_id_sum(chunk_lengths_p1, num_digits);
            p2 = p2 + invalid_id_sum(chunk_lengths_p2, num_digits);
        }
    }
    
    Solution::new(p1, p2)
}

fn num_digits(i: u64) -> u64 { (i.ilog10() + 1) as u64 }

fn chunk_lengths_p1(total_length: u64) -> Vec<u64> {
    if (total_length & 1) == 0 { vec![total_length / 2] } else { vec![] }
}

fn chunk_lengths_p2(total_length: u64) -> Vec<u64> {
    Factorization::run(total_length).factors.iter().dedup()
        .map(|factor| total_length / factor).collect()
}

fn masks(chunk_lengths: Vec<u64>, total_length: u64) -> Vec<u64> {
    chunk_lengths.into_iter().map(|chunk_length| {
        let divisor = 10u64.pow(chunk_length as u32);
        let num_chunks = total_length / chunk_length;
        (1..num_chunks).fold(1, |acc, _| acc * divisor + 1)
    }).collect()
}

fn split_by_num_digits(start: u64, stop: u64) -> Vec<(u64, u64, u64)> {
    (num_digits(start)..=num_digits(stop))
    .filter(|num_digits| *num_digits > 1)
    .map(|num_digits|
        (num_digits,
         std::cmp::max(start, 10u64.pow((num_digits - 1) as u32)), 
         std::cmp::min(stop, 10u64.pow(num_digits as u32) - 1))
    ).collect()
}

fn matches_for_mask_in_range(mask: u64, start: u64, stop: u64) -> Vec<u64> {
    ((start/mask)..=(stop/mask + 1))
    .map(|i| i * mask)
    .skip_while(|i| *i < start)
    .take_while(|i| *i <= stop)
    .collect()
}