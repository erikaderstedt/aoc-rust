// https://adventofcode.com/2023/day/10

use crate::{common::Solution, grid::{Grid, GridElement, Direction, Position}};

fn get_loop_and_starting_piece(start: &Position, direction: &Direction, maze: &Grid<PipeMazePart>) -> Option<(Vec<Position>, PipeMazePart)> {
    let mut d: Direction = direction.clone();

    let mut position = start.along(&d);
    let mut the_loop: Vec<Position> = Vec::new();

    the_loop.push(position.clone());

    while position != *start {
        match maze[&position].follow(&d) {
            Some(new_direction) => { d = new_direction; },
            None => { return None }
        }

        position = position.along(&d);
        the_loop.push(position.clone());
    }

    let starting_part = PipeMazePart::matching(&d.reverse(), direction).unwrap_or(PipeMazePart::Ground);
    
    Some((the_loop, starting_part))
}

fn count_inside(the_loop: &Vec<Position>, maze: &Grid<PipeMazePart>) -> usize {
    let mut on_loop = vec![false; maze.rows * maze.cols];

    // Cache this in a grid-sized vec instead of searching the loop vec each time.
    for position in the_loop.iter() {
        on_loop[position.row * maze.cols + position.column] = true;
    }

    maze.locations.iter()
        .enumerate()
        .fold((0, false), |(num_inside, inside), (index, part)|
            if on_loop[index] {
                match part {
                    PipeMazePart::VerticalPipe | PipeMazePart::NorthEastBend | PipeMazePart::NorthWestBend => (num_inside, !inside),
                    _ => (num_inside, inside),
                }
            } else {
                (num_inside + if inside { 1 } else { 0 }, inside)
            }
        ).0
}

pub fn solve(input: &str) -> Solution {
    let mut maze: Grid<PipeMazePart> = Grid::load(input);

    let start = maze.find(&PipeMazePart::StartingPosition).unwrap();
    
    let (the_loop, missing_piece) = vec![Direction::North, Direction::West, Direction::South]
        .into_iter()
        .find_map(|d| get_loop_and_starting_piece(&start, &d, &maze)).unwrap();

    maze[&start] = missing_piece;

    let p1 = (the_loop.len() + 1)/2;
    let p2 = count_inside(&the_loop, &maze);

    Solution::new(p1,p2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PipeMazePart {
    VerticalPipe,
    HorizontalPipe,
    NorthWestBend,
    NorthEastBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    StartingPosition,
}

impl PipeMazePart {
    fn directions(&self) -> Option<(Direction, Direction)> {
        match self {
            Self::VerticalPipe => Some((Direction::North, Direction::South)),
            Self::HorizontalPipe => Some((Direction::West, Direction::East)),
            Self::NorthEastBend => Some((Direction::East, Direction::North)),
            Self::NorthWestBend => Some((Direction::North, Direction::West)),
            Self::SouthEastBend => Some((Direction::South, Direction::East)),
            Self::SouthWestBend => Some((Direction::West, Direction::South)),
            _ => None,
        }
    }

    fn matching(direction1: &Direction, direction2: &Direction) -> Option<Self> {
        match (direction1, direction2) {
            (Direction::West, Direction::East) | (Direction::East, Direction::West) => Some(Self::HorizontalPipe),
            (Direction::West, Direction::North) | (Direction::North, Direction::West) => Some(Self::NorthWestBend),
            (Direction::East, Direction::North) | (Direction::North, Direction::East) => Some(Self::NorthEastBend),
            (Direction::West, Direction::South) | (Direction::South, Direction::West) => Some(Self::SouthWestBend),
            (Direction::East, Direction::South) | (Direction::South, Direction::East) => Some(Self::SouthEastBend),
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => Some(Self::VerticalPipe),
            _ => None,
        }        
    }

    fn follow(&self, direction: &Direction) -> Option<Direction> {
        match self.directions() {
            Some((d1,d2)) => {
                if direction.reverse() == d1 {
                    Some(d2)
                } else if direction.reverse() == d2 {
                    Some(d1)
                } else {
                    None
                }
            },
            None => None,
        }
        
    }
}

impl GridElement for PipeMazePart {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '|' => Some(Self::VerticalPipe),
            '-' => Some(Self::HorizontalPipe),
            'L' => Some(Self::NorthEastBend),
            'J' => Some(Self::NorthWestBend),
            '7' => Some(Self::SouthWestBend),
            'F' => Some(Self::SouthEastBend),
            '.' | 'O' | 'I' => Some(Self::Ground),
            'S' => Some(Self::StartingPosition),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Ground => '.',
            Self::HorizontalPipe => '-',
            Self::VerticalPipe => '|',
            Self::NorthEastBend => 'L',
            Self::NorthWestBend => 'J',
            Self::SouthEastBend => 'F',
            Self::SouthWestBend => '7',
            Self::StartingPosition => 'S',
        }
    }
}