// https://adventofcode.com/2023/day/5

use itertools::Itertools;
use range_ext::intersect::{Intersect, IntersectionExt};
use crate::common::Solution;

pub fn solve(input: &str) -> Solution {    

    let mut seeds: Vec<usize> = input.lines().next().unwrap().split(": ").last().unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut seed_ranges: Vec<std::ops::Range<usize>> = (0..(seeds.len() >> 1)).map(|i| seeds[2*i]..(seeds[2*i] + seeds[2*i+1])).collect();
    
    for map in input.split("\n\n").skip(1) {
        let mut moved = vec![false; seeds.len()];

        for r in map.lines().skip(1) {
            let (dest_start, map_start, length) = r.split(' ').map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
            
            for (i,s) in seeds.iter_mut().enumerate() {
                if moved[i] { 
                    continue;
                }
                if (map_start..(map_start+length)).contains(s) {
                    *s = *s - map_start + dest_start;
                    moved[i] = true;
                }
            }
        }
    }
    let p1 = seeds.iter().min().unwrap();
    
    for map in input.split("\n\n").skip(1) {
        let mut moved = vec![false; seed_ranges.len()];
        let mut new_ranges = Vec::new();

        for r in map.lines().skip(1) {
            let (dest_start, map_start, length) = r.split(' ').map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
            let m = map_start..(map_start+length);

            let mut new_unmoved_ranges = Vec::new();
            for (i,s) in seed_ranges.iter_mut().enumerate() {
                if moved[i] { 
                    continue;
                }
                match s.intersect_ext(&m) {
                    IntersectionExt::Empty | IntersectionExt::Less | IntersectionExt::Greater => {},
                    IntersectionExt::Within | IntersectionExt::Same => {
                        s.start = s.start + dest_start - map_start;
                        s.end = s.end + dest_start - map_start;
                        moved[i] = true;
                    },
                    IntersectionExt::LessOverlap => {
                        new_ranges.push(dest_start..(dest_start + s.end - m.start));
                        s.end = m.start;
                    },
                    IntersectionExt::GreaterOverlap => {
                        let length = m.end - s.start;
                        let start = s.start + dest_start - m.start;
                        new_ranges.push(start..(start+length));
                        s.start = m.end;
                    },
                    IntersectionExt::Over => {
                        new_ranges.push(dest_start..(dest_start+length));
                        new_unmoved_ranges.push(m.end..s.end);                    
                        s.end = m.start;
                    },                    
                };
            }

            moved.extend(new_unmoved_ranges.iter().map(|_| false));
            seed_ranges.extend(new_unmoved_ranges.into_iter());

        }
        seed_ranges.extend(new_ranges.into_iter());

    }
    let p2 = seed_ranges.iter().map(|r| r.start).min().unwrap();

    Solution::new(p1,p2)
}
