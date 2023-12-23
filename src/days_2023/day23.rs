// https://adventofcode.com/2023/day/23

use std::collections::HashSet;

use crate::{common::Solution, grid::{Grid, Direction, GridElement}};

#[derive(Debug,PartialEq, Eq,Clone)]
enum Stuff {
    Path,
    Forest,
    Slope(Direction)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Explorer {
    x: usize,
    y: usize,
    visited: Vec<usize>,
}

impl Explorer {

    fn at(index: usize, width: usize) -> Self {
        Explorer { x: index % width, y: index / width, visited: vec![] }
    }

    fn successors(&self, grid: &Grid<Stuff>) -> Vec<Explorer> {
        let mut v = self.visited.clone();
        v.push(self.y * grid.cols + self.x);

        if self.x == grid.cols - 2 && self.y == grid.rows - 1 {
            vec![]
        } else if self.x == 1 && self.y == 0 {
            vec![Explorer { x: 1, y: 1, visited: v} ]
        } else {
            match &grid.locations[self.y * grid.cols + self.x] {
                Stuff::Forest => panic!("??"),
                Stuff::Slope(direction) => {
                    let (x2,y2) = match direction {
                        Direction::East => (self.x + 1, self.y),
                        Direction::North => (self.x, self.y - 1),
                        Direction::South => (self.x, self.y + 1),
                        Direction::West => (self.x - 1, self.y),
                    };
                    let idx = y2 * grid.cols + x2;
                    if grid.locations[idx] != Stuff::Forest && !v.contains(&idx) {
                        vec![Explorer { x: x2, y: y2, visited: v }]
                    } else {
                        vec![]
                    }
                },
                Stuff::Path => {
                    let mut s = vec![];
                    for direction in [Direction::West, Direction::East, Direction::South, Direction::North].iter() {
                        let (x2,y2) = match direction {
                            Direction::East => (self.x + 1, self.y),
                            Direction::North => (self.x, self.y - 1),
                            Direction::South => (self.x, self.y + 1),
                            Direction::West => (self.x - 1, self.y),
                        };
                        let idx = y2 * grid.cols + x2;
                        if grid.locations[idx] != Stuff::Forest && 
                            grid.locations[idx] != Stuff::Slope(direction.reverse()) && 
                            !v.contains(&idx) {
                            s.push(Explorer { x: x2, y: y2, visited: v.clone() });
                        }
                    }
                    s
                },
            }
        }
    }
}

#[derive(PartialEq, Eq,Clone)]
struct Node {
    location: usize,
    paths: Vec<(u8,u16)>
}

#[derive(PartialEq, Eq,Clone)]
struct GraphState {
    node_index: u8,
    visited: u64,
    cost: u16,
}

impl GraphState {
    fn extend_with_successors(&self, check: &mut Vec<GraphState>, graph: &Vec<Node>) {
        check.extend(graph[self.node_index as usize].paths.iter()
            .filter_map(|(destination, cost)|
                if self.visited & (1 << destination) != 0 {
                    None
                } else {
                    Some( GraphState { 
                        node_index: *destination,
                        visited: self.visited | (1 << self.node_index),
                        cost: self.cost + cost})
                }))
    }
}

struct Edge {
    source: usize, destination: usize, cost: u16
}

fn build_graph(start: usize, grid: &Grid<Stuff>) -> Vec<Node> {

    let mut sources_to_try = vec![Explorer::at(start, grid.cols)];
    let mut v: Vec<Edge> = vec![];

    while let Some(initial) = sources_to_try.pop() {
        for start in initial.successors(grid).into_iter() {
            let mut check: Vec<Explorer> = Vec::new();

            let mut s2 = start.clone();
            s2.visited = vec![initial.y * grid.cols + initial.x];

            check.push(s2);
        
            while let Some(c) = check.pop() {
                let s = c.successors(&grid);
                
                if s.len() > 1 {
                    let idx = c.x + c.y * grid.cols;
                    v.push(Edge { 
                        source: initial.x + initial.y * grid.cols, 
                        destination: idx, cost: c.visited.len() as u16 });
                    
                    if v.iter().find(|connection| connection.source == idx).is_none() {
                        sources_to_try.push(c);
                    }

                    break;
                } else if s.len() == 1 {
                    let idx = s[0].x + s[0].y * grid.cols;
                    if s[0].x == grid.cols - 2 && s[0].y == grid.rows - 1 {
                        v.push(Edge { 
                            source: initial.x + initial.y * grid.cols, 
                            destination: idx, cost: c.visited.len() as u16 });
                    } else {
                        check.extend(s);
                    }
                }
            }
        }
    }

    let mut node_incides: HashSet<usize> = v.iter().map(|e| e.source).collect();
    node_incides.insert((grid.cols - 2) + (grid.rows-1)*grid.cols);

    let mut nodes: Vec<Node> = node_incides.iter().map(|i| Node { location: *i, paths: vec![] }).collect();

    for index in 0..nodes.len() {
        let paths: HashSet<(u8,u16)> = v.iter()
            .filter(|e| e.source == nodes[index].location)
            .map(|e| (nodes.iter().position(|n| n.location == e.destination).unwrap() as u8, e.cost))
            .collect();
        nodes[index].paths = paths.into_iter().collect();
    }

    nodes
}

fn find_longest_path(grid: &Grid<Stuff>) -> u16 {
    let nodes = build_graph(1, &grid);
    let goal = (grid.cols - 2) + (grid.rows - 1)*grid.cols;
    let start = nodes.iter().position(|n| n.location == 1).unwrap() as u8;
    let end = nodes.iter().position(|n| n.location == goal).unwrap() as u8;

    let mut p = 0;
    let mut check: Vec<GraphState> =vec![GraphState { node_index: start, visited: 0, cost: 1}];

    while let Some(c) = check.pop() {
        if c.node_index == end && c.cost > p {
            p = c.cost;
        } else {
            c.extend_with_successors(&mut check, &nodes);            
        }
    }
    p
}

pub fn solve(input: &str) -> Solution {
    let mut grid: Grid<Stuff> = Grid::load(input);

    let p1 = find_longest_path(&grid);

    for l in grid.locations.iter_mut() {
        if let Stuff::Slope(_) = l {
            *l = Stuff::Path;
        }
    }

    let p2 = find_longest_path(&grid);
    
    Solution::new(p1, p2)
}

impl GridElement for Stuff {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.'=> Some(Stuff::Path),
            '#' => Some(Stuff::Forest),
            '^' => Some(Stuff::Slope(Direction::North)),
            '<' => Some(Stuff::Slope(Direction::West)),
            'v' => Some(Stuff::Slope(Direction::South)),
            '>' => Some(Stuff::Slope(Direction::East)),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Stuff::Forest => '#',
            Stuff::Path => '.',
            Stuff::Slope(Direction::East) => '>',
            Stuff::Slope(Direction::West) => '<',
            Stuff::Slope(Direction::North) => '^',
            Stuff::Slope(Direction::South) => 'v',
        }
    }
}