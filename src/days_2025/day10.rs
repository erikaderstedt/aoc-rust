// https://adventofcode.com/2025/day/10

use itertools::Itertools;
use crate::common::{Solution, parsed_from_each_line};
use std::str::FromStr;

fn sort_by_leading_zeros(v: Vec<Vec<i32>>, q: Vec<i32>) -> (Vec<Vec<i32>>, Vec<i32>) {
    let sorted_indices: Vec<usize> = (0..v.len()).sorted_by_key(|i| v[*i].iter().take_while(|x| **x == 0).count() ).collect();

    let mut u = v.clone();
    let mut t = q.clone();

    for (j, i) in sorted_indices.into_iter().enumerate() {
        u[j] = v[i].clone();
        t[j] = q[i];
    }
    (u, t)
}

// Gauss eliminator. Also tries to put the matrix in row echelon form as much as possible.
// This function also rearranges columns (not normally ok, but we don't care which button is
// which at this point.)
fn gauss_elimination(v: Vec<Vec<i32>>, q: Vec<i32>) -> (Vec<Vec<i32>>, Vec<i32>) {
    let (mut u, mut t) = sort_by_leading_zeros(v, q);

    for c in 0..(u[0].len().min(u.len())) {
        if u[c][c] < 0 {
            u[c] = u[c].iter().map(|x| -x).collect();
            t[c] = -t[c];
        }
      
        if u[c][c] == 0 { 
            // Find first non-zero element in this row.
            if let Some((i,_)) = u[c].iter().enumerate().skip_while(|(_,v)| **v == 0).next() {
                // Swap columns i and c, for all rows.
                // We no longer care which button is which
                for r2 in 0..u.len() {
                    let s = u[r2][i];
                    u[r2][i] = u[r2][c];
                    u[r2][c] = s;
                }
            } else {
                continue;
            }

        }
        let x0 = u[c][c];
        
        // All rows below c which are non-zero in c need to be handled.
        for r in (c+1)..u.len() {
            let x1 = u[r][c];
            let factor = if x1.rem_euclid(x0) != 0 { x0 } else { 1 };
            u[r] = u[r].iter().zip(u[c].iter()).map(|(v1, v0)| factor * v1 - factor * x1 / x0 * v0).collect();
            t[r] = factor*t[r] - factor * x1 / x0 * t[c];
        }

        // Sort again by leading zeros
        (u, t) = sort_by_leading_zeros(u, t)
    }

    // Remove rows that are all zeros (check that t is also zero for that index)
    let all_zeros: Vec<usize> = u.iter().enumerate().filter(|(_, row)| row.iter().all(|v| *v == 0)).map(|(i, _)| i).collect();
    let mut num_removed = 0;
    for zero_index in all_zeros.into_iter() {
        assert!(t[zero_index - num_removed] == 0);
        u.remove(zero_index - num_removed);
        t.remove(zero_index - num_removed);
        num_removed = num_removed + 1;
    }

    // Put matrix into row echelon form as far as possible.
    for r in 0..(u.len()-1) {
        assert!(u[r][r] != 0, "m = np.array({:?});j=np.array({:?})", u, t); // Diagonal element must not be zero
        for c in (r+1)..u[0].len().min(u.len()) {
            if u[r][c] != 0 {
                let x0 = u[r][c];
                let x1 = u[c][c];
                if x1 != 0 {
                    let factor = if x0.rem_euclid(x1) != 0 {
                        // Multiply whole u[r] row by x1
                        for v in u[r].iter_mut() {
                            *v = *v * x1;
                        }
                        t[r] = t[r] * x1;
                        x1
                    } else {
                        1
                    };

                    for c2 in c..u[0].len() {
                        u[r][c2] = u[r][c2] - factor * x0 / x1 * u[c][c2];
                    }
                    t[r] = t[r] - factor * x0 / x1 * t[c];
                }
            }
        }
    }

    // Ensure diagonal is positive
    for c in 0..u.len() {
        if u[c][c] < 0 {
            u[c] = u[c].iter().map(|x| -x).collect();
            t[c] = -t[c];
        }
    } 

    (u,t)
}

#[derive(Debug,Clone)]
struct Button {
    mask: u64,
    affects: Vec<usize>,
}

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<Button>,
    joltages: Vec<i32>,
}

pub fn solve(input: &str) -> Solution {
    let machines: Vec<Machine> = parsed_from_each_line(input);

    let p1 = machines.iter().map(|m| m.minimum_required_button_presses()).sum::<usize>();
    let p2 = machines.iter().map(|m| m.gauss_elimination_solve()).sum::<i32>();

    Solution::new(p1, p2)
}

