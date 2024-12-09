// https://adventofcode.com/2024/day/9

use crate::common::Solution;


pub fn solve(input: &str) -> Solution {
    
    let mut original: Vec<Option<usize>> = input.as_bytes().chunks(2).enumerate().map(|(i, v)| -> Vec<Option<usize>> {
        let length_of_file = (v[0]- ('0' as u8)) as usize;
        let length_of_free_space = if v.len() == 2 { (v[1] - ('0' as u8)) as usize } else { 0 };
        vec![Some(i);length_of_file].into_iter().chain(vec![None;length_of_free_space]).collect()
    }).flatten().collect();

    loop {
        if let Some(first_free) = original.iter().position(|p| p.is_none()) {
            assert!(original[first_free] == None);
            // println!("{:?}", original);
            // println!("{} {}", first_free, original.len());
            if let Some(i) = original.pop() {
                if first_free < original.len() {
                    original[first_free] = i;
                }
            }    
        } else {
            break
        }

    }

    

    // println!("{:?}", original);
    let p1 = original.into_iter().enumerate().map(|(i, v)| i * v.unwrap()).sum::<usize>();
    let p2 = 0;

    Solution::new(p1, p2)
}
