// https://adventofcode.com/2022/day/22

use crate::common::Solution;
use crate::grid::{Grid, GridElement,Position};

#[derive(Clone,Debug,PartialEq, Eq)]
enum Space {
    Edge,
    Open,
    Wall,
}

impl GridElement for Space {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            ' ' => Some(Self::Edge),
            '.' => Some(Self::Open),
            '#' => Some(Self::Wall),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Edge => ' ',
            Self::Open => '.',
            Self::Wall => '#',
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    distance: isize,
    turn: Turn,
}

impl Move {
    fn parse(input: &str) -> Vec<Move> {
        input
            .split_terminator(|c| c == 'L' || c == 'R')
            .zip(input
                .split_terminator(|c: char| -> bool { c.is_ascii_digit() })
                .filter(|s| s.len() > 0)
                .cycle())
            .map(|(d, t)| {
                let distance = d.parse::<isize>().unwrap();
                let turn = match t {
                    "L" => Turn::Left,
                    "R" => Turn::Right,
                    _ => panic!(""),
                };
                Move { distance, turn }
            })
            .collect()
    }
}

#[derive(Clone,Debug,PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn apply(&self, turn: &Turn) -> Self {
        match self {
            Self::Left => match turn { Turn::Left => Self::Down, Turn::Right => Self::Up },
            Self::Down => match turn { Turn::Left => Self::Right, Turn::Right => Self::Left },
            Self::Right => match turn { Turn::Left => Self::Up, Turn::Right => Self::Down },
            Self::Up => match turn { Turn::Left => Self::Left, Turn::Right => Self::Right },
        }
    }
    fn value(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }
}

fn next_position(grid: &Grid<Space>, direction: Direction, position: Position) -> (Position, Direction) {
    (match direction {
        Direction::Left => Position { row: position.row, column: (0..grid.cols).rev().skip_while(|c| grid.locations[position.row*grid.cols + c] == Space::Edge).next().unwrap() },
        Direction::Right => Position { row: position.row, column: (0..grid.cols).skip_while(|c| grid.locations[position.row*grid.cols + c] == Space::Edge).next().unwrap() },
        Direction::Down => Position { row: (0..grid.rows).skip_while(|r| grid.locations[r*grid.cols + position.column] == Space::Edge).next().unwrap(), column: position.column },
        Direction::Up => Position { row: (0..grid.rows).rev().skip_while(|r| grid.locations[r*grid.cols + position.column] == Space::Edge).next().unwrap(), column: position.column },
    }, direction)
}

fn trace_route<I>(grid: &Grid<Space>, moves: &Vec<Move>, wrapper: I) -> usize 
    where I: Fn(&Position, &Direction) -> (Position, Direction) {
    let first_available_spot = (0..grid.cols).find(|c| grid.locations[grid.cols + *c] == Space::Open).unwrap();
    let mut position = Position { row: 1, column: first_available_spot};
    let mut direction = Direction::Right;

    for m in moves.iter() {

        for i in 0..m.distance {
            let mut new_position = match direction {
                Direction::Left => Position { row: position.row, column: position.column - 1 },
                Direction::Down => Position { row: position.row + 1, column: position.column },
                Direction::Right => Position { row: position.row, column: position.column + 1 },
                Direction::Up => Position { row: position.row - 1, column: position.column },
            };
            match grid[&new_position] {
                Space::Open => { position = new_position; },
                Space::Wall => { break; },
                Space::Edge => {
                    let (new_position, new_direction) = wrapper(&position, &direction);                    
                    if grid[&new_position] == Space::Open {
                        position = new_position;
                        direction = new_direction;
                    }
                }
            }           
        }
        direction = direction.apply(&m.turn);
    }

    // Last turn is bogus. Undo it.
    for _ in 0..3 { direction = direction.apply(&moves[0].turn); }

    position.row * 1000 + position.column * 4 + direction.value()
}

pub fn solve(input: &str) -> Solution {
    let (grid_text, moves): (&str,&str) = input.split_once("\n\n").unwrap();

    let grid = {
        let cols = grid_text.lines().map(|c| c.len()).max().unwrap();
        let rows = grid_text.lines().count();
        let mut locations = vec![Space::Edge; cols*rows];
        for (row, line) in grid_text.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                locations[row*cols + col] = Space::from_char(&c).unwrap();
            }
        }

        let mut grid = Grid { rows, cols, locations };
        grid.enclose(Space::Edge);
        grid };

    let moves = Move::parse(moves.trim());

    let p1 = trace_route(&grid, &moves, |position, direction| next_position(&grid, direction.clone(), position.clone()));
    let p2 = trace_route(&grid, &moves, |position, direction| 
        match direction {
            // h, f are inverted
            Direction::Left => match position.row {
                1..=50 => next_position(&grid, Direction::Right, Position { row: 151-position.row, column: 0}),   // h . 1 -> 150, 50 -> 101 
                51..=100 => next_position(&grid, Direction::Down, Position { row: 0, column: position.row - 50 }),  // g
                101..=150 => next_position(&grid, Direction::Right, Position { row: 151-position.row, column: 0}), // h 150 -> 1, 101 -> 50
                151..=200 => next_position(&grid, Direction::Down, Position { row: 0, column: position.row - 100, }), // k
                _ => panic!("Out of bounds when wrapping left!"), },
            Direction::Down => match position.column {
                1..=50 => next_position(&grid, Direction::Down, Position { row: 0, column: position.column + 100}), // m
                51..=100 => next_position(&grid, Direction::Left, Position { row: position.column + 100, column: 0}), // o
                101..=150 => next_position(&grid, Direction::Left, Position { row: position.column - 50, column: 0 }), // e
                _ => panic!("Out of bounds when wrapping down!"), },
            Direction::Right => match position.row {
                1..=50 => next_position(&grid, Direction::Left, Position { row: 151-position.row, column: 0}), // f
                51..=100 => next_position(&grid, Direction::Up, Position { row: 0, column: position.row + 50 }), // e
                101..=150 => next_position(&grid, Direction::Left, Position { row: 151-position.row, column: 0}), // f
                151..=200 => next_position(&grid, Direction::Up, Position { row: 0, column: position.row - 100 }), // o
                _ => panic!("Out of bounds when wrapping right!"), },
            Direction::Up => match position.column {
                1..=50 => next_position(&grid, Direction::Right, Position { row: position.column + 50, column: 0}), // g
                51..=100 => next_position(&grid, Direction::Right, Position { row: position.column + 100, column: 0}), // k // 
                101..=150 => next_position(&grid, Direction::Up, Position { row: 0, column: position.column - 100 }), // m
                _ => panic!("Out of bounds when wrapping down!"), },
        });
    
    Solution::new(p1,p2)
}
