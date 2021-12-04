// https://adventofcode.com/2021/day/4

use crate::common::Solution;

const SIDE: usize = 5;
const NUM_DIMENSIONS: usize = 2;
type Board = [u128;SIDE*NUM_DIMENSIONS];

fn sum_of_unmarked_numbers(board: &Board, drawn_numbers: &u128) -> usize {
    let mut u = (board[0] | board[1] | board[2] | board[3] | board[4] |
    board[5] | board[6] | board[7] | board[8] | board[9]) & (!drawn_numbers);
    
    let mut sum: usize = 0;
    for i in 0..100 {
        if (u & 1) == 1 { sum += i }
        u >>= 1;
    }
    sum
}

pub fn solve(input: &str) -> Solution {

    let bingo_numbers: Vec<usize> = input.lines().next().unwrap().split(',').map(|p| p.parse::<usize>().unwrap()).collect();
    let boards: Vec<Board> = input.split("\n\n").skip(1).map(|chunk| {
        let n: Vec<u128> = chunk.split_whitespace().filter_map(|p| p.parse::<u128>().ok()).collect();
        [(1<<n[0]) + (1<<n[1]) + (1<<n[2]) + (1<<n[3]) + (1<<n[4]),
        (1<<n[5]) + (1<<n[6]) + (1<<n[7]) + (1<<n[8]) + (1<<n[9]),
        (1<<n[10]) + (1<<n[11]) + (1<<n[12]) + (1<<n[13]) + (1<<n[14]),
        (1<<n[15]) + (1<<n[16]) + (1<<n[17]) + (1<<n[18]) + (1<<n[19]),
        (1<<n[20]) + (1<<n[21]) + (1<<n[22]) + (1<<n[23]) + (1<<n[24]),
        (1<<n[0]) + (1<<n[5]) + (1<<n[10]) + (1<<n[15]) + (1<<n[20]),
        (1<<n[1]) + (1<<n[6]) + (1<<n[11]) + (1<<n[16]) + (1<<n[21]),
        (1<<n[2]) + (1<<n[7]) + (1<<n[12]) + (1<<n[17]) + (1<<n[22]),
        (1<<n[3]) + (1<<n[8]) + (1<<n[13]) + (1<<n[18]) + (1<<n[23]),
        (1<<n[4]) + (1<<n[9]) + (1<<n[14]) + (1<<n[19]) + (1<<n[24])]
    }).collect();

    let m1 = bingo_numbers.iter().scan(0u128, |drawn_numbers, &num| {
        *drawn_numbers |= 1 << num;
        Some(match boards.iter().find(|board| board.iter().any(|w| w & *drawn_numbers == *w)) {
            Some(board) => Some(sum_of_unmarked_numbers(board, drawn_numbers) * num),
            _ => None,
        })
    }).skip_while(|w| w.is_none() ).next().unwrap().unwrap();
      
    let m2 = bingo_numbers.iter().scan(0u128, |drawn_numbers, &num| {
        *drawn_numbers |= 1 << num;
        Some(if boards.iter().all(|board| board.iter().any(|w| w & *drawn_numbers == *w)) {
            // All boards win. Remove this number and check which board does not win.
            *drawn_numbers -= 1 << num;
            boards.iter().find_map(|board| 
                if !board.iter().any(|w| w & *drawn_numbers == *w) { 
                    Some(sum_of_unmarked_numbers(board, &(*drawn_numbers | (1 << num))) * num)
                } else { 
                    None 
                })
            } else { None })
    }).skip_while(|w| w.is_none() ).next().unwrap().unwrap();

    Solution::new(m1, m2)
}