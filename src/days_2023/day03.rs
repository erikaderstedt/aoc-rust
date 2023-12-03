// https://adventofcode.com/2023/day/3

use std::collections::HashSet;

use crate::{common::Solution, grid::{Grid, GridElement, Position}};

#[derive(PartialEq, Eq, Clone,Debug)]
enum EngineSchematicPoint {
    Digit(u8),
    Symbol(char),
    Number(usize),
    Blank,
}

impl EngineSchematicPoint {
    fn is_symbol(&self) -> bool {
        match self {
            Self::Symbol(_) => true,
            _ => false,
        }
    }

    fn encode_part_number(position: &Position, part_number_value: usize) -> Self {
        Self::Number(position.row + position.column * 1000 + part_number_value * 1000000)
    }

    fn decode_part_number(encoded_number: usize) -> usize {
        encoded_number / 1000000
    }

}

impl GridElement for EngineSchematicPoint {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '\n' => None,
            '.' => Some(Self::Blank),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(Self::Digit(*c as u8 - '0' as u8)),
            _ => Some(Self::Symbol(*c)),
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Blank => '.',
            Self::Symbol(a) => *a,
            Self::Number(x) => 'X',
            Self::Digit(d) => (d + '0' as u8) as char,
        }
    }
}

pub fn solve(input: &str) -> Solution {    
    let mut engine: Grid<EngineSchematicPoint> = Grid::load(input);

    engine.enclose(EngineSchematicPoint::Blank);
    
    let p1: usize = engine.positions().filter_map(|pos|
        match engine[&pos] {
            EngineSchematicPoint::Digit(d) if pos.neighbors().any(|p| engine[&p].is_symbol()) => {
            // Scan to the left and right to find other digits.
            let mut value = d as usize;
            let mut scan_pos = pos.left();
            let mut multiplier = 10;
            let mut positions = vec![pos.clone()];
            loop {
                match engine[&scan_pos] {
                    EngineSchematicPoint::Digit(d) => {
                        value = multiplier * (d as usize) + value;
                        multiplier = 10 * multiplier;
                        engine[&scan_pos] = EngineSchematicPoint::Blank;
                        positions.push(scan_pos.clone());
                        scan_pos = scan_pos.left();
                    },
                    _ => { break },
                }
            };
            scan_pos = pos.right();
            loop {
                match engine[&scan_pos] {
                    EngineSchematicPoint::Digit(d) => {
                        value = 10 * value + (d as usize);
                        engine[&scan_pos] = EngineSchematicPoint::Blank;
                        positions.push(scan_pos.clone());
                        scan_pos = scan_pos.right();
                    },
                    _ => { break },
                }
            };
            for p in positions.into_iter() {
                engine[&p] = EngineSchematicPoint::encode_part_number(&pos, value);
            }
            Some(value)
            },
            _ => None,
        }
    ).sum();

    let p2: usize = engine.positions().filter_map(|pos|
        match engine[&pos] {
            EngineSchematicPoint::Symbol('*') => {

                let numbers: HashSet<usize> = pos.neighbors().filter_map(|n| match engine[&n] {
                    EngineSchematicPoint::Number(x) => Some(x),
                    _ => None,
                }).collect();
                
                if numbers.len() == 2 {                    
                    Some(numbers.iter().map(|n| EngineSchematicPoint::decode_part_number(*n)).product::<usize>())
                } else {
                    None
                }
            },
            _ => None,
        }
    ).sum();

    Solution::new(p1,p2)
}
