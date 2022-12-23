// https://adventofcode.com/2022/day/23

use std::collections::HashMap;
use crate::common::Solution;

const EMPTY: u8 = '.' as u8;
const ELF: u8 = '#' as u8;

#[derive(Debug,Clone,PartialEq,Eq)]
enum Proposal {
    North,
    South,
    West,
    East,
}

const WIDTH: usize = 256;
const OFFSET: usize = (WIDTH - 73) / 2;

pub fn solve(input: &str) -> Solution {
    let mut grid = [EMPTY; WIDTH * WIDTH];
    let mut elves: Vec<usize> = Vec::with_capacity(10000);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            if *c == ELF { 
                let index = (y + OFFSET) * WIDTH + (x + OFFSET);
                grid[index] = ELF;
                elves.push(index);
            }
        }
    }

    let mut proposal_directions = vec![Proposal::North, Proposal::South, Proposal::West, Proposal::East];
    let mut iteration = 0;
    let mut p1 = 0;
    loop {    
        // New move locations for each elf
        let mut locations: HashMap<usize,usize> = HashMap::new();

        for (index, elf) in elves.iter().enumerate() {
            if grid[elf-WIDTH] == EMPTY && grid[elf-WIDTH-1] == EMPTY && grid[elf-WIDTH+1] == EMPTY &&
                grid[elf-1] == EMPTY && grid[elf+1] == EMPTY &&
                grid[elf+WIDTH] == EMPTY && grid[elf+WIDTH-1] == EMPTY && grid[elf+WIDTH+1] == EMPTY {
                    continue;
                }
    
            match proposal_directions.iter().find(|dir|
                match dir {
                    Proposal::North => grid[elf-WIDTH] == EMPTY && grid[elf-WIDTH-1] == EMPTY && grid[elf-WIDTH+1] == EMPTY,
                    Proposal::South => grid[elf+WIDTH] == EMPTY && grid[elf+WIDTH-1] == EMPTY && grid[elf+WIDTH+1] == EMPTY,
                    Proposal::West => grid[elf-WIDTH-1] == EMPTY && grid[elf-1] == EMPTY && grid[elf+WIDTH-1] == EMPTY,
                    Proposal::East => grid[elf-WIDTH+1] == EMPTY && grid[elf+1] == EMPTY && grid[elf+WIDTH+1] == EMPTY,
                }) {
                    Some(direction) => {
                        let proposed_position = match direction {
                            Proposal::East => elf + 1,
                            Proposal::North => elf - WIDTH,
                            Proposal::South => elf + WIDTH,
                            Proposal::West => elf - 1,
                        };
                        if locations.contains_key(&proposed_position) {
                            locations.remove(&proposed_position);                    
                        } else {
                            locations.insert(proposed_position, index);
                        }
                    },
                    None => {},
            }
        }

        if locations.len() == 0 {
            break;
        }
        for (p, elf_index) in locations.into_iter() {
            grid[elves[elf_index]] = EMPTY;
            elves[elf_index] = p;
            grid[elves[elf_index]] = ELF;
        }
        let p = proposal_directions.remove(0);
        proposal_directions.push(p);

        iteration += 1;
        if iteration == 10 {
            let min_x = elves.iter().map(|e| e % WIDTH).min().unwrap();
            let max_x = elves.iter().map(|e| e % WIDTH).max().unwrap();
            let min_y = elves.iter().map(|e| e / WIDTH).min().unwrap();
            let max_y = elves.iter().map(|e| e / WIDTH).max().unwrap();
            let area = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize;
            let num_elves = elves.len();
            p1 = area - num_elves;
        }
    }
        
    Solution::new(p1,iteration + 1)
}
