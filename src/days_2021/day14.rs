// https://adventofcode.com/2021/day/12
use crate::common::Solution;
use std::collections::HashMap;
type Pair = (u8,u8);
type Polymer = HashMap<Pair,usize>;

fn most_vs_least_common(polymer: &Polymer, initial_byte: u8) -> usize {
	let mut counts: HashMap<u8, usize> = HashMap::new();
	for (pair, count) in polymer.iter() {
		*counts.entry(pair.1).or_insert(0) += count;
	}
	*counts.entry(initial_byte).or_insert(0) += 1;
	let most_common: usize = counts.iter().map(|(_, c)| *c).max().unwrap();
	let least_common: usize = counts.iter().map(|(_, c)| *c).min().unwrap();
	most_common - least_common
}

pub fn solve(input: &str) -> Solution {
	let reaction_pairs: HashMap<Pair,(Pair,Pair)> = input
		.lines()
		.skip(2)
		.map(|line| {
			let c = line.as_bytes();
			((c[0], c[1]), ((c[0], c[6]),(c[6], c[1])))
		}).collect();

	let mut polymer: Polymer = HashMap::new();
	for pair in input
		.lines()
		.next()
		.unwrap()
		.as_bytes()
		.windows(2) {
			*polymer.entry((pair[0], pair[1])).or_insert(0) += 1;
		}
	let initial_byte: u8 = input.as_bytes()[0];

	for _step in 0..10 {
		let mut new_polymer: Polymer = HashMap::new();
		for (pair, count) in polymer.into_iter() {
			let (p1,p2) = reaction_pairs[&pair];
			*new_polymer.entry(p1).or_insert(0) += count;
			*new_polymer.entry(p2).or_insert(0) += count;
		}
		polymer = new_polymer;
	}
	let m1 = most_vs_least_common(&polymer, initial_byte);
	for _step in 10..40 {
		let mut new_polymer: Polymer = HashMap::new();
		for (pair, count) in polymer.into_iter() {
			let (p1,p2) = reaction_pairs[&pair];
			*new_polymer.entry(p1).or_insert(0) += count;
			*new_polymer.entry(p2).or_insert(0) += count;
		}
		polymer = new_polymer;
	}
	let m2 = most_vs_least_common(&polymer, initial_byte);


    Solution::new(m1,m2)
}


