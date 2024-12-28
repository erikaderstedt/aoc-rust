// https://adventofcode.com/2024/day/24

use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

use crate::common::Solution;

type Wire = u32;

#[derive(PartialEq, Eq,Clone,Debug)]
enum Operation {
    Xor,
    And,
    Or,
}

#[derive(Debug,Clone)]
struct Gate {
    inputs: (Wire, Wire),
    gate: Operation,
    output: Wire, 
}

impl Gate {
    fn has_inputs(self: &Gate, wire1: &Wire, wire2: &Wire) -> bool {
        (self.inputs.0 == *wire1 && self.inputs.1 == *wire2) || (self.inputs.0 == *wire2 && self.inputs.1 == *wire1)
    }
    fn has_input(self: &Gate, wire1: &Wire) -> bool {
        self.inputs.0 == *wire1 || self.inputs.1 == *wire1
    }
}

fn wire(b: &[u8]) -> Wire {
    ((b[0] as u32) << 16) + ((b[1] as u32) << 8) + (b[2] as u32)
}

fn index_for_wire(wire: Wire) -> u32 {
    let d1 = ((wire >> 8) & 0xff) - (b'0' as u32);
    let d2 = (wire & 0xff) - (b'0' as u32);

    (d1 * 10) + d2
}

fn wire_to_string(wire: &Wire) -> String {
    let mut b: Vec<u8> = vec![0;3];
    b[0] = (wire >> 16) as u8;
    b[1] = ((wire >> 8) & 255) as u8;
    b[2] = (wire & 255) as u8;
    
    std::str::from_utf8(&b[..]).unwrap().to_string()
}

pub fn solve(input: &str) -> Solution {
    let mut v: HashMap<Wire, bool> = input
    .split("\n\n") 
    .next()
    .unwrap()
    .lines()
    .map(|line| {
        (wire(line.as_bytes()), line.as_bytes()[5] == b'1')        
    })
    .collect();

    let pure_inputs = v.len();

    let gates: Vec<Gate> = input
    .split("\n\n")
    .skip(1).next()
    .unwrap()
    .lines()
    .map(|line| {
        let s: Vec<&str> = line.split(' ').collect();
        Gate {
            inputs: (wire(s[0].as_bytes()), wire(s[2].as_bytes())),
            gate: match s[1] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("what?"),
            },
            output: wire(s[4].as_bytes())
        }
    })
    .collect();

    while v.len() < gates.len() + pure_inputs  {
        for gate in gates
        .iter() {
            if !v.contains_key(&gate.output) {
                match (v.get(&gate.inputs.0), v.get(&gate.inputs.1)) {
                    (Some(v1), Some(v2)) => {
                        v.insert(gate.output.clone(), 
                        match gate.gate {
                            Operation::And => *v1 && *v2,
                            Operation::Or => *v1 || *v2,
                            Operation::Xor => v1 ^ v2,
                        });
                    },
                    _ => {},
                }
            }
        }
    }

    let p1 = v.iter()
    .filter(|(k,_)| **k >> 16 == (b'z' as u32))
    .fold(0u64, |t,(k,v)| {
        if *v {
            t + (1u64 << index_for_wire(*k))
        } else {
            t
        }
    });

    let mut cin = gates.iter().find(|g| g.has_inputs( &wire(b"x00"), &wire(b"y00"))).unwrap().output;

    let mut wrong: Vec<Wire> = vec![];

    for i in 1..=44 {
        let x = wire(format!("x{:02}", i).as_bytes());
        let y = wire(format!("y{:02}", i).as_bytes());
        let z = wire(format!("z{:02}", i).as_bytes());

        // 5 different outputs in each full adder
        // A XOR B  ---> (A XOR B) XOR Cin    ---> z
        //           --> (A XOR B) AND Cin    ---> intermediate
        // A AND B  ---> (A AND B) OR Intermediate ---> Cout

        let xor = gates.iter().find(|g| g.has_inputs(&x, &y) && g.gate == Operation::Xor).unwrap().clone();
        let and = gates.iter().find(|g| g.has_inputs(&x, &y) && g.gate == Operation::And).unwrap().clone();
        let output = gates.iter().find(|&g| g.has_input(&cin) && g.gate == Operation::Xor).unwrap().clone();
        let intermediate = gates.iter().find(|&g| g.has_input(&cin) && g.gate == Operation::And).unwrap().clone();

        let a_xor_b_correct = intermediate.has_input(&xor.output) && output.has_input(&xor.output);
        let output_correct = output.output == z;

        cin = 
        if let Some(cout) = gates.iter().find(|&g| g.has_input(&and.output) && g.gate == Operation::Or) {
            let cout_used_in: Vec<Gate> = gates.iter().filter(|g| g.has_input(&cout.output)).cloned().collect();
            let cout_correct = cout_used_in.len() == 2 &&
                ((cout_used_in[0].gate == Operation::Xor && cout_used_in[1].gate == Operation::And) ||
                (cout_used_in[1].gate == Operation::Xor && cout_used_in[0].gate == Operation::And));
            
            // We can't detect the case where intermediate and and outputs are switched. Since this will not cause an
            // error in the output, it is not one of the available misconfigurations.
            if a_xor_b_correct { // a_and_b was correct
                if output_correct {
                    if cout_correct {
                        // Everything is correct (since errors come in pairs on the same full adder, at least in my input)
                        cout.output.clone()
                    } else {
                        wrong.push(intermediate.output);
                        wrong.push(cout.output);
                        intermediate.output.clone()
                    }
                } else {
                    if cout_correct {
                        wrong.push(intermediate.output);
                        wrong.push(output.output);
                        cout.output.clone()
                    } else {
                        wrong.push(cout.output);
                        wrong.push(output.output);
                        output.output.clone()
                    }
                }
            } else {
                if output_correct {
                    if cout_correct {
                        wrong.push(intermediate.output);
                        wrong.push(xor.output);
                        cout.output.clone()
                    } else {
                        wrong.push(cout.output);
                        wrong.push(xor.output);
                        xor.output.clone()
                    }
                } else {
                    wrong.push(output.output);
                    wrong.push(xor.output);
                    cout.output.clone()
                }
            }
        } else  if let Some(cout) = gates.iter().find(|&g| g.has_input(&intermediate.output) && g.gate == Operation::Or) {
            // a_and_b was incorrect

            if a_xor_b_correct {
                if output_correct {
                    wrong.push(cout.output);
                    wrong.push(and.output);
                    and.output.clone()
                } else {
                    wrong.push(and.output);
                    wrong.push(output.output);             
                    cout.output.clone()
                }
            } else {
                wrong.push(and.output);
                wrong.push(xor.output);             
                cout.output.clone()
            }
        } else {
            panic!("Should not be reachable.");
        };

        if wrong.len() == 8 {
            break;
        }
    }

    wrong.sort_unstable();
    let p2 = wrong.into_iter().map(|w| wire_to_string(&w)).join(",");

    Solution::new(p1, p2)
}
