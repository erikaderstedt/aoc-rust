use crate::common::Solution;

// This code is *much* faster when using u32 instead of usize. Probably due to cache size. 
fn determine_nth_number_spoken(starting_nums: &[u32], n: usize) -> usize {
    let mut spoken = vec![0u32; n as usize];
    for i in 0..(starting_nums.len()-1) { spoken[starting_nums[i] as usize] = (i+1) as u32; }

    (starting_nums.len()..n).fold(starting_nums[starting_nums.len()-1] as usize, |last_spoken, turn_number| {
        let speak = match spoken[last_spoken] { 0 => 0, n => turn_number - (n as usize) };
        spoken[last_spoken] = turn_number as u32;
        speak
    })
}

pub fn solve(_input: &str) -> Solution {
    let nums: [u32;6] = [1,0,15,2,10,13];

    let p1 = determine_nth_number_spoken(&nums, 2020usize);
    let p2 = determine_nth_number_spoken(&nums, 30000000usize);

    Solution::new(p1,p2)
}