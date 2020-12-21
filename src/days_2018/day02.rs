use crate::common::Solution;
use itertools::Itertools;

fn has_n_identical_letters(s: &str, n: u8) -> bool {
    let mut counts = [0u8;26];
    for letter in s.as_bytes().iter() { counts[(letter - ('a' as u8)) as usize] += 1; }
    counts.iter().any(|&c| c == n)
}

fn exactly_one_difference_between(a:&str, b:&str) -> bool {
    a.chars().zip(b.chars()).filter(|(c1,c2)| c1 != c2).count() == 1
}

pub fn solve(input: &str) -> Solution {
    let box_ids: Vec<&str> = input.lines().collect();
    let p1 = box_ids.iter().filter(|&b| has_n_identical_letters(b, 2)).count() *
            box_ids.iter().filter(|&b| has_n_identical_letters(b, 3)).count();

    let two_correct_boxes: (&str,&str) = box_ids.iter()
        .combinations(2)
        .filter_map(|v: Vec<&&str>| -> Option<(&str,&str)> {
            let a: &str = v[0]; let b: &str = v[1];
            if exactly_one_difference_between(a, b) { Some((a,b)) } else { None }
        })
        .next().unwrap();

    let p2: String = two_correct_boxes.0.chars().zip(two_correct_boxes.1.chars())
        .filter_map(|(c1,c2)| if c1 == c2 { Some(c1.clone()) } else { None }).collect();

    Solution::new(p1,p2)
}
