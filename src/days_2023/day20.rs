// https://adventofcode.com/2023/day/20

use std::collections::{HashMap, VecDeque};

use crate::common::Solution;

const P1_BUTTON_PRESSES: usize = 1000;

#[derive(PartialEq, Eq,Clone, Debug,Copy)]
enum Kind {
    FlipFlop(bool),
    Conjunction(u64,u32),
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    kind: Kind,
    destination_modules: Vec<usize>,
}

pub fn solve(input: &str) -> Solution {
    let mut names: HashMap<&str, usize> = input.lines()
        .enumerate()
        .map(|(index,line)| {
            let (name, _) = line.split_once(" -> ").unwrap() ;
            (match name.as_bytes()[0] {
                b'%' | b'&' => &name[1..],
                _ => name
            }, index)})
        .collect();

    names.insert("rx", usize::MAX);

    let mut modules: Vec<Module> = input.lines()
        .map(|line| {
            let (name, module_list) = line.split_once(" -> ").unwrap() ;
            let modules: Vec<usize> = module_list.split(", ").map(|s| names[s]).collect();
            match line.as_bytes()[0] {
                b'%' => {
                    Module { kind: Kind::FlipFlop(false),
                        destination_modules: modules }
                },
                b'&' => {
                    let name = &name[1..];
                    let count = input
                        .lines()
                        .filter(|line| {
                            let (_, other_modules) = line.split_once(" -> ").unwrap();
                            return other_modules.contains(name)})
                        .count() as u32;
                    let kind = Kind::Conjunction(0, count);
                    Module { kind, destination_modules: modules }
                },
            _ => Module { kind: Kind::Broadcaster, destination_modules: modules },            
            }})
        .collect();

    let mut num_low = 0;
    let mut num_high = 0;

    for _button_push in 0..P1_BUTTON_PRESSES {
        let mut v: VecDeque<(bool, usize, usize)> = VecDeque::new();
        v.push_front((false, names["broadcaster"], 0));
        num_low += 1;

        while let Some((signal, module_index, source_index)) = v.pop_front() {
            match modules.get_mut(module_index) {
                Some(module) => {
                    let pulse_to_send: Option<bool> = match module.kind {
                        Kind::Broadcaster => Some(signal),
                        Kind::FlipFlop(mut state) => {
                            if signal == false {
                                state = !state;
                                module.kind = Kind::FlipFlop(state);
                                Some(state)
                            } else {
                                None
                            }},
                        Kind::Conjunction(mut inputs, num_inputs) => {
                            if signal {
                                inputs |= 1 << source_index;
                            } else {
                                inputs &= u64::MAX ^ (1 << source_index);
                            }
                            let all_inputs_high = inputs.count_ones() == num_inputs;
                            module.kind = Kind::Conjunction(inputs, num_inputs);
                            Some(!all_inputs_high)
                        }
                    };
                    if let Some(pulse_to_send) = pulse_to_send {
                        if pulse_to_send { 
                            num_high += module.destination_modules.len() 
                        } else { 
                            num_low += module.destination_modules.len()
                        }
                        v.extend(module.destination_modules.iter().map(|n| (pulse_to_send, *n, module_index) ));
                    }
                },
                None => { },
            }
        }
    }
    
    let p1 = num_high * num_low;

    let outputs_to_rx = modules.iter().position(|m| m.destination_modules[0] == usize::MAX).unwrap();
    let outputs_to_that_node: usize = modules
        .iter()
        .filter(|module| module.destination_modules.contains(&outputs_to_rx))
        .count();
    let mut deltas: Vec<usize> = vec![];

    let mut button_press = P1_BUTTON_PRESSES;
    loop {
        let mut v: VecDeque<(bool, usize, usize)> = VecDeque::new();
        v.push_front((false, names["broadcaster"], 0));
        button_press += 1;

        while let Some((signal, module_index, source_index)) = v.pop_front() {
            if signal == true && module_index == outputs_to_rx {
                // Depends on one cycle not being more than 2x other cycles.
                deltas.push(button_press);
            }
            match modules.get_mut(module_index) {
                Some(module) => {
                    let pulse_to_send: Option<bool> = match module.kind {
                        Kind::Broadcaster => Some(signal),
                        Kind::FlipFlop(mut state) => {
                            if signal == false {
                                state = !state;
                                module.kind = Kind::FlipFlop(state);
                                Some(state)
                            } else {
                                None
                            }},
                        Kind::Conjunction(mut inputs, num_inputs) => {
                            if signal {
                                inputs |= 1 << source_index;
                            } else {
                                inputs &= u64::MAX ^ (1 << source_index);
                            }
                            let all_inputs_high = inputs.count_ones() == num_inputs;
                            module.kind = Kind::Conjunction(inputs, num_inputs);
                            Some(!all_inputs_high)
                        }
                    };
                    if let Some(pulse_to_send) = pulse_to_send {
                        v.extend(module.destination_modules.iter().map(|n| (pulse_to_send, *n, module_index) ));
                    }
                },
                None => { },
            }
        }

        if deltas.len() == outputs_to_that_node {
            break;
        }
    }

    let p2: usize = deltas.into_iter().product();

    Solution::new(p1, p2)
}
