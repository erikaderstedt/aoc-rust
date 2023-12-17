use itertools::Itertools;

use crate::common::Solution;

fn run_program(memory: &Vec<usize>, noun: usize, verb: usize) -> usize {

    let mut memory = memory.clone();
    memory[1] = noun;
    memory[2] = verb;

    let mut instruction_pointer: usize = 0;
    loop {
        let f = match memory[instruction_pointer] {
            1 => { |a,b| a + b },
            2 => { |a,b| a * b },
            99 => { return memory[0]; }
            _ => { panic!("Invalid opcode!"); },
        };

        let term1 = memory[memory[instruction_pointer + 1]];
        let term2 = memory[memory[instruction_pointer + 2]];
        let destination = memory[instruction_pointer + 3];
        memory[destination] = f(term1,term2);
        instruction_pointer += 4;
    }
}

pub fn solve(input: &str) -> Solution {
    let memory: Vec<usize> = input.trim_end().split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    let p1 = run_program(&memory, 12, 2);

    let (noun, verb) = (0..100)
        .cartesian_product(0..100)
        .find(|(noun, verb)| run_program(&memory, *noun, *verb) == 19690720)
        .unwrap();

    let p2 = 100 * noun + verb;
    
    Solution::new(p1,p2)
}
