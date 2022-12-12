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

#[derive(Clone,PartialEq,Eq, Hash)]
pub struct Position { pub row: Row, pub column: Column }

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.column, self.row)
    }
}

impl Position {
    pub fn above(&self) -> Position { Position { row: self.row - 1, column: self.column }}
    // pub fn above_left(&self) -> Position { Position { row: self.row - 1, column: self.column - 1 }}
    // pub fn above_right(&self) -> Position { Position { row: self.row - 1, column: self.column + 1}}
    pub fn below(&self) -> Position { Position { row: self.row + 1, column: self.column }}
    // pub fn below_left(&self) -> Position { Position { row: self.row + 1, column: self.column - 1 }}
    // pub fn below_right(&self) -> Position { Position { row: self.row + 1, column: self.column + 1}}

    pub fn left(&self) -> Position { Position { row: self.row, column: self.column - 1 }}
    pub fn right(&self) -> Position { Position { row: self.row, column: self.column + 1 }}

    // pub fn origin() -> Position { Position { row: 0usize, column: 0 } }
}

impl<T: GridElement> Grid<T> {

    #[allow(dead_code)]

    pub fn load(input: &str) -> Grid<T> {
        let cols = input.lines().next().unwrap().len();
        let locations: Vec<T> = input.chars().filter_map(|c| if c.is_whitespace() { None } else { T::from_char(&c) }).collect();
        let rows = locations.len()/cols;
        assert!(rows * cols == locations.len(), "Grid is not rectangular, perhaps some items won't parse");
        Grid { rows, cols, locations }
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

    pub fn find(&self, element: &T) -> Option<Position> {
        self.locations.iter().position(|i| i == element).map(|l| Position { row: l / self.cols, column: l % self.cols })
    }

    // pub fn positions(&self) -> GridIterator {
    //     GridIterator { row: 0, col: 0, min_col:0, max_col: self.cols, max_row: self.rows }
    // }

    // pub fn display(&self) {
    //     for row in 0..self.rows {
    //         let s: String = (0..self.cols).map(|column| -> char { 
    //             let p = Position { row, column };
    //             self.get(&p).unwrap().to_char()
    //         }).collect();
    //         println!("{}", s);
    //     }
    // }

    // Iterate over grid elements of a certain type.
    // Iterate over all grid points together with position

    // 
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

    // pub fn get(&self, p: &Position) -> Option<T> {
    //     let i = p.row * self.cols + p.column;
    //     if p.row < self.rows && p.column < self.cols && i < self.locations.len() {
    //         Some(self.locations[i].clone())
    //     } else {
    //         None
    //     }
    // }
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

// pub struct GridIterator {
//     row: usize,
//     col: usize,
//     min_col: usize,
//     max_col: usize,
//     max_row: usize,
// }

// impl Iterator for GridIterator {
//     type Item = Position;

//     fn next(&mut self) -> Option<Position> {
//         if self.row >= self.max_row {
//             None
//         } else {
//             let v = Some(Position { row: self.row, column: self.col });
//             self.col += 1;
//             if self.col == self.max_col {
//                 self.row += 1;
//                 self.col = self.min_col;
//             }
//             v
//         }
//     }
// }

