// https://adventofcode.com/2023/day/10

use crate::{common::Solution, grid::{Grid, GridElement, Direction, Position}};

fn get_loop_and_starting_piece(start: &Position, direction: &Direction, maze: &Grid<PipeMazePart>) -> Option<(Vec<Position>, PipeMazePart)> {
    let mut d: Direction = direction.clone();
    let mut position = match direction {
        Direction::East => Position { row: start.row, column: start.column + 1 },
        Direction::North => Position { row: start.row - 1, column: start.column },
        Direction::West => Position { row: start.row, column: start.column - 1 },
        Direction::South => Position { row: start.row + 1, column: start.column },        
    };
    
    let mut the_loop: Vec<Position> = Vec::new();

    the_loop.push(start.clone());
    the_loop.push(position.clone());

    while position != *start {
        match maze[&position].follow(&d) {
            Some(new_direction) => { d = new_direction; },
            None => { return None }
        }

        position = match d {
            Direction::East => Position { row: position.row, column: position.column + 1 },
            Direction::North => Position { row: position.row - 1, column: position.column },
            Direction::West => Position { row: position.row, column: position.column - 1 },
            Direction::South => Position { row: position.row + 1, column: position.column },        
        };
        the_loop.push(position.clone());
    }
    let starting_part = match d {
        Direction::East => match direction {
            Direction::East => PipeMazePart::HorizontalPipe,
            Direction::North => PipeMazePart::NorthWestBend,
            Direction::South => PipeMazePart::SouthWestBend,
            Direction::West => panic!("?"),
            },
        Direction::North => match direction {
            Direction::North => PipeMazePart::VerticalPipe,
            Direction::South => panic!("?"),
            Direction::West => PipeMazePart::SouthWestBend,
            Direction::East => PipeMazePart::SouthEastBend,
        },
        Direction::South => match direction {
            Direction::South => PipeMazePart::VerticalPipe,
            Direction::North => panic!("?"),
            Direction::West => PipeMazePart::NorthWestBend,
            Direction::East => PipeMazePart::NorthEastBend,            
        },
        Direction::West => match direction {
            Direction::West => PipeMazePart::HorizontalPipe,
            Direction::North => PipeMazePart::NorthEastBend,
            Direction::South => PipeMazePart::SouthEastBend,
            Direction::East => panic!("?"),
        },
    };
    
    Some((the_loop, starting_part))
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum CrossingState {
    Inside,
    Outside,
    OutsideLeft,
    OutsideRight,
}

#[derive(PartialEq, Clone)]
enum InsideState {
    Unknown,
    Outside,
    OnTheLoop,
    Inside,
}

fn flood_fill_inside(inside: &mut Vec<InsideState>, from_position: &Position, maze: &Grid<PipeMazePart>) {
    inside[from_position.row * maze.cols + from_position.column] = InsideState::Inside;

    for p in from_position.nearest_neighbors() {
        if inside[p.row * maze.cols + p.column] == InsideState::Unknown {
            flood_fill_inside(inside, &p, maze);
        }    
    }
}

fn count_inside(the_loop: &Vec<Position>, maze: &Grid<PipeMazePart>) -> usize {

    let mut inside = vec![InsideState::Unknown; maze.rows * maze.cols];

    // The loop is known.
    for position in the_loop.iter() {
        inside[position.row * maze.cols + position.column] = InsideState::OnTheLoop;
    }

    // Set all easy outside positions
    for position in maze.positions_going_inward() { 
        if inside[position.row * maze.cols + position.column] == InsideState::Unknown {
            if position.row == 0 || position.column == 0 || position.row == maze.rows - 1 || position.column == maze.cols - 1 {
                inside[position.row * maze.cols + position.column] = InsideState::Outside; 
            } else if position.nearest_neighbors().any(|p| inside[p.row * maze.cols + p.column] == InsideState::Outside) {
                inside[position.row * maze.cols + position.column] = InsideState::Outside;                    
            }
        }
    }

    // Evaluate all other positions.
    for position in maze.positions() {
        if inside[position.row * maze.cols + position.column] != InsideState::Unknown {
            continue;
        }

        let mut state = CrossingState::Outside;
        let (direction, delta, start_value, _) = vec![Direction::East, Direction::West, Direction::South, Direction::North]
            .into_iter()
            .map(|d| match d {
                Direction::East => { 
                    let first_outside = (0..position.column).rev().find(|x| inside[position.row * maze.cols + x] == InsideState::Outside).unwrap();
                    (d, (1,0), (first_outside, position.row), position.column - first_outside)
                },
                Direction::West => { 
                    let first_outside = ((position.column + 1)..maze.cols).find(|x| inside[position.row * maze.cols + x] == InsideState::Outside).unwrap();
                    (d, (-1,0), (first_outside, position.row), first_outside - position.column)
                },
                Direction::South => { 
                    let first_outside = (0..position.row).rev().find(|y| inside[y * maze.cols + position.column] == InsideState::Outside).unwrap();
                    (d, (0,1), (position.column, first_outside), position.row - first_outside)
                },
                Direction::North => { 
                    let first_outside = ((position.row + 1)..maze.rows).find(|y| inside[y * maze.cols + position.column] == InsideState::Outside).unwrap();                
                    (d, (0,-1), (position.column, first_outside), first_outside - position.row)
                },
            })
            .min_by_key(|x| x.3).unwrap();

        let mut p = Position { row: start_value.1, column: start_value.0 };
        while p != position {
            if inside[p.row * maze.cols + p.column] == InsideState::Outside || !the_loop.contains(&p) { 
                if state == CrossingState::Outside {
                    inside[p.row * maze.cols + p.column] = InsideState::Outside;
                }
            } else {            
                state = maze[&p].transform_state(&state, &direction).unwrap();
            }

            p.row = p.row.checked_add_signed(delta.1).unwrap();
            p.column = p.column.checked_add_signed(delta.0).unwrap();
        }

        if state == CrossingState::Inside {
            flood_fill_inside(&mut inside, &position, maze);
        }
    }

    inside.into_iter().filter(|s| *s == InsideState::Inside).count()
}

pub fn solve(input: &str) -> Solution {
    let mut maze: Grid<PipeMazePart> = Grid::load(input);

    maze.enclose(PipeMazePart::Ground);

    let start = maze.find(&PipeMazePart::StartingPosition).unwrap();
    let dirs = vec![Direction::North, Direction::West, Direction::South];
    let (the_loop, missing_piece) = dirs.iter().find_map(|d| get_loop_and_starting_piece(&start, d, &maze)).unwrap();

    maze[&start] = missing_piece;

    let p1 = the_loop.len()/2;
    let p2 = count_inside(&the_loop, &mut maze);

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
            Self::SouthEastBend => Some((Direction::East, Direction::South)),
            Self::SouthWestBend => Some((Direction::West, Direction::South)),
            _ => None,
        }
    }

    fn transform_state(&self, state: &CrossingState, traveling: &Direction) -> Option<CrossingState> {
        match self.directions() {
            Some((d1,d2)) if traveling.reverse() == d1 || traveling.reverse() == d2 => {
                let out = if traveling.reverse() == d1 { d2 } else { d1 };
                match state {
                    CrossingState::Inside | CrossingState::Outside => None,
                    CrossingState::OutsideLeft => {
                        if out == traveling.counter_clockwise() {
                            Some(CrossingState::Inside)
                        } else if out == traveling.clockwise() {
                            Some(CrossingState::Outside)
                        } else {
                            Some(*state)
                        }},
                    &CrossingState::OutsideRight => {
                        if out == traveling.clockwise() {
                            Some(CrossingState::Inside)
                        } else if out == traveling.counter_clockwise() {
                            Some(CrossingState::Outside)
                        } else {
                            Some(*state)
                        }},
                }
            },
            Some((d1,d2)) if *traveling == d1 || *traveling == d2 => {
                let other = if *traveling == d1 { d2 } else { d1 };
                match state {
                    CrossingState::Inside => {
                        if other == traveling.counter_clockwise() {
                            Some(CrossingState::OutsideLeft)
                        } else if other == traveling.clockwise() {
                            Some(CrossingState::OutsideRight)
                        } else {
                            // This should be caught by the first case.
                            panic!("What is going on");
                        }
                    },
                    CrossingState::Outside => {
                        if other == traveling.counter_clockwise() {
                            Some(CrossingState::OutsideRight)
                        } else if other == traveling.clockwise() {
                            Some(CrossingState::OutsideLeft)
                        } else {
                            // This should be caught by the first case.
                            panic!("What is going on");
                        }
                    },
                    CrossingState::OutsideLeft | CrossingState::OutsideRight => None,
                }
            },
            Some(_) => match state {
                CrossingState::Inside => Some(CrossingState::Outside),
                CrossingState::Outside => Some(CrossingState::Inside),
                _ => None,
            },
            None => None,
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