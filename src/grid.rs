use std::marker::Sized;
use std::ops::{Index,IndexMut};
use std::cmp::{PartialEq,Eq};
use std::fmt;

pub trait GridElement: Sized + PartialEq + Eq + Clone {
    fn from_char(c: &char) -> Option<Self>;
    fn to_char(&self) -> char;
}

#[derive(Clone,PartialEq,Eq)]
pub struct Grid<T: GridElement> {
    pub rows: usize,
    pub cols: usize,
    pub locations: Vec<T>,
}

type Row = usize;
type Column = usize;

#[derive(Debug,Clone, PartialEq, Eq,Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {

    pub fn u8(&self) -> u8 {
        match self {
            Direction::East => 1u8,
            Direction::North => 2u8,
            Direction::South => 4u8,
            Direction::West => 8u8,
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    #[allow(dead_code)]
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    #[allow(dead_code)]
    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    #[allow(dead_code)]
    pub fn turns(&self) -> [Direction; 2] {
        match self {
            Direction::East | Direction::West => [Direction::North, Direction::South],
            Direction::North | Direction::South => [Direction::West, Direction::East],
        }
    }
}

#[derive(Clone,PartialEq,Eq, Hash)]
pub struct Position { pub row: Row, pub column: Column }

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.column, self.row)
    }
}
#[allow(dead_code)]
impl Position {
    pub fn above(&self) -> Position { Position { row: self.row - 1, column: self.column }}
    pub fn above_left(&self) -> Position { Position { row: self.row - 1, column: self.column - 1 }}
    pub fn above_right(&self) -> Position { Position { row: self.row - 1, column: self.column + 1}}
    pub fn below(&self) -> Position { Position { row: self.row + 1, column: self.column }}
    pub fn below_left(&self) -> Position { Position { row: self.row + 1, column: self.column - 1 }}
    pub fn below_right(&self) -> Position { Position { row: self.row + 1, column: self.column + 1}}
    pub fn left(&self) -> Position { Position { row: self.row, column: self.column - 1 }}
    pub fn right(&self) -> Position { Position { row: self.row, column: self.column + 1 }}

    pub fn along(&self, direction: &Direction) -> Self {
        match direction {
            Direction::East => Position { row: self.row, column: self.column + 1 },
            Direction::North => Position { row: self.row - 1, column: self.column },
            Direction::West => Position { row: self.row, column: self.column - 1 },
            Direction::South => Position { row: self.row + 1, column: self.column },        
        }
    }

    // pub fn origin() -> Position { Position { row: 0usize, column: 0 } }

    pub fn neighbors(&self) -> NeighborIterator {
        NeighborIterator { center_row: self.row, center_col: self.column, index: 0 }
    }

    pub fn nearest_neighbors(&self) -> NearestNeighborIterator {
        NearestNeighborIterator { center_row: self.row, center_col: self.column, index: 0 }
    }
}

impl<T: GridElement> Grid<T> {

    #[allow(dead_code)]
    pub fn load(input: &str) -> Grid<T> {
        let cols = input.as_bytes().iter().position(|c| *c == '\n' as u8).unwrap();
        let locations: Vec<T> = input.chars().filter_map(|c| T::from_char(&c)).collect();
        let rows = locations.len()/cols;
        assert!(rows * cols == locations.len(), "Grid is not rectangular, perhaps some items won't parse");
        Grid { rows, cols, locations }
    }

    #[allow(dead_code)]
    pub fn repeated(&self, across: usize, down: usize) -> Grid<T> {
        let ncols = self.cols * across;
        let nrows = self.rows * down;
        let locations: Vec<T> = (0..nrows)
            .map(|row| {
                self.locations.iter().skip(row * self.cols).take(self.cols).cycle().take(ncols)
            })
            .flatten()
            .cycle()
            .take(ncols*nrows)
            .cloned()
            .collect();
        Grid { rows: ncols, cols: nrows, locations}
    }

    // pub fn safe_index(&self, y: i64, x: i64) -> Option<usize> {
    //     if x < 0 || y < 0 { return None };
    //     let row = y as usize;
    //     let column = x as usize;
    //     let index = column + row*self.cols;
    //     if column >= self.cols || index >= self.locations.len() { return None }
    //     Some(index)
    // }
    #[allow(dead_code)]

    pub fn enclose(&mut self, element: T) {
        self.rows += 2;
        self.cols += 2;
        let mut l: Vec<T> = Vec::with_capacity(self.rows * self.cols);
        
        for _c in 0..self.cols { l.push(element.clone()) }
        for r in 1..(self.rows-1) {
            l.push(element.clone());
            for c in 1..(self.cols-1) {
                l.push(self.locations[(r-1)*(self.cols-2) + c-1].clone());
            }
            l.push(element.clone());
        }
        for _c in 0..self.cols { l.push(element.clone()) }
        self.locations = l;
    }

    #[allow(dead_code)]
    pub fn find(&self, element: &T) -> Option<Position> {
        self.locations.iter().position(|i| i == element).map(|l| Position { row: l / self.cols, column: l % self.cols })
    }

    pub fn positions(&self) -> GridIterator {
        GridIterator { row: 0, col: 0, min_col:0, max_col: self.cols, max_row: self.rows }
    }

    #[allow(dead_code)]
    pub fn positions_going_inward(&self) -> InwardGridIterator {
        InwardGridIterator::of_size(self.rows, self.cols)
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        for row in 0..self.rows {
            let s: String = (0..self.cols).map(|column| -> char { 
                let p = Position { row, column };
                self.get(&p).unwrap().to_char()
            }).collect();
            println!("{}", s);
        }
    }

