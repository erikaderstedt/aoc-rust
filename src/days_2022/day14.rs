// https://adventofcode.com/2022/day/14
use crate::common::Solution;
use crate::grid::{Grid,GridElement, Position};

const X_OFFSET: usize = 300;
const HEIGHT: usize = 170;
const WIDTH: usize = 400;

fn introduce_sand(cave: &mut Grid<CaveItem>, at: Position) -> bool {
    let mut p = at;

    loop { 
        if p.row == HEIGHT - 1 {
            return false;
        }
        assert!(p.column < WIDTH - 1);
        assert!(p.column > 0);
        
        p.row += 1;
        if cave[&p] != CaveItem::Air {
            p.column -= 1;
            if cave[&p] != CaveItem::Air {
                p.column += 2;
                if cave[&p] != CaveItem::Air {
                    p.column -= 1; p.row -= 1;
                    cave[&p] = CaveItem::Sand;
                    return true;
                }
            }
        }
    }
}

fn add_sand_until_it_falls_off(cave: &mut Grid<CaveItem>) -> usize {
    let mut p1 = 0;
    let start = Position { column: 500 - X_OFFSET, row: 0 };
    while introduce_sand(cave, start.clone()) {
        p1 += 1;
    }
    p1
}

fn add_sand_until_full(cave: &mut Grid<CaveItem>) -> usize {
    let mut p2 = 0;
    let start = Position { column: 500 - X_OFFSET, row: 0 };
    while cave[&start] == CaveItem::Air && introduce_sand(cave, start.clone()) {
        p2 += 1;
    }
    p2
}

pub fn solve(input: &str) -> Solution {
    let mut cave = Grid { rows: HEIGHT, cols: WIDTH, locations: vec![CaveItem::Air; WIDTH * HEIGHT] };

    let mut bottom = 0;
    for line in input.lines() {
        let groups: Vec<Position> = line.split(" -> ").filter_map(|g| g
                .split_once(",")
                .map(|(s1,s2)| {
                    let n1 = s1.parse::<usize>().unwrap();
                    let n2 = s2.parse::<usize>().unwrap();
                    assert!(n1 >= X_OFFSET, "Column too far to the left");
                    assert!(n1 < X_OFFSET + WIDTH, "Too narrow");
                    assert!(n2 < HEIGHT, "Too low");
                    Position { column: n1 - X_OFFSET, row: n2 }
                }))    
                .collect();

        bottom = bottom.max(groups.iter().map(|g| g.row).max().unwrap());

        assert!(groups.len() >= 2, "Not enough groups");
        let mut i = 1;
        let mut current = groups[0].clone();
        let mut target = groups[1].clone();
        loop {
            // Draw current and move one step to target
            cave[&current] = CaveItem::Rock;
            if current.column > target.column { current.column -= 1; }
            else if current.column < target.column { current.column += 1; }

            if current.row > target.row { current.row -= 1; }
            else if current.row < target.row { current.row += 1; }

            if current == target {
                i += 1;
                if i < groups.len() {
                    target = groups[i].clone();
                } else {
                    break;
                }
            }
        }
        cave[&target] = CaveItem::Rock;
    }

    let p1 = add_sand_until_it_falls_off(&mut cave);

    for x in 0..WIDTH {
        let p = Position { column: x, row: bottom + 2 };
        cave[&p] = CaveItem::Rock;
    }

    let p2 = add_sand_until_full(&mut cave) + p1;

    Solution::new(p1,p2)
}

#[derive(Clone,PartialEq,Eq)]
enum CaveItem {
    Air,
    Rock,
    Sand,
}

impl GridElement for CaveItem {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(CaveItem::Rock),
            'o' => Some(CaveItem::Sand),
            '.' => Some(CaveItem::Air),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            CaveItem::Air => '.',
            CaveItem::Rock => '#',
            CaveItem::Sand => 'o',
        }
    }    
}