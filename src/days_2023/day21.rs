// https://adventofcode.com/2023/day/21

use crate::{common::Solution, grid::{Grid, GridElement}};

#[derive( PartialEq, Eq, Clone)]
enum Garden {
    Start,
    Plots,
    Rocks,
    Visited,
}

const P1_STEPS: usize = 64;
const P2_STEPS: usize = 26501365;

pub fn solve(input: &str) -> Solution {
    let mut original_grid: Grid<Garden> = Grid::load(input);
    let original_start_index = original_grid.locations.iter().position(|l| *l == Garden::Start).unwrap();
    original_grid.locations[original_start_index] = Garden::Plots;
    let original_x = original_start_index % original_grid.cols;
    let original_y = original_start_index / original_grid.cols;
    
    // Repeat grid 5x5. Since the elf reaches at most 1 step out per iteration, 
    // and we need 2x131 + 65 steps this is enough.
    
    // Also, any point previously activated will flip back and forth.
    // Just remember the recently activated in the previous iteration, and walk from them. Mutate
    // the grid to say Visited when we reach a spot.
    let sz = original_grid.rows;
    let mut grid = original_grid.repeated(5, 5);
    let start_index = (original_y + 2 * sz) * grid.cols +  original_x + 2 * sz;
    grid.locations[start_index] = Garden::Visited;

    let mut p1 = 0;
    let mut reached: Vec<usize> = vec![start_index];
    let mut counts = vec![];

    let mut visited_on_even_steps = 1;
    let mut visited_on_odd_steps = 0;

    let mut step = 0;
    loop {
        let step_is_even = step % 2 == 0;
        let num_visited = || if step_is_even { visited_on_even_steps } else { visited_on_odd_steps };

        if step == P1_STEPS { p1 = num_visited() }
        if step % sz == P2_STEPS % sz {            
            counts.push(num_visited());
            if counts.len() == 3 {
                break;
            }
        }
        
        // Reached are the ones we reached last iteration.
        let mut v = vec![];
        for index in reached.into_iter() {
            let c = grid.cols;
            let mut handle = |o: usize| { 
                if grid.locations[o] == Garden::Plots { 
                    v.push(o.clone());
                    if !step_is_even {
                        visited_on_even_steps += 1;
                    } else {
                        visited_on_odd_steps += 1;
                    }
                    grid.locations[o] = Garden::Visited;
                }};

            handle(index - c);
            handle(index + c);
            handle(index - 1);
            handle(index + 1);
        }
        reached = v;
        step += 1;
    }
    
    // counts[n] = A * n^2 / 2 + C * n + D
    // counts[0] = 0 + 0 + D
    // counts[1] = A/2 + C + D
    // counts[2] = A * 2 + 2 * C + D
    let d = counts[0];
    let a = counts[2] - counts[1]*2 + d;
    let c = counts[1] - d - a/2;
        
    let n = P2_STEPS / sz;
    let p2 = a * n * n / 2 + c * n + d;

    Solution::new(p1, p2)
}

impl GridElement for Garden {
    fn from_char(c: &char) -> Option<Self> { 
        match c {
            '.' => Some(Self::Plots),
            '#' => Some(Self::Rocks),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
    fn to_char(&self) -> char { 
        match self {
            Self::Plots => '.',
            Self::Rocks => '#',
            Self::Start => 'S',
            Self::Visited => 'O',
        }
    }
}