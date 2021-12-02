use crate::common::Solution;
use crate::common::parsed_from_each_line;
use itertools::Itertools;

// day 24 2015

pub fn solve(input: &str) -> Solution {
    let packages: Vec<usize> = parsed_from_each_line(input);
    let total = packages.iter().sum::<usize>();
    let p1 = {

        // All numbers are odd. Since we need an even weight (508), we need an even number of packages in each bag.
        // 28 packages.
        // 113+109 is too small. So is 113+109+103+107. This means at least 6 packages in each bag is required.
        // Best answer is 6 bags. We only need to find the best combination out of 376740 ways to select 6 packages
        //
        let each_bag = total/3;
        packages.iter().cloned()
            .combinations(6)
            .filter(|c| c.iter().sum::<usize>() == each_bag)
            .map(|c| c.iter().product::<usize>())
            .min()
            .unwrap()
    };
    let p2 = {
        // Three packages are not enough.
        // Four won't work either since the sum will be an even number. 
        // This means that 5 packages is the minimum needed.
        let each_bag = total/4;
        packages.iter().cloned()
            .combinations(5)
            .filter(|c| c.iter().sum::<usize>() == each_bag)
            .map(|c| c.iter().product::<usize>())
            .min()
            .unwrap()
    };
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}