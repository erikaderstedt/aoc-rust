// https://adventofcode.com/2021/day/14
use crate::common::Solution;

const NUM_ELEMENTS: usize = 10;
const NUM_PAIRS: usize = NUM_ELEMENTS * NUM_ELEMENTS;
fn element_index(element: u8) -> usize {
	match element {
		b'B' => 0,
		b'C' => 1,
		b'F' => 2,
		b'H' => 3,
		b'K' => 4,
		b'N' => 5,
		b'O' => 6,
		b'P' => 7,
		b'S' => 8,
		b'V' => 9,
		_ => panic!("Unrecognized element"),
	}
}

type Polymer = [usize;NUM_PAIRS];
type ReactionTable = [(usize,usize);NUM_PAIRS];

fn most_vs_least_common(polymer: &Polymer, initial_element: usize) -> usize {
	let mut counts = [0usize; NUM_ELEMENTS];
	counts[initial_element] = 1;
	for (pair, count) in polymer.iter().enumerate() {
		counts[pair % NUM_ELEMENTS] += count;
	}
	let most_common: usize = *counts.iter().max().unwrap();
	let least_common: usize = *counts.iter().filter(|&&c| c > 0).min().unwrap();
	most_common - least_common
}

fn update(polymer: Polymer, reaction_pairs: &ReactionTable) -> Polymer {
	let mut new_polymer: Polymer = [0usize;NUM_PAIRS];
	for (pair, count) in polymer.iter().enumerate() {
		if *count > 0 {
			let r = reaction_pairs[pair];
			new_polymer[r.0] += count;
			new_polymer[r.1] += count;
		}
	}
	new_polymer
}

pub fn solve(input: &str) -> Solution {
	let mut reaction_table: ReactionTable = [(0usize,0usize);NUM_PAIRS];
	for line in input.lines().skip(2) {
		let p1 = element_index(line.as_bytes()[0]);
		let p2 = element_index(line.as_bytes()[1]);
		let r = element_index(line.as_bytes()[6]);
		reaction_table[p1 * NUM_ELEMENTS + p2] = 
			(p1 * NUM_ELEMENTS + r, r * NUM_ELEMENTS + p2);
	}

	let mut polymer = [0usize;NUM_PAIRS];
	for pair in input.lines().next().unwrap().as_bytes().windows(2) {
		polymer[element_index(pair[0]) * NUM_ELEMENTS + element_index(pair[1])] += 1;
	}

	let initial_byte: usize = element_index(input.as_bytes()[0]);

	for _step in 0..10 { polymer = update(polymer, &reaction_table); }
	let m1 = most_vs_least_common(&polymer, initial_byte);
	for _step in 10..40 { polymer = update(polymer, &reaction_table); }
	let m2 = most_vs_least_common(&polymer, initial_byte);

    Solution::new(m1,m2)
}