    // Iterate over grid elements of a certain type.
    // Iterate over all grid points together with position
    #[allow(dead_code)]
    pub fn neighbor_positions_satisfying_condition<F>(&self, position: &Position, include_neighbor: F) -> Vec<Position> 
        where F: Copy + FnOnce(&T,&T) -> bool    
    {
        let mut n = Vec::new();
        let this_value = &self[position];
        if position.row > 0 && include_neighbor(this_value, &self[&position.above()]) { n.push(position.above())}
        if position.row < self.rows - 1 && include_neighbor(this_value, &self[&position.below()]) { n.push(position.below())}
        if position.column > 0 && include_neighbor(this_value, &self[&position.left()]) { n.push(position.left())}
        if position.column < self.cols - 1 && include_neighbor(this_value, &self[&position.right()]) { n.push(position.right())}
        n
    }

    // pub fn neighbors(&self, position: &Position) -> Vec<&T> {
    //     let mut n = Vec::new();
    //     if position.row > 0 { n.push(&self[&position.above()]) }
    //     if position.row < self.rows - 1 { n.push(&self[&position.below()]) }
    //     if position.column > 0 { n.push(&self[&position.left()]) }
    //     if position.column < self.cols - 1 { n.push(&self[&position.right()]) }

    //     // if position.row > 0 && position.column > 0 { n.push(&self[&position.above_left()]) }
    //     // if position.row > 0 && position.column < self.cols - 1 { n.push(&self[&position.above_right()]) }
    //     // if position.row < self.rows - 1 && position.column > 0 { n.push(&self[&position.below_left()]) }
    //     // if position.row < self.rows - 1 && position.column < self.cols - 1 { n.push(&self[&position.below_right()]) }

    //     n
    // }

    pub fn get(&self, p: &Position) -> Option<T> {
        let i = p.row * self.cols + p.column;
        if p.row < self.rows && p.column < self.cols && i < self.locations.len() {
            Some(self.locations[i].clone())
        } else {
            None
        }
    }

}

impl<T: GridElement> Index<&Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.locations[index.row * self.cols + index.column]
    }
}

impl<T: GridElement> IndexMut<&Position> for Grid<T> {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        &mut self.locations[index.row * self.cols + index.column]
    }
}

pub struct GridIterator {
    row: usize,
    col: usize,
    min_col: usize,
    max_col: usize,
    max_row: usize,
}

impl Iterator for GridIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.row >= self.max_row {
            None
        } else {
            let v = Some(Position { row: self.row, column: self.col });
            self.col += 1;
            if self.col == self.max_col {
                self.row += 1;
                self.col = self.min_col;
            }
            v
        }
    }
}

pub struct InwardGridIterator {
    direction: Direction,
    row: usize,
    col: usize,
    east_end: usize, // cols-1
    west_end: usize, // 0
    south_end: usize, // rows-1
    north_end: usize, // 1
    total: usize, // rows * cols
}

impl InwardGridIterator {
    #[allow(dead_code)]
    fn of_size(rows: usize, cols: usize) -> InwardGridIterator {
        InwardGridIterator { direction: Direction::East, 
            row: 0, col: 0, east_end: cols - 1, west_end: 0, south_end: rows-1, north_end: 1, total: rows*cols }
    }
}

impl Iterator for InwardGridIterator {
    type Item = Position;
    // east, south, use end_col, end_row.
    // end_col, 0, end_col - 1, 1, end_col - 2
    fn next(&mut self) -> Option<Position> {
        if self.total == 0 { None } else {
            let p = Some(Position { row: self.row, column: self.col});

            match self.direction {
                Direction::East => {
                    self.col += 1;
                    if self.col == self.east_end {
                        self.direction = Direction::South;
                        self.east_end -= 1
                    }
                },
                Direction::South => {
                    self.row += 1;
                    if self.row == self.south_end {
                        self.direction = Direction::West;
                        self.south_end -= 1
                    }
                },
                Direction::West => {
                    self.col -= 1;
                    if self.col == self.west_end {
                        self.direction = Direction::North;
                        self.west_end += 1
                    }
                }
                Direction::North => {
                    self.row -= 1;
                    if self.row == self.north_end {
                        self.direction = Direction::East;
                        self.north_end += 1;
                    }
                }
            }
            self.total -= 1;
            p
        }
    }
}

pub struct NeighborIterator {
    center_row: usize,
    center_col: usize,
    index: usize,
}

impl Iterator for NeighborIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let n = match self.index {
            0 => Some( Position { row: self.center_row - 1, column: self.center_col - 1 }),
            1 => Some( Position { row: self.center_row - 1, column: self.center_col }),
            2 => Some( Position { row: self.center_row - 1, column: self.center_col + 1 }),
            3 => Some( Position { row: self.center_row, column: self.center_col - 1}),
            4 => Some( Position { row: self.center_row, column: self.center_col + 1}),
            5 => Some( Position { row: self.center_row + 1, column: self.center_col - 1}),
            6 => Some( Position { row: self.center_row + 1, column: self.center_col }),
            7 => Some( Position { row: self.center_row + 1, column: self.center_col + 1}),
            _ => None,
        };
        self.index = self.index + 1;
        n
    }
}

pub struct NearestNeighborIterator {
    center_row: usize,
    center_col: usize,
    index: usize,
}

impl Iterator for NearestNeighborIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let n = match self.index {
            0 => Some( Position { row: self.center_row - 1, column: self.center_col }),
            1 => Some( Position { row: self.center_row, column: self.center_col - 1}),
            2 => Some( Position { row: self.center_row, column: self.center_col + 1}),
            3 => Some( Position { row: self.center_row + 1, column: self.center_col }),
            _ => None,
        };
        self.index = self.index + 1;
        n
    }
}
