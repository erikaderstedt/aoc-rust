// https://adventofcode.com/2024/day/21

use std::collections::HashMap;
use itertools::Itertools;
use crate::common::Solution;

const NUMERIC: [[u8;3];4] = 
[   [ b'7',b'8',b'9' ],
    [ b'4',b'5',b'6' ],
    [ b'1',b'2',b'3' ],
    [ b' ',b'0',b'A' ],
];

const DIRECTIONAL: [[u8;3];2] =
[   [ b' ',b'^',b'A' ],
    [ b'<',b'v',b'>' ]
];

fn first_directional_commands(numeric_keys: &[u8]) -> Vec<u8> {
    let mut numeric_lookup_table = [(0usize,0usize);256];
    for column in 0..3 {
        for row in 0..4 {
            numeric_lookup_table[NUMERIC[row][column] as usize] = (row, column);
        }
    }
    
    let mut result = vec![b'A'];
    let (mut row, mut column) = numeric_lookup_table[b'A' as usize];
    
    for k in numeric_keys.iter() {
        let (move_to_row, move_to_column) = numeric_lookup_table[*k as usize];
        if row < 3 || (row == 3 && move_to_column > 0) {
            while move_to_column < column { result.push(b'<'); column -= 1; }
        }
        if column > 0 || (column == 0 && move_to_row < 3) {
            while move_to_row > row { result.push(b'v'); row += 1; }
        }
        while move_to_row < row { result.push(b'^'); row -= 1; }
        while move_to_column < column { result.push(b'<'); column -= 1; }
        while move_to_column > column { result.push(b'>'); column += 1; }
        while move_to_row > row { result.push(b'v'); row += 1; }
        result.push(b'A');
    }
    result
}

fn expand_pair(c1: &u8, c2: &u8, depth: u8, cache: &mut HashMap<(u8,u8,u8),usize>, lookup: &[(usize,usize);256]) -> usize {

    let k = (c1.clone(), c2.clone(), depth.clone());

    if let Some(r) = cache.get(&k) {
        return r.clone();
    }

    let mut result = vec![b'A'];
    let (mut row, mut column) = lookup[*c1 as usize];
    let (move_to_row, move_to_column) = lookup[*c2 as usize];
    
    if row == 1 || move_to_column > 0 {
        while move_to_column < column { result.push(b'<'); column -= 1; }
    }
    if column > 0 { 
        while move_to_row < row { result.push(b'^'); row -= 1; }
    }
    while move_to_row > row { result.push(b'v'); row += 1; }
    while move_to_column < column { result.push(b'<'); column -= 1; }
    while move_to_column > column { result.push(b'>'); column += 1; }
    while move_to_row < row { result.push(b'^'); row -= 1; }
    result.push(b'A');

    let total = if depth == 1 { 
        result.len() - 1
    } else {
        result
        .iter()
        .tuple_windows()
        .map(|(c1, c2)| expand_pair(c1, c2, depth - 1, cache, lookup))
        .sum::<usize>()
    };

    cache.insert(k, total.clone());
    total
}

fn calculate_movement_length<const NROBOTS: u8>(keycodes: &Vec<&str>) -> usize {
    let mut lookup = [(0usize,0usize);256];
    for column in 0..3 {
        for row in 0..2 {
            lookup[DIRECTIONAL[row][column] as usize] = (row, column);
        }
    }

    let mut cache: HashMap<(u8,u8,u8),usize> = HashMap::new();

    keycodes
    .iter()
    .map(|keycode| {
        let length = first_directional_commands(&keycode.as_bytes())
        .iter()
        .tuple_windows()
        .map(|(c1, c2)| expand_pair(c1, c2, NROBOTS, &mut cache, &lookup))
        .sum::<usize>();

        let complexity = std::str::from_utf8(&keycode.as_bytes()[0..3])
        .unwrap()
        .parse::<usize>()
        .unwrap();
        
        complexity * length
    })
    .sum::<usize>()
}

pub fn solve(input: &str) -> Solution {
    let keycodes: Vec<&str> = input.lines().collect();

    let p1 = calculate_movement_length::<2>(&keycodes);
    let p2 = calculate_movement_length::<25>(&keycodes);

    Solution::new(p1, p2)
}
