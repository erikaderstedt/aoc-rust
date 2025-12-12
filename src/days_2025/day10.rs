// https://adventofcode.com/2025/day/10

use crate::common::{Solution, parsed_from_each_line};
use std::str::FromStr;
// // use pathfinding::prelude::bfs;
use std::collections::VecDeque;
// use std::usize;

// ny metod:
// pathfinding:
// titta på 3,5,4,7 (joltages)
// se att de är jämna eller udda
// vilka knappar behövs tryckas för att nå detta state
// "tryck" på dem
// vad är kvar? ett helt jämnt
// En ny branch för varje statedetta branchar)
// Exempel, rad 127 från input:


// fn sort_by_leading_zeros(v: Vec<Vec<i32>>, q: Vec<i32>) -> (Vec<Vec<i32>>, Vec<i32>) {
//     let sorted_indices: Vec<usize> = (0..v.len()).sorted_by_key(|i| v[*i].iter().take_while(|x| **x == 0).count() ).collect();

//     let mut u = v.clone();
//     let mut t = q.clone();

//     for (j, i) in sorted_indices.into_iter().enumerate() {
//         u[j] = v[i].clone();
//         t[j] = q[i];
//     }
//     (u, t)
// }

// fn gauss_elimination(v: Vec<Vec<i32>>, q: Vec<i32>) -> (Vec<Vec<i32>>, Vec<i32>) {
//     let (mut u, mut t) = sort_by_leading_zeros(v, q);

//     for c in 0..(u[0].len().min(u.len())) {
//     // for c in 0..1 {
//         if u[c][c] < 0 {
//             u[c] = u[c].iter().map(|x| -x).collect();
//             t[c] = -t[c];
//         }
//         let x0 = u[c][c];        
//         if x0 == 0 { continue; }
        
//         // All rows below c which are non-zero in c need to be handled.
//         for r in (c+1)..u.len() {
//             let x1 = u[r][c];
//             let factor = if x1.rem_euclid(x0) != 0 { x0 } else { 1 };
//             u[r] = u[r].iter().zip(u[c].iter()).map(|(v1, v0)| factor * v1 - factor * x1 / x0 * v0).collect();
//             t[r] = factor*t[r] - factor * x1 / x0 * t[c];
//         }

//         // Sort again by leading zeros
//         (u, t) = sort_by_leading_zeros(u, t)
//     }

//     // Remove rows that are all zeros (check that t is also zero for that index)
//     let all_zeros: Vec<usize> = u.iter().enumerate().filter(|(_, row)| row.iter().all(|v| *v == 0)).map(|(i, _)| i).collect();
//     let mut num_removed = 0;
//     for zero_index in all_zeros.into_iter() {
//         assert!(t[zero_index - num_removed] == 0);
//         u.remove(zero_index - num_removed);
//         t.remove(zero_index - num_removed);
//         num_removed = num_removed + 1;
//     }

