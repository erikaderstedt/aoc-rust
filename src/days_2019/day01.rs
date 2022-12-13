use crate::common::{Solution,parsed_from_each_line};

fn fuel(mass: &usize) -> usize {
	let f = mass / 3 - 2;
	if f >= 6 {
		f + fuel(&f)
	} else {
		f
	}
}

pub fn solve(input: &str) -> Solution {
	let module_masses: Vec<usize> = parsed_from_each_line(input);

	let p1: usize = module_masses.iter().map(|m| m / 3 - 2).sum();
	let p2: usize = module_masses.iter().map(|m| fuel(m)).sum();

    Solution::new(p1,p2)
}
