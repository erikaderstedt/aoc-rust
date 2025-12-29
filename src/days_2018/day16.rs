// https://adventofcode.com/2018/day/16

use crate::common::Solution;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Sample {
    before: [usize; 4],
    op_code: usize,
    input_a: usize,
    input_b: usize,
    output: usize,
    after: [usize; 4],
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn process(&self, input_a: usize, input_b: usize, output: usize, data: &mut [usize; 4]) {
        match self {
            Opcode::Addi => data[output] = data[input_a] + input_b,
            Opcode::Addr => data[output] = data[input_a] + data[input_b],
            Opcode::Muli => data[output] = data[input_a] * input_b,
            Opcode::Mulr => data[output] = data[input_a] * data[input_b],
            Opcode::Bani => data[output] = data[input_a] & input_b,
            Opcode::Banr => data[output] = data[input_a] & data[input_b],
            Opcode::Bori => data[output] = data[input_a] | input_b,
            Opcode::Borr => data[output] = data[input_a] | data[input_b],
            Opcode::Eqir => data[output] = if input_a == data[input_b] { 1 } else { 0 },
            Opcode::Eqri => data[output] = if data[input_a] == input_b { 1 } else { 0 },
            Opcode::Eqrr => data[output] = if data[input_a] == data[input_b] { 1 } else { 0 },
            Opcode::Gtir => data[output] = if input_a > data[input_b] { 1 } else { 0 },
            Opcode::Gtri => data[output] = if data[input_a] > input_b { 1 } else { 0 },
            Opcode::Gtrr => data[output] = if data[input_a] > data[input_b] { 1 } else { 0 },
            Opcode::Setr => data[output] = data[input_a],
            Opcode::Seti => data[output] = input_a,
        }
    }
}

impl Sample {
    fn apply_ri<F>(&self, f: F, h: &mut HashSet<Opcode>, opcode: Opcode)
    where
        F: Fn(usize, usize) -> usize,
    {
        if f(self.before[self.input_a], self.input_b) == self.after[self.output] {
            h.insert(opcode);
        }
    }
    fn apply_rr<F>(&self, f: F, h: &mut HashSet<Opcode>, opcode: Opcode)
    where
        F: Fn(usize, usize) -> usize,
    {
        if f(self.before[self.input_a], self.before[self.input_b]) == self.after[self.output] {
            h.insert(opcode);
        }
    }
    fn apply_ir<F>(&self, f: F, h: &mut HashSet<Opcode>, opcode: Opcode)
    where
        F: Fn(usize, usize) -> usize,
    {
        if f(self.input_a, self.before[self.input_b]) == self.after[self.output] {
            h.insert(opcode);
        }
    }
    fn matches_opcodes(&self) -> HashSet<Opcode> {
        let mut h = HashSet::new();
        self.apply_ri(|a, b| a + b, &mut h, Opcode::Addi);
        self.apply_rr(|a, b| a + b, &mut h, Opcode::Addr);
        self.apply_ri(|a, b| a * b, &mut h, Opcode::Muli);
        self.apply_rr(|a, b| a * b, &mut h, Opcode::Mulr);
        self.apply_ri(|a, b| a & b, &mut h, Opcode::Bani);
        self.apply_rr(|a, b| a & b, &mut h, Opcode::Banr);
        self.apply_ri(|a, b| a | b, &mut h, Opcode::Bori);
        self.apply_rr(|a, b| a | b, &mut h, Opcode::Borr);

        self.apply_ri(|a, _| a, &mut h, Opcode::Setr);
        self.apply_ir(|a, _| a, &mut h, Opcode::Seti);
        self.apply_ir(|a, b| if a > b { 1 } else { 0 }, &mut h, Opcode::Gtir);
        self.apply_ri(|a, b| if a > b { 1 } else { 0 }, &mut h, Opcode::Gtri);
        self.apply_rr(|a, b| if a > b { 1 } else { 0 }, &mut h, Opcode::Gtrr);
        self.apply_ir(|a, b| if a == b { 1 } else { 0 }, &mut h, Opcode::Eqir);
        self.apply_ri(|a, b| if a == b { 1 } else { 0 }, &mut h, Opcode::Eqri);
        self.apply_rr(|a, b| if a == b { 1 } else { 0 }, &mut h, Opcode::Eqrr);

        h
    }
}

pub fn solve(input: &str) -> Solution {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let num_samples = blocks.len() - 2;

    let possible_opcodes: Vec<(usize, HashSet<Opcode>)> = blocks
        .iter()
        .take(num_samples)
        .map(|s| Sample::from(s))
        .map(|sample| (sample.op_code, sample.matches_opcodes()))
        .collect();

    let p1 = possible_opcodes
        .iter()
        .filter(|(_, h)| h.len() >= 3)
        .count();

    let mut opcodes: [Option<Opcode>; 16] = [None; 16];

    while opcodes.iter().any(|p| p.is_none()) {
        for i in 0..16 {
            if opcodes[i].is_some() {
                continue;
            }
            let mut common = possible_opcodes
                .iter()
                .cloned()
                .filter_map(|(o, h)| if o == i { Some(h) } else { None })
                .reduce(|acc, h| acc.intersection(&h).cloned().collect())
                .unwrap();

            // Remove already determined opcodes
            for det in opcodes.iter().filter_map(|u| *u) {
                common.remove(&det);
            }
            if common.len() == 1 {
                let code = common.drain().next().unwrap();
                opcodes[i] = Some(code);
            }
        }
    }

    let mut regs = [0; 4];
    for (opcode_index, input_a, input_b, output) in blocks.last().unwrap().lines().map(|line| {
        line.split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    }) {
        let op = opcodes[opcode_index].unwrap();
        op.process(input_a, input_b, output, &mut regs);
    }
    let p2 = regs[0];

    Solution::new(p1, p2)
}

impl Sample {
    fn from(s: &str) -> Sample {
        let (row1, row2, row3) = s.lines().collect_tuple().unwrap();
        let (op_code, input_a, input_b, output) = row2
            .trim()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let before = row1[9..=18]
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_array()
            .unwrap();
        let after = row3[9..=18]
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_array()
            .unwrap();
        Sample {
            before,
            op_code,
            input_a,
            input_b,
            output,
            after,
        }
    }
}
