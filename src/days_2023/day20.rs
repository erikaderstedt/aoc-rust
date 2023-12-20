// https://adventofcode.com/2023/day/20

use std::collections::HashMap;

use crate::common::Solution;

const P1_BUTTON_PRESSES: usize = 1000;
const APPROX_SIGNALS_SENT_PER_BUTTON_PRESS: usize = 200;
const RX_INDEX: usize = usize::MAX;
const NUM_BITS: usize = 12;

#[derive(PartialEq, Eq)]
enum Kind {
    FlipFlop(bool),
    Conjunction(u64,u32),
    Broadcaster,
}

impl Kind {
    fn tranceive(&mut self, signal: bool, source: usize) -> Option<bool> {
        match self {
            Kind::Broadcaster => Some(signal),
            Kind::FlipFlop(state) => {
                if signal == false {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }},
            Kind::Conjunction(inputs, num_inputs) => {
                if signal {
                    *inputs |= 1 << source;
                } else {
                    *inputs &= u64::MAX ^ (1 << source);
                }
                let all_inputs_high = inputs.count_ones() == *num_inputs;
                Some(!all_inputs_high)
            }
        }
    }
}

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

    names.insert("rx", RX_INDEX);

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
    let broadcaster = names["broadcaster"];

    for _button_press in 0..P1_BUTTON_PRESSES {
        let mut v: Vec<(bool, usize, usize)> = Vec::with_capacity(APPROX_SIGNALS_SENT_PER_BUTTON_PRESS);
        let mut v_index = 0;
        v.push((false, broadcaster, 0));
        num_low += 1;

        while v_index < v.len() {
            let (signal, module_index, source_index) = v[v_index];
            v_index += 1;
            match modules.get_mut(module_index) {
                Some(module) => {
                    if let Some(pulse_to_send) = module.kind.tranceive(signal, source_index) {
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

    // Each broadcaster output leads to a counter loop. This is a stricter assumption than the
    // previous implementation, but I still feel it is ok.
    let p2: usize = modules[broadcaster].destination_modules.iter()
        .map(|m| {
            // Two output nodes. One is the counter, and the other is the second bit.
            let counter = *(modules[*m].destination_modules.iter().find(|x| modules[**x].destination_modules.len() > 2).unwrap());
            let all_bits_except_the_last = (0..(NUM_BITS-1))
                .fold((0, m), |(mut value, module_index), shift| {
                    let next = modules[*module_index].destination_modules.iter().find(|x| **x != counter).unwrap();
                    if modules[*module_index].destination_modules.len() > 1 {
                        value += 1 << shift;
                    };
                    (value, next)
                }).0;
            all_bits_except_the_last + (1 << (NUM_BITS-1)) // Last bit is always set. As is the first one.
        })
        .product();

    Solution::new(p1, p2)
}