impl Machine {

    fn apply<const N:usize>(u: &Vec<Vec<i32>>, t: &Vec<i32>, free: [i32;N]) -> Option<i32> {
        let lc = u[0].len()-1;
        let mut total = 0;
        for r in 0..u.len() {
            let f = t[r] - match N {
                1 => u[r][lc]*free[0],
                2 => u[r][lc-1]*free[0] + u[r][lc]*free[1],
                3 => u[r][lc-2]*free[0] + u[r][lc-1]*free[1] + u[r][lc]*free[2],
                _ => panic!("not implemented")
            };            
            if f >= 0 && f.rem_euclid(u[r][r]) == 0 {
                total = total + f / u[r][r]
            } else {
                return None;
            }
        }
        Some(total)
    }

    fn solve_free_variables(&self, u: &Vec<Vec<i32>>, t: &Vec<i32>, num_free: usize) -> i32 {
        let mut m = vec![self.joltages.iter().max().unwrap().clone();num_free];
        let w = u[0].len();
        for row in 0..u.len() {
            if u[row].iter().skip(w - num_free).all(|v| *v >= 0) && t[row] > 0 {
                for (i, value) in u[row].iter().skip(w - num_free).enumerate() {
                    if *value > 0 {
                        m[i] = m[i].min(t[row] / value)
                    }
                }
            }
        }

        match num_free {
            1 => (0..=m[0]).filter_map(|value| Machine::apply(u, t, [value]).map(|v| v + value)).min().unwrap(),
            2 => (0..=m[0]).filter_map(|v1| (0..=m[1]).filter_map(|v2| Machine::apply(u, t, [v1,v2]).map(|v| v + v1 + v2)).min()).min().unwrap(),
            3 => (0..=m[0]).filter_map(|v1| (0..=m[1]).filter_map(|v2| (0..=m[2]).filter_map(|v3| Machine::apply(u, t, [v1,v2,v3]).map(|v| v + v1 + v2 + v3)).min()).min()).min().unwrap(),
            _ => panic!("not supported"),
        }
    }

    // This is the equation system to solve.
    fn button_press_matrices(&self) -> (Vec<Vec<i32>>, Vec<i32>) {
        let m = self.joltages.len();
        let n = self.buttons.len();
        let mut v = vec![vec![0i32;n];m];

        for (i, button) in self.buttons.iter().enumerate() {
            for &joltage in button.affects.iter() {
                v[joltage][i] = 1;
            }
        }
        (v, self.joltages.iter().map(|j| *j as i32).collect())
    }

    fn gauss_elimination_solve(&self) -> i32 {
        let (u,v) = self.button_press_matrices();
        let (mut u, mut v) = gauss_elimination(u, v);

        let mut presses = vec![];
        let mut remaining_columns: Vec<usize> = (0..self.buttons.len()).collect();
        let mut changes_made = true;
        while changes_made {
            // Find any row with only one non-zero value
            // if so, calculate that value, remove the row and adjust u,v for all other rows
            if let Some(r) = u.iter().position(|row| row.iter().filter(|x| **x != 0).count() == 1) {
                let c = u[r].iter().position(|x| *x != 0).unwrap();
                let n = v[r];
                let d = u[r][c];
                assert!(n.rem_euclid(d) == 0);
                let value = n / d;

                presses.push(value);

                // Remove this row
                u.remove(r);
                v.remove(r);

                // Remove this column
                for (r, row) in u.iter_mut().enumerate() {
                    if row[c] != 0 {
                        v[r] = v[r] - row[c] * value;
                    }
                    row.remove(c);
                }
                remaining_columns.remove(c);
                changes_made = true;
            } else {
                // Remove rows that are all zeros (check that t is also zero for that index)
                let all_zeros: Vec<usize> = u.iter().enumerate().filter(|(_, row)| row.iter().all(|v| *v == 0)).map(|(i, _)| i).collect();
                let mut num_removed = 0;
                for zero_index in all_zeros.into_iter() {
                    assert!(v[zero_index - num_removed] == 0);
                    u.remove(zero_index - num_removed);
                    v.remove(zero_index - num_removed);
                    num_removed = num_removed + 1;
                    
                }
                changes_made = num_removed > 0;
            }
        }
        let already_determined = presses.into_iter().sum::<i32>();
        if remaining_columns.len() == 0 {
            already_determined
        } else {
            let n_free_variables = u[u.len()-1].iter().skip_while(|v| **v == 0).count() - 1;
            self.solve_free_variables(&u, &v, n_free_variables) + already_determined
        }
    }

