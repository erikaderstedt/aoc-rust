// https://adventofcode.com/2023/day/22

use std::{str::FromStr, collections::HashSet};

use itertools::Itertools;

use crate::common::{Solution, parsed_from_each_line};

#[derive(Clone)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    
    fn initial(start: (usize, usize, usize), end: (usize, usize, usize)) -> Brick {
        Brick { start, end }
    }

    fn above(&self, space: &Vec<Vec<Vec<usize>>>) -> HashSet<usize> {
        let mut above: HashSet<usize> = HashSet::new();
        let z = self.end.2;

        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                let v = space[x][y][z+1];
                if v != 0 {
                    above.insert(v);
                }
            }
        }

        above
    }

    fn below(&self, space: &Vec<Vec<Vec<usize>>>) -> HashSet<usize> {
        let mut below: HashSet<usize> = HashSet::new();
        let z = self.start.2;

        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                let v = space[x][y][z-1];
                if v != 0 {
                    below.insert(v);
                }
            }
        }

        below
    }
}

fn drop_bricks(bricks: &mut Vec<Brick>, space: &mut Vec<Vec<Vec<usize>>>) -> usize {
    let mut total_moved: HashSet<usize> = HashSet::new();
    loop {
        let mut moved = 0;
        for (index,brick) in bricks.iter_mut().enumerate() {
            let z = brick.start.2.min(brick.end.2);
            if z == 1 { continue; }

            if let Some(new_z) = 
                (1..z).rev()
                    .take_while(|z|
                        (brick.start.0..=brick.end.0)
                            .all(|x| (brick.start.1..=brick.end.1)
                                .all(|y| space[x][y][*z] == 0)))
                    .last() {
                        
                    for x in brick.start.0..=brick.end.0 {
                        for y in brick.start.1..=brick.end.1 {
                            for z in brick.start.2..=brick.end.2 {
                                space[x][y][z] = 0;
                            }
                        }
                    }

                    brick.end.2 -= z - new_z;
                    brick.start.2 = new_z;

                    for x in brick.start.0..=brick.end.0 {
                        for y in brick.start.1..=brick.end.1 {
                            for z in brick.start.2..=brick.end.2 {
                                space[x][y][z] = index + 1;
                            }
                        }
                    }
                    total_moved.insert(index);
                    moved += 1;

            }
        }
        if moved == 0 { break; }
    }
    total_moved.len()
}

pub fn solve(input: &str) -> Solution {
    let mut bricks: Vec<Brick> = parsed_from_each_line(input);

    let mut space = bricks.iter()
        .enumerate()
        .fold(
        vec![vec![vec![0; 300]; 10]; 10],
        |mut space, (index,brick)| {
            for x in brick.start.0..=brick.end.0 {
                for y in brick.start.1..=brick.end.1 {
                    for z in brick.start.2..=brick.end.2 {
                        space[x][y][z] = index + 1;
                    }
                }
            }
            space
        }
    );

    drop_bricks(&mut bricks, &mut space);

    // Bricks above.. do they have other bricks below?
    let p1 = bricks.iter()
        .filter(|brick| {
            let above = brick.above(&space);            
            above.len() == 0 || above.iter().all(|a| bricks[*a - 1].below(&space).len() > 1)
        })
        .count();
    
    let p2: usize = bricks.iter().enumerate()
        .map(|(index, brick)| -> usize {

            let mut b0 = bricks.clone();
            let mut s2 = space.clone();

            for x in brick.start.0..=brick.end.0 {
                for y in brick.start.1..=brick.end.1 {
                    for z in brick.start.2..=brick.end.2 {
                        s2[x][y][z] = 0;
                    }
                }
            }
            b0.remove(index);

            drop_bricks(&mut b0, &mut s2)
        })
        .sum();
        
    Solution::new(p1, p2)
}

impl FromStr for Brick {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p1, p2)) = s.split_once("~") {

            let start = p1.split(',').take(3).map(|s| s.parse::<usize>().unwrap()).collect_tuple();
            let end = p2.split(',').take(3).map(|s| s.parse::<usize>().unwrap()).collect_tuple();
            
            match (start, end) {
                (Some(start),Some(end)) => Ok( Brick::initial(start, end) ),
                _ => Err("Could not parse dimensions."),
            }
        } else {
            Err("Malformed record")
        }
    }
}