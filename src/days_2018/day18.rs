// https://adventofcode.com/2018/day/11

use crate::{
    common::Solution,
    grid::{Grid, GridElement, Position},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LumberArea {
    Open,
    Trees,
    Lumberyard,
    Edge,
}

impl GridElement for LumberArea {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(Self::Lumberyard),
            '|' => Some(Self::Trees),
            '.' => Some(Self::Open),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Edge => '?',
            Self::Lumberyard => '#',
            Self::Open => '.',
            Self::Trees => '|',
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut a: Grid<LumberArea> = Grid::load(input).enclosed(LumberArea::Edge);

    let mut results: Vec<usize> = vec![];

    let p2 = loop {
        let mut b = a.clone();
        for row in 1..(a.rows - 1) {
            for col in 1..(a.cols - 1) {
                let position = Position { row, column: col };
                match a.get(&position).unwrap() {
                    LumberArea::Open => {
                        if position
                            .neighbors()
                            .filter(|p| a.get(p).unwrap() == LumberArea::Trees)
                            .count()
                            >= 3
                        {
                            b.set(&position, LumberArea::Trees);
                        }
                    }
                    LumberArea::Edge => {}
                    LumberArea::Trees => {
                        if position
                            .neighbors()
                            .filter(|p| a.get(p).unwrap() == LumberArea::Lumberyard)
                            .count()
                            >= 3
                        {
                            b.set(&position, LumberArea::Lumberyard);
                        }
                    }
                    LumberArea::Lumberyard => {
                        if !(position
                            .neighbors()
                            .any(|p| a.get(&p).unwrap() == LumberArea::Lumberyard)
                            && position
                                .neighbors()
                                .any(|p| a.get(&p).unwrap() == LumberArea::Trees))
                        {
                            b.set(&position, LumberArea::Open);
                        }
                    }
                }
            }
        }
        let trees = a.indices_matching(|p| *p == LumberArea::Trees).len();
        let lumberyards = a.indices_matching(|p| *p == LumberArea::Lumberyard).len();
        let result = trees * lumberyards;

        let n = results.len();
        if let Some(period) = results
            .iter()
            .rev()
            .position(|old_result| *old_result == result)
        {
            let period = period + 1;
            let offset = 1_000_000_000usize.rem_euclid(period);
            if period > 2 && (n + 1).rem_euclid(period) == offset {
                // Require a period of at least three, because my input has some spurious repeating values initially.
                // My actual period is 56.
                break result;
            }
        }

        results.push(result);
        a = b;
    };

    let p1 = results[9];
    Solution::new(p1, p2)
}
