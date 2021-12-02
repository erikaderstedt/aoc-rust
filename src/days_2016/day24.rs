use crate::common::Solution;
use pathfinding::prelude::dijkstra;
use itertools::Itertools;

use crate::grid::{Grid,GridElement,Position};

#[derive(Eq,PartialEq,Clone,Debug)]
enum DuctSpace {
    Open,
    Wall,
    You,
    PlaceOfInterest(char),
}

impl GridElement for DuctSpace {
    fn from_char(c: &char) -> Option<DuctSpace> {
        match c {
            '.' => Some(DuctSpace::Open),
            '#' => Some(DuctSpace::Wall),
            '0' => Some(DuctSpace::You),
            _ if c.is_ascii_digit() => Some(DuctSpace::PlaceOfInterest(c.clone())),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            DuctSpace::Open => '.',
            DuctSpace::You => '@',
            DuctSpace::Wall => '#',
            DuctSpace::PlaceOfInterest(c) => c.clone(),
        }
    }
}



pub fn solve(input: &str) -> Solution {
    let mut grid: Grid<DuctSpace> = Grid::load(input);
    println!("grid: {:?} {}", grid.cols, grid.rows);
    let start = grid.find(&DuctSpace::You).unwrap();
    println!("You start at {:?}", start);

//    let mut targets: Vec<Position> = ('1'..='4').map(|c| grid.find(&DuctSpace::PlaceOfInterest(c.clone())).unwrap()).collect();
    let mut targets: Vec<Position> = ('1'..='7').map(|c| grid.find(&DuctSpace::PlaceOfInterest(c.clone())).unwrap()).collect();
    targets.insert(0, start.clone());
    println!("Targets: {:?}", targets);
    for t in targets.iter() { grid[t] = DuctSpace::Open; }

    // Find shortest path between two targets
    let mut distances: Vec<Vec<usize>> = Vec::new();
    for _ in 0..targets.len() { distances.push(vec![0;targets.len()]) }
    for t1 in 0..(targets.len()-1) {
        for t2 in (t1+1)..targets.len() {
            let r = dijkstra(&targets[t1], |p| -> Vec<(Position,usize)> {
                let mut v: Vec<(Position,usize)> = Vec::new();

                if p.row > 0 && grid[&p.above()] == DuctSpace::Open { v.push((p.above(),1)) }
                if p.row < grid.rows - 1 && grid[&p.below()] == DuctSpace::Open { v.push((p.below(),1)) }
                if p.column > 0 && grid[&p.left()] == DuctSpace::Open { v.push((p.left(),1)) }
                if p.column < grid.cols - 1 && grid[&p.right()] == DuctSpace::Open { v.push((p.right(),1)) }
                v
            }, |p| -> bool {
                *p == targets[t2]
            }).expect("No path found");
            distances[t1][t2] = r.0.len() - 1;
            distances[t2][t1] = r.0.len() - 1;
            println!("{:?} and {:?}, {} steps", t1, t2, r.0.len());            
        }

    }
    println!("{:?}", distances);

    // There are 7! = 5040 different combinations.
    // println!("{}", [1,2,3,4,5,6,7].iter().permutations(7).count());
    let p1 = [1,2,3,4,5,6,7].iter().cloned().permutations(7).map(|p: Vec<usize>| -> usize {
        distances[0][p[0]] + 
        distances[p[0]][p[1]] +
        distances[p[1]][p[2]] +
        distances[p[2]][p[3]] +
        distances[p[3]][p[4]] +
        distances[p[4]][p[5]] +
        distances[p[5]][p[6]]        
    }).min().unwrap();
    let p2 = [1,2,3,4,5,6,7].iter().cloned().permutations(7).map(|p: Vec<usize>| -> usize {
        distances[0][p[0]] + 
        distances[p[0]][p[1]] +
        distances[p[1]][p[2]] +
        distances[p[2]][p[3]] +
        distances[p[3]][p[4]] +
        distances[p[4]][p[5]] +
        distances[p[5]][p[6]] +
        distances[p[6]][0]  
    }).min().unwrap();

    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}