// https://adventofcode.com/2023/day/22

use std::{str::FromStr, collections::HashSet};
use itertools::Itertools;
use crate::common::{Solution, parsed_from_each_line};

#[derive(Clone)]
struct Brick {
    start: (u8, u8, u16),
    end: (u8, u8, u16),
}

type Space = Vec<Vec<Vec<u16>>>;

impl Brick {
    
    fn initial(start: (u16, u16, u16), end: (u16, u16, u16)) -> Brick {
        Brick { start: (start.0 as u8, start.1 as u8, start.2), 
            end: (end.0 as u8, end.1 as u8, end.2) }
    }

    fn configure_space(&self, space: &mut Space, index: u16) {
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                for z in self.start.2..=self.end.2 {
                    space[x as usize][y as usize][z as usize] = index;
                }
            }
        }
    }

    fn above(&self, space: &Space) -> HashSet<u16> {
        let mut above: HashSet<u16> = HashSet::new();
        let z = self.end.2 as usize;

        for y in self.start.1..=self.end.1 {
            for x in self.start.0..=self.end.0 {
                let v = space[x as usize][y as usize][z+1];
                if v != 0 {
                    above.insert(v);
                }
            }
        }

        above
    }

    fn below(&self, space: &Space) -> HashSet<u16> {
        let mut below: HashSet<u16> = HashSet::new();
        let z = self.start.2 as usize;

        for y in self.start.1..=self.end.1 {
            for x in self.start.0..=self.end.0 {
                let v = space[x as usize][y as usize][z-1];
                if v != 0 {
                    below.insert(v);
                }
            }
        }

        below
    }
}

fn drop_bricks(bricks: &mut Vec<Brick>, space: &mut Space, start_at: usize) -> usize {

    let mut total_moved = vec![false; bricks.len()];
    let mut first_moved = start_at;
    loop {
        let mut num_moved = 0;
        for (index,brick) in bricks.iter_mut().enumerate().skip(first_moved) {
            let z = brick.start.2 as usize;

            if let Some(new_z) = 
                (1..z).rev()
                    .take_while(|z|
                        (brick.start.0..=brick.end.0)
                            .all(|x| (brick.start.1..=brick.end.1)
                                .all(|y| space[x as usize][y as usize][*z] == 0)))
                    .last() {
                    
                    brick.configure_space(space, 0);

                    brick.end.2 = brick.end.2 - (z - new_z) as u16;
                    brick.start.2 = new_z as u16;

                    brick.configure_space(space, (index as u16) +1);

                    first_moved = index + 1;
                    total_moved[index] = true;
                    num_moved += 1;
            }
        }
        if num_moved == 0 { break; }
    }
    total_moved.into_iter().filter(|t| *t).count()
}

pub fn solve(input: &str) -> Solution {
    let mut bricks: Vec<Brick> = parsed_from_each_line(input);
    let mut space = vec![vec![vec![0; 300]; 10]; 10];
    
    bricks.sort_unstable_by_key(|b| b.start.2);

    // The actual number does not matter, just that it is unique amond 
    for (index, brick) in bricks.iter().enumerate() {
        brick.configure_space(&mut space, index as u16 +1);
    }

    drop_bricks(&mut bricks, &mut space, 0);

    // Bricks above.. do they have other bricks below?
    let not_safe: Vec<usize> = bricks.iter()
        .enumerate()
        .filter_map(|(index,brick)| {
            let above = brick.above(&space);            
            if above.len() == 0 || above.iter().all(|a| bricks[*a as usize - 1].below(&space).len() > 1) {
                None
            } else {
                Some(index)
            }
        })
        .collect();
    let p1 = bricks.len() - not_safe.len();

    let p2: usize = not_safe.into_iter()
        .map(|index| -> usize {

            let mut b0 = bricks.clone();
            let mut s2 = space.clone();

            b0[index].configure_space(&mut s2, 0);
            b0[index].start.2 = 1; // Faster than removing if from the list.

            drop_bricks(&mut b0, &mut s2, index)
        })
        .sum();
        
    Solution::new(p1, p2)
}

impl FromStr for Brick {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p1, p2)) = s.split_once("~") {

            let start = p1.split(',').take(3).map(|s| s.parse::<u16>().unwrap()).collect_tuple();
            let end = p2.split(',').take(3).map(|s| s.parse::<u16>().unwrap()).collect_tuple();
            
            match (start, end) {
                (Some(start),Some(end)) => Ok( Brick::initial(start, end) ),
                _ => Err("Could not parse dimensions."),
            }
        } else {
            Err("Malformed record")
        }
    }
}