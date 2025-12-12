// https://adventofcode.com/2025/day/10

use itertools::Itertools;
use crate::common::{Solution, parsed_from_each_line};
use std::str::FromStr;
// use rayon::prelude::*;


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

    // Reduced form if possible
    // Ideally we would need to find the GCD of all non-zero items in the row and in t.
    // We settle for just removing 2,3,5
    for r in 0..u.len() {
        for p in [2,3,5].iter() {
            if u[r].iter().all(|v| v.rem_euclid(*p) == 0) && t[r].rem_euclid(*p) == 0 {
                for x in u[r].iter_mut() {
                    *x = *x / p;
                }
                t[r] = t[r] / p
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

impl Machine {

    fn apply<const N:usize>(u: &Vec<Vec<i32>>, t: &Vec<i32>, free: [i32;N]) -> Option<i32> {
        let lc = u[0].len()-1;
        // TODO: early abort as soon as there is a
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

    fn solve_with_one_free_variable(&self, u: &Vec<Vec<i32>>, t: &Vec<i32>) -> i32 {
        let m = self.joltages.iter().max().unwrap();
        let lc = u[0].len() - 1;
        // Look in last column. Is there any positive
        let largest = (0..u.len()).filter_map(|row_index|  {
            let value = u[row_index][lc];
            if value > 0 && t[row_index] > 0 {
                Some(t[row_index] / value)
            } else {
                None
            }
        }).max().unwrap_or(*m);

        (0..=largest).filter_map(|value| Machine::apply(u, t, [value]).map(|v| v + value)).min().unwrap()
    }

    fn solve_with_two_free_variables(&self, u: &Vec<Vec<i32>>, t: &Vec<i32>) -> i32 {
        let m = self.joltages.iter().max().unwrap().clone();
        // let lc = u[0].len() - 1;
        // // Look in last two column. Each row with no negatives will give conditions Is there any positive
        // let largest = (0..u.len()).filter_map(|row_index|  {
        //     let value = u[row_index][lc];
        //     if value > 0 && t[row_index] > 0 {
        //         Some(t[row_index] / value)
        //     } else {
        //         None
        //     }
        // }).max().unwrap_or(m);

        (0..=m).filter_map(|v1| (0..m).filter_map(|v2| Machine::apply(u, t, [v1,v2]).map(|v| v + v1 + v2)).min()).min().unwrap()
    }

    fn solve_with_three_free_variables(&self, u: &Vec<Vec<i32>>, t: &Vec<i32>) -> i32 {
        let m = self.joltages.iter().max().unwrap().clone();
        // let lc = u[0].len() - 1;
        // // Look in last two column. Each row with no negatives will give conditions Is there any positive
        // let largest = (0..u.len()).filter_map(|row_index|  {
        //     let value = u[row_index][lc];
        //     if value > 0 && t[row_index] > 0 {
        //         Some(t[row_index] / value)
        //     } else {
        //         None
        //     }
        // }).max().unwrap_or(m);

        // TODO: increase each from 0, until we start getting values. When the value starts to increase, we stop?
        (0..=m).filter_map(|v1| (0..m).filter_map(|v2| (0..m).filter_map(|v3| Machine::apply(u, t, [v1,v2,v3]).map(|v| v + v1 + v2 + v3)).min()).min()).min().unwrap()
    }

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

    // fn lights(&self) -> Vec<Vec<Vec<Button>>> {

    //     // Pre-calculate every combination of lights
    //     // To get pattern X, we need to apply buttons (A,B,C) or (B,C,D) or ..
    //     (0..(1 << self.joltages.len()))
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
    //         .collect()
    // }

    // fn pathfinding_solve(&self) -> Option<u32> {
    //     let start = self.joltages.clone();
    //     let lights = self.lights();

    //     let mut q: Vec<(Vec<i32>,Vec<u32>)> = vec![(start.clone(),vec![])].into();

    //     let mut results = vec![];
    //     let mut ma = 0;
    //     while let Some((v,result)) = q.pop() {
    //         if v.iter().all(|&x| x == 0 ) {
    //             // [###.] (1,3) (0,1,2) {10,12,10,2}
    //             // Alla jämna, dela med två:
    //             // 5,6,5,1
    //             // (0,1,2) + (1,3) => 4,4,4,0
    //             // alla jämna, dela med två
    //             // 2,2,2,0
    //             // alla jämna, dela med två
    //             // 1,1,1,0
    //             // (0,1,2)
    //             // 2*(2+2*2*1) = 12 (vilket är svaret) 
    //             // (1,3) (0,1,2) {10,12,10,2}
    //             // 0, {5,6,5,1}
    //             // 2, {2,2,2,0}
    //             // 0, {1,1,1,0}
    //             // 1, {0,0,0,0}
    //             // 0 + 2*(2+ 2* (0 +2*(1)))

    //             let r = result.iter().rev().fold(0, |acc, value| value + 2*acc);
    //             results.push(r);                
    //         } else {
    //             // Calculate the even odd value
    //             let s = v.iter().enumerate().fold(0, |acc, (i, x)| acc + ((x & 1) << (i as u64)));

    //             // Remember s -> resulting new items in q.
    //             // Hmm that will depend on the remaining joltage

    //             for possible_buttons in lights[s as usize].iter() {

    //                 let mut j = v.clone();
    //                 // Apply these buttons to the state (subtract joltage contributions)
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

    fn gauss_elimination_solve(&self) -> Option<i32> {
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
            // println!("{}", already_determined);
            Some(already_determined)
        } else {
            let n_free_variables = u[u.len()-1].iter().skip_while(|v| **v == 0).count() - 1;

            match n_free_variables {
                1 => {
                    let a = self.solve_with_one_free_variable(&u, &v);
                    // println!("{} = {} + {}", a + already_determined, a, already_determined);
                    Some(a + already_determined) }, 
                2 => {
                    // println!("---- {}", already_determined);
                    let a = self.solve_with_two_free_variables(&u, &v);
                    // println!("{} = {} + {} (2 free)", a + already_determined, a, already_determined);
                    Some(a + already_determined)}, //Some(+ presses.into_iter().sum::<i32>()),
                3 => {
                    let a = self.solve_with_three_free_variables(&u, &v);
                    // println!("{} = {} + {} (3 free)", a + already_determined, a, already_determined);
                    Some(a + already_determined)
                    // println!("{} free variables",n_free_variables);
                    // let n = u[0].len();
                    // let mut j = 0;
                    // for row in u.iter() {
                    //     let last: Vec<&i32> = row.iter().skip(n-n_free_variables).collect();
                    //     if last.iter().all(|v| **v >= 0) {
                    //         // println!("{:?}", last);
                    //         j = j + 1;
                    //     }
                    // }
                    // // if there are no negative in the last three in the row, this gives an upper bound t / u[r][c]

                    // println!("a=np.array({:?});b=np.array({:?});r={}", u, v, already_determined);
                },
                _ => { panic!("Too many free variables for this implementation!") }
            }

        }
    }

    fn minimum_required_button_presses_pt2(&self) -> i32 {
        self.gauss_elimination_solve().unwrap_or(0) // or_else(|| self.pathfinding_solve()).unwrap_or(u32::MAX)
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
            // .inspect(|p| println!("{0:15b}", p))    
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

}

pub fn solve(input: &str) -> Solution {
    let machines: Vec<Machine> = parsed_from_each_line(input);

        // let m = Machine::from_str("[..#.###] (0,3,4,5,6) (1,3,4,5) (2,3) (0,2,3,5) (1,2,5,6) (0,1,2,3,5,6) (0,2,3,4,6) (1,6) {28,59,60,61,23,64,46}").unwrap();

        // let p2 = m.gauss_elimination_solve().unwrap_or(0);

    // for m in machines.iter() {
    // let m = Machine::from_str("[..##.] (0,2) (1) (4) (0,4) (1,4) (2,3) {26,8,209,200,25}").unwrap();
    // let m = Machine::from_str("[##..#...#] (0,1,2,5,6,8) (3,4,5,7,8) (2,3,4) (1,2,4,5,6,7,8) (2,5) (0,5,7) (0,4,5,6,8) {45,33,46,21,45,82,44,43,60}").unwrap();
    // let m = Machine::from_str("[###.] (1,3) (0,1,2) {10,12,10,2}").unwrap();

    // println!("{}", m.minimum_required_button_presses_pt2());
    // }
    // let (u,v) = m.button_press_matrices();
    // let (mut u, mut v) = gauss_elimination(u, v);
    
    // let mut presses = vec![];
    // let mut remaining_columns: Vec<usize> = (0..m.buttons.len()).collect();
    // loop {
    //     // Find any row with only one non-zero value
    //     // if so, calculate that value, remove the row and adjust u,v for all other rows

    //     if let Some(r) = u.iter().position(|row| row.iter().filter(|x| **x != 0).count() == 1) {
    //         let c = u[r].iter().position(|x| *x != 0).unwrap();
    //         let n = v[r];
    //         let d = u[r][c];
    //         assert!(n.rem_euclid(d) == 0);
    //         let value = n / d;
    //         // println!("r {} c {} n {} d {} value {}", r,c,n,d,value);
    //         presses.push(value);

    //         // Remove this row
    //         u.remove(r);
    //         v.remove(r);

    //         // Remove this column
    //         for (r, row) in u.iter_mut().enumerate() {
    //             if row[c] != 0 {
    //                 v[r] = v[r] - row[c] * value;
    //             }
    //             row.remove(c);
    //         }
    //         remaining_columns.remove(c);
    //     } else {
    //         // Remove rows that are all zeros (check that t is also zero for that index)
    //         let all_zeros: Vec<usize> = u.iter().enumerate().filter(|(_, row)| row.iter().all(|v| *v == 0)).map(|(i, _)| i).collect();
    //         let mut num_removed = 0;
    //         for zero_index in all_zeros.into_iter() {
    //             assert!(v[zero_index - num_removed] == 0);
    //             u.remove(zero_index - num_removed);
    //             v.remove(zero_index - num_removed);
    //             num_removed = num_removed + 1;
    //         }

    //         // Remove rows that are zero
    //         break;
    //     }
    // }
    //     if u.len() > 0 {
    //         let free = u[0].len() - u.len();
    //         let easy_rows = u.iter()
    //             .enumerate()
    //             .rev()
    //             .filter(|(index, row)| row.iter().all(|value| *value >= 0) && v[*index] >= 0).count();

    //         //     if free > easy_rows {
    //         //                 println!("m=np.array({:?});joltages=np.array({:?}) # {:?}, {:?}", u, v, presses.iter().sum::<i32>(), remaining_columns);

    //         //         // println!("WAAAAAAAAAAAAAAAAH!")
    //         //     } else {
    //         // println!("{} free, {} easy", free, easy_rows);
    //         //     }
    //                         println!("m=np.array({:?});joltages=np.array({:?}) # {:?}, {:?}", u, v, presses.iter().sum::<i32>(), remaining_columns);
                
    //     //         println!("all positive: {} {:?}", index, row);
    //     //         } else {
    //     // println!("m=np.array({:?});joltages=np.array({:?}) # {:?}, {:?}", u, v, presses.iter().sum::<i32>(), remaining_columns);

    //             // }
                
    //         // if u[0].len() == u.len() {
    //         //     println!("{:?} {:?} {:?}, {:?}", u, v, presses.iter().sum::<i32>(), remaining_columns);
            
    //         // } else {
    //         //     println!("{:?} remaining", (u[0].len() as i32) -  (u.len() as i32));
    //         // }
    //         // println!("{:?}x{:?} remaining", u.len(), u[0].len());
    //     } else {
    //         println!("done {}", presses.iter().sum::<i32>());
    //     }
    //     // println!("{:?} {:?} {:?}, {:?}", u, v, presses.iter().sum::<i32>(), remaining_columns);
    // // }
    // // 

    // // När denna kod är klar, beräkna antal möjliga för samtliga maskiner
    // // så får jag ett hum om huruvida det kommer att gå att iterera över hela spannet
    // let mut mb = m.maximum_required_button_presses(&remaining_columns);
    // // Only keep 
    // for _ in 0..10 {
    // for (row_index, row) in u.iter().enumerate() {
    //     for c in 0..row.len() {
    //         if row[c] == 0 {
    //             continue;                
    //         }
    //         // sätt övriga kolumner till sitt högsta värde
    //         //    - ger nytt lägsta värde för denna kolumn
    //         //    sätt övriga kolumner till sitt lägsta värde
    //         //    - ger nytt högsta värde för denna kolumn
    //         let max_values: Vec<i32> = mb.iter().map(|x| x.iter().max().cloned().unwrap_or(0)).collect();
    //         let min_values: Vec<i32> = mb.iter().map(|x| x.iter().min().cloned().unwrap_or(0)).collect();
    //         let new_lowest = (v[row_index] - row.iter().zip(max_values.iter()).map(|(v, m)| v * m).sum::<i32>()) / row[c];
    //         mb[c] = mb[c].iter().filter(|v| **v >= new_lowest).cloned().collect();
    //         let new_highest = (v[row_index] - row.iter().zip(min_values.iter()).map(|(v, m)| v * m).sum::<i32>()) / row[c];
    //         mb[c] = mb[c].iter().filter(|v| **v <= new_highest).cloned().collect();
            
    //         println!("c {} {} {}", c, new_lowest, new_highest);

    //     }
    // }
    // println!("{:?}", mb);
    // machines[0].minimum_required_button_presses();
    // }
    // let p1 = 0;
    let p1 = machines.iter().map(|m| m.minimum_required_button_presses()).sum::<usize>();
    let p2 = machines.iter().map(|m| m.minimum_required_button_presses_pt2()).sum::<i32>();
    // let p2 = machines.iter().map(|m| m.minimum_required_button_presses_pt2()).sum::<usize>();
    


    Solution::new(p1, p2)
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