    fn minimum_required_button_presses(&self) -> usize {

        // No button will be pressed more than once.
        // This means that there are 2^N different states to enumerate
        // where N is the number of buttons.
        // 0 - not pressed, 1 - pressed
        // Just enumerate these, find which have the correct output
        // and minimize by popcount

        let n = self.buttons.len();
        (1..(1<<n))
            .filter_map(|i: usize|  if self.buttons
                .iter()
                .enumerate()
                .filter(|(j, _)| ((1 << j) & i) > 0)
                .fold(0u64, |acc, (_, b)| acc ^ b.mask) == self.target {
                    Some(i.count_ones() as usize)
                } else {
                    None
                }

        )
        .min()
        .unwrap()
    }

    // This is a neat algorithm. Idea off Reddit. Takes around 16 seconds to run. I used
    // this to get my star, leaving it in for posterity.
    
    // fn pathfinding_solve(&self) -> Option<u32> {
    //     let start = self.joltages.clone();
    //     // Pre-calculate every combination of lights
    //     // To get pattern X, we need to apply buttons (A,B,C) or (B,C,D) or ..
    //     let lights: Vec<Vec<Vec<Button>>> = (0..(1 << self.joltages.len()))
    //         .map(|pattern: u64| 
    //         (0..(1<<self.buttons.len())).filter_map(|button_mask: usize| {
    //             let buttons: Vec<Button> = 
    //                 self.buttons.iter()
    //                 .enumerate()
    //                 .filter(|(j, _)| ((1 << j) & button_mask) > 0)
    //                 .map(|(_,b)| b)
    //                 .cloned()
    //                 .collect();
    //             // Buttons to activate
    //             if buttons.iter().fold(0u64, |acc, button| acc ^ button.mask) == pattern {
    //                 Some(buttons)
    //             } else {
    //                 None
    //             }
    //         }) // Sets of buttons that match this mattern
    //         .collect())
    //         .collect();

    //     let mut q: Vec<(Vec<i32>,Vec<u32>)> = vec![(start.clone(),vec![])].into();
    //     let mut results = vec![];
    //     let mut ma = 0;
    //     while let Some((v,result)) = q.pop() {
    //         if v.iter().all(|&x| x == 0 ) {

    //             let r = result.iter().rev().fold(0, |acc, value| value + 2*acc);
    //             results.push(r);                
    //         } else {
    //             // Calculate the even odd value
    //             let s = v.iter().enumerate().fold(0, |acc, (i, x)| acc + ((x & 1) << (i as u64)));

    //             for possible_buttons in lights[s as usize].iter() {
    //                 // This is a set of possible_buttons that will result in the state 's'.
    //                 // Apply these buttons to the state (subtract joltage contributions)
    //                 let mut j = v.clone();
    //                 for button in possible_buttons.iter() {
    //                     for affects in button.affects.iter() {
    //                         j[*affects] = j[*affects] - 1;
    //                     }
    //                 }

    //                 // Are any joltage values negative? If so, skip this possible_buttons
    //                 if j.iter().any(|&x| x < 0) {
    //                     continue;
    //                 }

    //                 // Cut them in half
    //                 for x in j.iter_mut() {
    //                     *x = *x >> 1;
    //                 }

    //                 let mut nr = result.clone();
    //                 nr.push(possible_buttons.len() as u32);
    //                 q.push((j, nr));
    //                 if q.len() > ma {
    //                     ma = q.len();
    //                 }
    //             }
    //         }
    //     }
    //     results.into_iter().min()
    // }

}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<&str> = s.split(' ').collect();
        let target = l[0]
            .replace("[", "")
            .replace("]", "")
            .chars()
            .map(|u| if u == '#' { 1u64 } else { 0u64 } )
            .enumerate()
            .fold(0, |acc, (index, v)| acc + (v << (index as u64)));
        let joltages = l[l.len()-1]
            .replace("{", "")
            .replace("}", "")
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let buttons = l[1..(l.len()-1)].iter()
            .map(|s| {
                let n: Vec<usize> = s
                    .replace("(", "")
                    .replace(")", "")
                    .split(',')
                    .map(|q| q.parse::<usize>().unwrap())
                    .collect();
                let b = n.iter().fold(0u64, |acc, value| acc + (1 << value));
                Button { mask: b, affects: n }
            })
            .collect();
        Ok( Machine { target, buttons, joltages })
    }
}
