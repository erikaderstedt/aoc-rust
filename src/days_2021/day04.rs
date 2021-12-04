// https://adventofcode.com/2021/day/4

use crate::common::Solution;
use std::convert::TryInto;

const SIDE: usize = 5;
const BOARD_SIZE: usize = SIDE*SIDE;

struct Board {
    numbers: [Option<usize>;BOARD_SIZE],
}

impl Board {
    // Returns true if we just got bingo
    fn mark(&mut self, number: &usize) -> bool {
        for i in 0..BOARD_SIZE { 
            match self.numbers[i] {
                Some(n) if n == *number => self.numbers[i] = None,
                _ => {},
            }
        }
        let horizontal = self.numbers[..]
            .chunks(SIDE)
            .any(|row| !row.iter().any(|v| v.is_some()) );
        let vertical = (0..5)
            .any(|c| !self.numbers[..].iter()
                            .skip(c)
                            .step_by(SIDE)
                            .any(|v| v.is_some()));
        return horizontal || vertical
    }

    fn sum_of_unmarked_numbers(&self) -> usize {
        self.numbers.iter().filter_map(|n| *n).sum()
    }  
}

pub fn solve(input: &str) -> Solution {

    let bingo_numbers: Vec<usize> = input.lines().next().unwrap().split(',').map(|p| p.parse::<usize>().unwrap()).collect();
    let mut boards: Vec<Board> = input.split("\n\n").skip(1).map(|chunk| {
        let numbers = chunk.split_whitespace().map(|p| p.parse::<usize>().ok()).collect::<Vec<Option<usize>>>();
        Board { numbers: numbers.try_into().unwrap() }
    }).collect();

    let mut m1: Option<usize> = None;
    let mut m2 = 0;
    for num in bingo_numbers.iter() {
        let mut i = 0;
        while i < boards.len() {
            if boards[i].mark(num) { 
                if m1 == None { m1 = Some(boards[i].sum_of_unmarked_numbers() * num) }
                if boards.len() == 1 { m2 = boards[i].sum_of_unmarked_numbers() * num }
                boards.remove(i);
            } else {
                i += 1;
            }
        }
    }
    Solution::new(m1.unwrap(), m2)
}