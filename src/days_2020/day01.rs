use crate::common::Solution;
use crate::common::parsed_from_each_line;

pub fn solve(input: &str) -> Solution {
    let expenses: Vec<usize> = parsed_from_each_line(input);
    let mut occuring_numbers = [false;2020];
    for expense in expenses.into_iter() { occuring_numbers[expense] = true; }

    // Part 1: Find two numbers that add up to 2020
    let m1 = (0..1010).find(|i| occuring_numbers[*i] && occuring_numbers[2020 - *i]).unwrap();    

    // Part 2: Find three numbers that add up to 2020
    let m2 = (0..1010).find_map(|i| -> Option<[usize;3]> {
        if !occuring_numbers[i] { None } else {
            match ((i + 1)..(2020 - i)).find(|j| occuring_numbers[*j] && occuring_numbers[2020 - *j - i]) {
                None => None,
                Some(x) => Some([i, x, 2020 - x - i]),
            }
        }
    }).unwrap();

    Solution::new(m1 * (2020 - m1), m2[0] * m2[1] * m2[2])
}