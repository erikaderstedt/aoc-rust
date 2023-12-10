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

#[derive(PartialEq, Eq, Debug)]
enum CrossingState {
    Inside,
    Outside,
    OutsideLeft,
    OutsideRight,
}

// impl CrossingState {

//     fn transform(&self, direction: &Direction, part: &PipeMazePart) -> CrossingState {
//         match *part {
//             PipeMazePart::Ground | PipeMazePart::StartingPosition => self.clone(),
//             PipeMazePart::HorizontalPipe => match direction {
//                 Direction::East | Direction::West => panic!("Invalid transform"),
//                 Direction::North | Direction::South => self.clone(),
//             },
//             PipeMazePart::VerticalPipePipe => match direction {
//                 Direction::North | Direction::South => panic!("Invalid transform"),
//                 Direction::East | Direction::West => self.clone(),
//             },
//             PipeMazePart::NorthEastBend => match direction {
//                 Direction::East if self == CrossingState::Inside => CrossingState::OutsideLeft,
//                 Direction::East if self == CrossingState::Outside => CrossingState::OutsideRight,
//                 Direction::West if self == CrossingState::OutsideLeft => CrossingState::Outside,
//                 Direction::West if self == CrossingState::OutsideRight => CrossingState::Inside,
//                 Direction::North if self == CrossingState::No
                
//                 }
//             }
//         }
//     }
// }

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

    // Set all easy outside positions
    for position in maze.positions_going_inward() { 
        if position.row == 0 || position.column == 0 || position.row == maze.rows - 1 || position.column == maze.cols - 1 {
            inside[position.row * maze.cols + position.column] = InsideState::Outside; 
        } else if the_loop.contains(&position) {
            continue;
        } else if position.nearest_neighbors().any(|p| inside[p.row * maze.cols + p.column] == InsideState::Outside) {
            inside[position.row * maze.cols + position.column] = InsideState::Outside;                    
        }
    }

    for position in the_loop.iter() {
        inside[position.row * maze.cols + position.column] = InsideState::OnTheLoop;
    }

    for position in maze.positions() {
        if inside[position.row * maze.cols + position.column] != InsideState::Unknown {
            continue;
        }

        let mut state = CrossingState::Outside;
        if  position.column > maze.cols / 2 {
            let mut column = maze.cols - 1;
            while column >= position.column {
                let p = Position { row: position.row, column };
                column -= 1;     

                if !the_loop.contains(&p) { 
                    if state == CrossingState::Outside {
                        inside[p.row * maze.cols + p.column] = InsideState::Outside;
                    }
                    continue; 
                }

                state = match maze[&p] {
                    PipeMazePart::HorizontalPipe | PipeMazePart::Ground | PipeMazePart::StartingPosition => state,
                    PipeMazePart::VerticalPipe => match state {
                            CrossingState::Outside => CrossingState::Inside,
                            CrossingState::Inside => CrossingState::Outside,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::NorthEastBend => match state {
                            CrossingState::OutsideLeft => CrossingState::Outside,
                            CrossingState::OutsideRight => CrossingState::Inside,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::NorthWestBend => match state {
                            CrossingState::Outside => CrossingState::OutsideLeft,
                            CrossingState::Inside => CrossingState::OutsideRight,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::SouthWestBend => match state {
                            CrossingState::Outside => CrossingState::OutsideRight,
                            CrossingState::Inside => CrossingState::OutsideLeft,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::SouthEastBend => match state {
                            CrossingState::OutsideLeft => CrossingState::Inside,
                            CrossingState::OutsideRight => CrossingState::Outside,
                            _ => panic!("Invalid pipe part!"),
                        },
                    };
            }
        } else {
            for column in 1..position.column {
                let p = Position { row: position.row, column };
                
                if !the_loop.contains(&p) { 
                    if state == CrossingState::Outside {
                        inside[p.row * maze.cols + p.column] = InsideState::Outside;
                    }
                    continue; 
                }

                state = match maze[&p] {
                    PipeMazePart::HorizontalPipe | PipeMazePart::Ground | PipeMazePart::StartingPosition => state,
                    PipeMazePart::VerticalPipe => match state {
                            CrossingState::Outside => CrossingState::Inside,
                            CrossingState::Inside => CrossingState::Outside,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::NorthWestBend => match state {
                            CrossingState::OutsideLeft => CrossingState::Inside,
                            CrossingState::OutsideRight => CrossingState::Outside,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::NorthEastBend => match state {
                            CrossingState::Outside => CrossingState::OutsideRight,
                            CrossingState::Inside => CrossingState::OutsideLeft,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::SouthEastBend => match state {
                            CrossingState::Outside => CrossingState::OutsideLeft,
                            CrossingState::Inside => CrossingState::OutsideRight,
                            _ => panic!("Invalid pipe part!"),
                        },
                    PipeMazePart::SouthWestBend => match state {
                            CrossingState::OutsideLeft => CrossingState::Outside,
                            CrossingState::OutsideRight => CrossingState::Inside,
                            _ => panic!("Invalid pipe part!"),
                        },
                    };            
            }
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
    fn follow(&self, direction: &Direction) -> Option<Direction> {
        match self {
            PipeMazePart::Ground | PipeMazePart::StartingPosition => None,
            PipeMazePart::HorizontalPipe | PipeMazePart::VerticalPipe => Some(direction.clone()),
            PipeMazePart::NorthEastBend => {
                match direction {
                    Direction::South => Some(Direction::East),
                    Direction::West => Some(Direction::North),
                    _ => None,
                }
            }
            PipeMazePart::NorthWestBend => {
                match direction {
                    Direction::South => Some(Direction::West),
                    Direction::East => Some(Direction::North),
                    _ => None,
                }
            }
            PipeMazePart::SouthEastBend => {
                match direction {
                    Direction::North => Some(Direction::East),
                    Direction::West => Some(Direction::South),
                    _ => None,
                }
            }
            PipeMazePart::SouthWestBend => {
                match direction {
                    Direction::North => Some(Direction::West),
                    Direction::East => Some(Direction::South),
                    _ => None,
                }
            }
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