//     (u,t)
// }

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

    // fn button_press_matrices(&self) -> (Vec<Vec<i32>>, Vec<i32>) {
    //     let m = self.joltages.len();
    //     let n = self.buttons.len();
    //     let mut v = vec![vec![0i32;n];m];

    //     for (i, button) in self.buttons.iter().enumerate() {
    //         for joltage in 0..m {
    //             if button & ((1 << joltage) as u64) > 0 {
    //                 v[joltage][i] = 1;
    //             }
    //         }
    //     }
    //     (v, self.joltages.iter().map(|j| *j as i32).collect())
    // }


    fn lights(&self) -> Vec<Vec<Vec<Button>>> {

        // Pre-calculate every combination of lights
        // To get pattern X, we need to apply buttons (A,B,C) or (B,C,D) or ..
        (0..(1 << self.joltages.len()))
            .map(|pattern: u64| 
            (0..(1<<self.buttons.len())).filter_map(|button_mask: usize| {
                let buttons: Vec<Button> = 
                    self.buttons.iter()
                    .enumerate()
                    .filter(|(j, _)| ((1 << j) & button_mask) > 0)
                    .map(|(_,b)| b)
                    .cloned()
                    .collect();
                // Buttons to activate
                if buttons.iter().fold(0u64, |acc, button| acc ^ button.mask) == pattern {
                    Some(buttons)
                } else {
                    None
                }
            }) // Sets of buttons that match this mattern
            .collect())
            .collect()
    }

    fn minimum_required_button_presses_pt2(&self) -> u32 {
        let lights = self.lights();

        // TODO: lägg på gaussian solver först
        // om vi redan vet att en viss knapp ska tryckas på X ggr, dra bort detta från joltage requirements och ta bort knappen
        // från lights.

        // println!("{:?}", self);
        // println!("{} {}", self.buttons.len(), self.joltages.len());
    
        // // vi behöver inte veta n, bara 
        let start = self.joltages.clone();
        let mut q: VecDeque<(Vec<i32>,Vec<u32>)> = vec![(start.clone(),vec![])].into();
        // let mut explored = vec![start];
        let mut results = vec![];
        while let Some((v,result)) = q.pop_front() {
            if v.iter().all(|&x| x == 0 ) {
                // [###.] (1,3) (0,1,2) {10,12,10,2}
                // Alla jämna, dela med två:
                // 5,6,5,1
                // (0,1,2) + (1,3) => 4,4,4,0
                // alla jämna, dela med två
                // 2,2,2,0
                // alla jämna, dela med två
                // 1,1,1,0
                // (0,1,2)
                // 2*(2+2*2*1) = 12 (vilket är svaret) 
                // (1,3) (0,1,2) {10,12,10,2}
                // 0, {5,6,5,1}
                // 2, {2,2,2,0}
                // 0, {1,1,1,0}
                // 1, {0,0,0,0}
                // 0 + 2*(2+ 2* (0 +2*(1)))

                let r = result.iter().rev().fold(0, |acc, value| value + 2*acc);
                results.push(r);
            } else {
                // Calculate the even odd value
                let s = v.iter().enumerate().fold(0, |acc, (i, x)| acc + ((x & 1) << (i as u64)));

                for possible_buttons in lights[s as usize].iter() {

                    let mut j = v.clone();
                    // Apply these buttons to the state (subtract joltage contributions)
                    for button in possible_buttons.iter() {
                        for affects in button.affects.iter() {
                            j[*affects] = j[*affects] - 1;
                        }
                    }

                    // Are any joltage values negative? If so, skip this possible_buttons
                    if j.iter().any(|&x| x < 0) {
                        continue;
                    }
                    // Remaining joltage values should all be even.
                    assert!(j.iter().all(|&x| (x & 1) == 0));

                    // Cut them in half
                    for x in j.iter_mut() {
                        *x = *x >> 1;
                    }
                    let mut nr = result.clone();
                    nr.push(possible_buttons.len() as u32);
                    q.push_back((j, nr));
                }
            }
        }
        results.into_iter().min().unwrap_or(u32::MAX)
    }


            //     // F
            //     for b in self.buttons.iter() {
            //         let mut n = v.clone();
            //         let mut exceeded = false;
            //         for (j, x) in n.iter_mut().enumerate() {
            //             if (1 << (j as u64)) & b > 0 {
            //                 *x = *x + 1;
            //                 if *x > self.joltages[j] {
            //                     exceeded = true;
            //                 }
            //             }

            //         }
            //         if !exceeded && !explored.contains(&n) {
            //             println!("{:?} {:?} {}", v, n, i);
            //             explored.push(n.clone());
            //             q.push_back((n, i + 1));
            //         }
            //     }
            // }
        // }
        // panic!("afdadfskladf");
//         let p = bfs(&0u64, 
// |state: &u64| {
//         let v: Vec<u64> = self.buttons.iter().map(|&b| state ^ b).collect();
//         println!("state: {} v: {:?}", state, v);
//         v },
//         |state: &u64| *state == self.target)
//         .unwrap();

//         p.len()


    // fn press_remaining_buttons_for_joltage(&self, remaining_buttons: Vec<usize>, m: Vec<Vec<i32>>, n: Vec<i32>) -> usize {

    //     // This is at most three in my input.
    //     let free_variables = u[0].len() - u.len();



    //     // Pick the first "free_variables" columns as free variables
    //     // Construct a search space for free variables

    //     // For each point in the search space:
    //     // - create a copy of m, n
    //     // - apply 
              
    //     // Search the space 
    //     // Conditions for each remaining button
    //     // For example, a difficult one:
    //     // [[1, 0, 1, 1, 1],    22
    //     //  [0, 1, 0, 0, 1],    27
    //     //  [0, 0, 1, 1, 0],    8
    //     //  [0, 0, 0, 1, -1]]   -7
    //     //
    //     // ta en rad med bara >= 0
    //     // sätt en av knapparna till något
    //     // lös matrisen - få fram totala antalet 


    //     // om b1 = 26:
    //     // [13,26, 0,8, 1]
    //     // om b1 = 27
    //     // [12, 27, 1,7, 0]
    //     // b4: 1
    //     // b3: 


    // }

    // fn maximum_required_button_presses(&self, columns: &Vec<usize>) -> Vec<Vec<i32>> {
    //     columns.iter().map(|button_index| {
    //         let button = self.buttons[*button_index];
    //         // What joltages does this button affect?
    //         // What is the minimum value of these joltages?
    //         let maximum_number_of_presses = self.joltages.iter()
    //             .enumerate()
    //             .filter(|(index, _)| (1 << *index) & (button as usize) > 0)
    //             .map(|(_, joltage)| *joltage)
    //             .min().unwrap_or(0);

    //         (0..=maximum_number_of_presses).map(|v| v).collect()
    //     })
    //     .collect()
    // }

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
    let p2 = machines.iter().map(|m| m.minimum_required_button_presses_pt2()).sum::<u32>();
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
