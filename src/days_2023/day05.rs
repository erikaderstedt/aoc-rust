// https://adventofcode.com/2023/day/5

use itertools::Itertools;
use range_ext::intersect::{Intersect, IntersectionExt};
use crate::common::Solution;
use std::ops::Range;

pub fn solve(input: &str) -> Solution {    
    let mut seeds: Vec<u64> = input.lines().next().unwrap().split(": ").last().unwrap().split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
    let mut seed_ranges: Vec<Range<u64>> = (0..(seeds.len() >> 1)).map(|i| seeds[2*i]..(seeds[2*i] + seeds[2*i+1])).collect();

    let maps: Vec<Vec<(u64,Range<u64>)>> = 
        input.split("\n\n").skip(1)
            .map(|p| 
                p.lines().skip(1).map(|line| {
                    let (dest_start, map_start, length) = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect_tuple().unwrap();
                    (dest_start, map_start..(map_start+length))
            })
            .collect()
        ).collect();
    
    for map in maps.iter() {
        for s in seeds.iter_mut() {
            for (destination, source_range) in map.iter() {
                if source_range.contains(s) {
                    *s = *s + destination - source_range.start;
                    break;
                }
            }
        }
    }
    let p1 = seeds.iter().min().unwrap();
    
    for map in maps.iter() {
        seed_ranges = seed_ranges.into_iter().map(|seed| mapped_range(seed, map)).flatten().collect();
    }
    let p2 = seed_ranges.iter().min_by_key(|r| r.start).unwrap().start;

    Solution::new(p1,p2)
}

fn mapped_range(seed: Range<u64>, source_ranges: &Vec<(u64,Range<u64>)>) -> Vec<Range<u64>> {
    let mut new_ranges = Vec::new();
    let mut seed = seed.clone();
    let mut add_remainder_to_output = true;

    for (destination, range) in source_ranges.iter() {
        match seed.intersect_ext(range) {
            IntersectionExt::Empty | IntersectionExt::Less | IntersectionExt::Greater => {},
            IntersectionExt::Within | IntersectionExt::Same => {
                new_ranges.push((seed.start + *destination - range.start)..(seed.end + *destination - range.start));
                add_remainder_to_output = false;
                break;
            },
            IntersectionExt::LessOverlap => {
                new_ranges.push(*destination..(*destination + seed.end - range.start));
                seed.end = range.start;
            },
            IntersectionExt::GreaterOverlap => {
                new_ranges.push((seed.start + *destination - range.start)..(*destination + range.end - range.start));
                seed.start = range.end;
            },
            IntersectionExt::Over => {
                new_ranges.push(*destination..(*destination+(range.end - range.start)));
                new_ranges.extend(mapped_range(range.end..seed.end, source_ranges));
                seed.end = range.start;
            },
        }
    }
    if add_remainder_to_output {
        new_ranges.push(seed);
    }
    new_ranges
}