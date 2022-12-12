use crate::common::Solution;

fn digits(num: &u32) -> [u32;6] {
    [
        (num/100000)%10,
        (num/10000)%10,
        (num/1000)%10,
        (num/100)%10,
        (num/10)%10,
        num%10,
    ]
}

pub fn solve(_input: &str) -> Solution {
    let num_matches = (165432..707912).filter(|n| {
        let d = digits(n);
        let two_adjacent_are_the_same = (1..6).any(|i| d[i-1] == d[i]);
        let not_smaller = (1..6).all(|i| d[i-1] <= d[i]);
        two_adjacent_are_the_same && not_smaller
    }).count();

    let num_matches_rejecting_longer_groups = (165432..707912).filter(|n| {
        let d = digits(n);
        let not_smaller = (1..6).all(|i| d[i-1] <= d[i]);
        let two_adjacent_are_the_same = (1..6).any(|i| {
            d[i-1] == d[i] && 
            ((i == 5) || d[i+1] != d[i]) && 
            ((i == 1) || d[i-2] != d[i])
        });
        // 
        two_adjacent_are_the_same && not_smaller
    }).count();

    Solution::new(num_matches, num_matches_rejecting_longer_groups)
}