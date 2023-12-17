// https://adventofcode.com/2023/day/5

use crate::common::Solution;

#[derive(PartialEq, Eq)]
enum AddressingMode {
    Position,
    Immediate
}

fn run_program(mut m: Vec<isize>, mut inputs: Vec<isize>) -> Vec<isize> {

    let mut outputs = vec![];

    let mut i: usize = 0;
    loop {
        let opcode = m[i] % 100;
        let addressing_modes: [AddressingMode; 3] = 
            [if (m[i]/100)   % 10 == 0 { AddressingMode::Position } else { AddressingMode::Immediate },
             if (m[i]/1000)  % 10 == 0 { AddressingMode::Position } else { AddressingMode::Immediate },
             if (m[i]/10000) % 10 == 0 { AddressingMode::Position } else { AddressingMode::Immediate }];

        let param1 = if addressing_modes[0] == AddressingMode::Position { m[i+1] as usize } else { i+1 };
        let param2 = if addressing_modes[1] == AddressingMode::Position { m[i+2] as usize } else { i+2 };
        let param3 = if addressing_modes[2] == AddressingMode::Position { m[i+3] as usize } else { i+3 };

        match opcode {
            1 => { m[param3] = m[param1] + m[param2]; i += 4; },
            2 => { m[param3] = m[param1] * m[param2]; i += 4; },
            3 => { m[param1] = inputs.pop().unwrap(); i += 2; },
            4 => { outputs.push(m[param1]); i += 2; },
            5 => { if m[param1] != 0 { i = m[param2] as usize; } else { i += 3; }}
            6 => { if m[param1] == 0 { i = m[param2] as usize; } else { i += 3; }}
            7 => { m[param3] = if m[param1] < m[param2] { 1 } else { 0 }; i += 4; },
            8 => { m[param3] = if m[param1] == m[param2] { 1 } else { 0 }; i += 4; },
            99 => { return outputs; },
            _ => { panic!("Invalid opcode {}", opcode)},
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut memory: Vec<isize> = input.trim_end().split(',').map(|s| s.parse::<isize>().unwrap()).collect();

    memory.extend(vec![0isize; 100]);

    let p1 = run_program(memory.clone(), vec![1]).pop().unwrap();
    let p2 = run_program(memory, vec![5]).pop().unwrap();

    Solution::new(p1,p2)
}
