// https://adventofcode.com/2023/day/3

use itertools::Itertools;

use crate::{common::Solution, grid::{Grid, GridElement, Position}};

#[derive(PartialEq, Eq, Clone)]
enum EngineSchematicPoint {
    Digit(u8),
    Gear,
    OtherSymbol,
    Blank,
}

impl EngineSchematicPoint {
    fn any_symbol(&self) -> bool {
        match self {
            Self::Gear | Self::OtherSymbol => true,
            _ => false,
        }
    }
}

impl GridElement for EngineSchematicPoint {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '\n' => None,
            '.' => Some(Self::Blank),
            '*' => Some(Self::Gear),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(Self::Digit(*c as u8 - '0' as u8)),
            _ => Some(Self::OtherSymbol),
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Blank => '.',
            Self::OtherSymbol => '?',
            Self::Gear => '*',
            Self::Digit(d) => (d + '0' as u8) as char,
        }
    }
}

struct Number {
    value: usize,
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

pub fn solve(input: &str) -> Solution {    
    let mut engine: Grid<EngineSchematicPoint> = Grid::load(input);

    engine.enclose(EngineSchematicPoint::Blank);

    let contiguous_regions = engine
        .positions()
        .group_by(|e| match engine[e] { EngineSchematicPoint::Digit(_) => true, _ => false });

    let mut numbers: Vec<Number> = Vec::new();
    for (is_digits, group) in &contiguous_regions {
        if is_digits {
            let mut value = 0;
            let mut multiplier = 1;
            let positions: Vec<Position> = group.collect();
            for position in positions.iter().rev() {
                match engine[position] {
                    EngineSchematicPoint::Digit(n) => {
                        value += multiplier * (n as usize);
                        multiplier *= 10;
                    },
                    _ => {},
                }
            }
            let left = positions[0].column - 1;
            let right = left + positions.len() + 1;
            let top = positions[0].row - 1;
            let bottom = top + 2;
            numbers.push(Number { value, left, right, top, bottom })
        }
    }

    let p1: usize = numbers.iter()
        .filter(|n| (n.left..=n.right).any(|x| 
                        engine[&Position { row: n.top, column: x }].any_symbol() ||
                        engine[&Position { row: n.bottom, column: x }].any_symbol()) ||
            engine[&Position { row: n.top + 1, column: n.left }].any_symbol() ||
            engine[&Position { row: n.top + 1, column: n.right }].any_symbol())
        .map(|n| n.value)
        .sum();

    let p2: usize = engine.positions()
        .filter_map(|p| {
            match engine[&p] {
                EngineSchematicPoint::Gear => { 
                    let values: Vec<usize> = numbers.iter()
                    .filter(|n| n.left <= p.column && n.right >= p.column && n.top <= p.row && n.bottom >= p.row)
                    .map(|n| n.value).collect();
    
                    if values.len() == 2 {
                        Some(values[0] * values[1])
                    } else {
                        None
                    }
                },
                _ => None,
            }
        })
        .sum();

    Solution::new(p1,p2)
}
