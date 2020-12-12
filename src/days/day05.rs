use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    let mut seat_ids: Vec<i64> = input.lines()
        .map(|s| s.chars().fold(0, |total,c| total * 2 + if c == 'B' || c == 'R' { 1 } else { 0 }))
        .collect();

    seat_ids.sort_by(|a, b| b.cmp(a)); // Reverse

    let p1 = seat_ids[0];
    let p2 = seat_ids.into_iter().enumerate().find(|(index,seat_id)| p1-(*index as i64) != *seat_id).unwrap().1 + 1;

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}