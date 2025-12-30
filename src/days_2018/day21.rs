// https://adventofcode.com/2018/day/20

use crate::{
    common::Solution,
    days_2018::operation::{Instruction, Operation},
};

pub fn next(value: usize, constant: usize) -> usize {
    let mut a = value | 65536;
    let mut b = constant; // seti 1397714 1 3
    while a > 0 {
        b += a & 0xff; // bani 2 255 5, addr 3 5 3
        b &= 16777215; // bani 3 16777215 3
        b *= 65899; // muli 3 65899 3
        b &= 16777215; // bani 3 16777215 3
        a = a >> 8;
    }
    b
}

pub fn solve(input: &str) -> Solution {
    let (ip_register, instructions) = Instruction::parse_program(input);

    let p1 = {
        let mut registers = [0usize; 6];
        let mut ip = 0;

        loop {
            registers[ip_register] = ip;
            instructions[ip].execute(&mut registers);
            if instructions[ip].op == Operation::Eqrr {
                break registers[3];
            }

            ip = registers[ip_register] + 1;
        }
    };

    // Get the constant - a "seti" operation with argument > 100000.
    let constant = instructions
        .iter()
        .filter(|i| i.op == Operation::Seti && i.a > 100000)
        .map(|i| i.a)
        .next()
        .unwrap();
    let mut seen = vec![0];
    // When the loop starts to repeat we will never halt.
    // The last non-repeating value is the longest we can go for and still halt.
    let p2 = loop {
        let value = next(seen.last().unwrap().clone(), constant);
        if seen.contains(&value) {
            break seen.last().unwrap();
        }
        seen.push(value);
    };

    // This way also works, but takes around 9 seconds to run.
    // let mut registers = [0usize; 6];
    // let mut ip = 0;
    // let p2 = loop {
    //     registers[ip_register] = ip;
    //     instructions[ip].execute(&mut registers);
    //     if instructions[ip].op == Operation::Eqrr {
    //         let value = registers[instructions[ip].a];
    //         if seen.contains(&value) {
    //             break seen.last().unwrap();
    //         } else {
    //             seen.push(value);
    //         }
    //     }

    //     ip = registers[ip_register] + 1;
    // };
    Solution::new(p1, p2)
}
