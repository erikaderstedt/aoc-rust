// https://adventofcode.com/2017/day/22

use crate::{
    common::Solution,
    grid::{Direction, Grid, GridElement, Position},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

const SZ: usize = 512;

fn evolve<F>(original: &Grid<Node>, iterations: usize, evolver: F) -> usize
where
    F: Fn(&Node, &Direction) -> (Node, Direction),
{
    // Create empty grid SZxSZ and put the input in the middle.
    let mut grid: Grid<Node> = Grid {
        rows: SZ,
        cols: SZ,
        locations: vec![Node::Clean; SZ * SZ],
    };

    for r in 0..original.rows {
        let y = SZ / 2 + r - original.rows / 2;
        let x = SZ / 2 - original.cols / 2;
        for i in 0..original.cols {
            grid.locations[y * grid.cols + x + i] = original.locations[r * original.cols + i];
        }
    }

    let mut num_infections = 0;
    let mut p = Position {
        row: SZ / 2,
        column: SZ / 2,
    };
    let mut d = Direction::North;
    for _ in 0..iterations {
        let n = grid.get(&p).unwrap();
        let (new_n, new_d) = evolver(&n, &d);
        if new_n == Node::Infected {
            num_infections += 1;
        }
        d = new_d;
        grid.set(&p, new_n);
        p = p.along(&d);
    }

    num_infections
}

pub fn solve(input: &str) -> Solution {
    let original: Grid<Node> = Grid::load(input);

    let p1 = evolve(&original, 10000, |node, direction| match node {
        Node::Clean => (Node::Infected, direction.counter_clockwise()),
        Node::Infected => (Node::Clean, direction.clockwise()),
        Node::Flagged | Node::Weakened => panic!("Not possible in part 1."),
    });
    let p2 = evolve(&original, 10000000, |node, direction| match node {
        Node::Clean => (Node::Weakened, direction.counter_clockwise()),
        Node::Infected => (Node::Flagged, direction.clockwise()),
        Node::Weakened => (Node::Infected, direction.clone()),
        Node::Flagged => (Node::Clean, direction.reverse()),
    });

    Solution::new(p1, p2)
}

impl GridElement for Node {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(Self::Infected),
            '.' => Some(Self::Clean),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        panic!("Not implemented.");
    }
}
