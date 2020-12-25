use crate::common::Solution;

const VALUE: usize = 20_201_227;

fn find_loop_size(key: usize, subject_number: usize) -> usize {
    let mut i = 1;
    let mut n = 0;
    loop {
        i = i*subject_number % VALUE;
        n += 1;
        if i == key { break n; }
    }
}

fn transform_key(key: usize, loop_size: usize) -> usize {
    let mut i = key;
    for _n in 0..(loop_size-1) {
        i = i * key % VALUE;        
    }
    i 
}

const CARD_PUBLIC_KEY: usize = 12092626;
const DOOR_PUBLIC_KEY: usize = 4707356;
pub fn solve(_input: &str) -> Solution {

    let card_loop_size = find_loop_size(CARD_PUBLIC_KEY, 7);
    let p1 = transform_key(DOOR_PUBLIC_KEY, card_loop_size);
    
    Solution { part_1: p1.to_string(), part_2: "".to_string() }
}