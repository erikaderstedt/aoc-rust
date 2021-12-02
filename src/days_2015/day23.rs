use crate::common::Solution;
use std::str::FromStr;
use crate::common::parsed_from_each_line;


#[derive(Debug,Clone)]
enum Register { A, B }

impl FromStr for Register {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err("Bad register!")
        }
    }
}

// #[derive(Debug,Clone)]
// enum Source {
//     Immediate(i64),
//     Register(Register),
// }

// impl FromStr for Source {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.parse::<Register>() {
//             Ok(r) => Ok(Source::Register(r)),
//             Err(_e) => match s.parse::<i64>() {
//                 Ok(i) => Ok(Source::Immediate(i)),
//                 Err(_e) => Err("Bad source value!"),
//             },
//         }
//     }
// }

#[derive(Debug,Clone)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register,i64),
    Jio(Register,i64),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ' ' || c == ',').collect();
        match parts[0] {
            "hlf" => {
                let s1 = parts[1].parse::<Register>()?;
                Ok(Instruction::Hlf(s1))
            },
            "tpl" => {
                let s1 = parts[1].parse::<Register>()?;
                Ok(Instruction::Tpl(s1))
            },
            "inc" => {
                let s1 = parts[1].parse::<Register>()?;
                Ok(Instruction::Inc(s1))
            },
            "jmp" => {
                let i = parts[1].parse::<i64>().expect("Bad int");
                Ok(Instruction::Jmp(i))
            },
            "jio" => {
                let s1 = parts[1].parse::<Register>()?;
                let i = parts[3].parse::<i64>().expect("Bad int");
                Ok(Instruction::Jio(s1,i))
            },
            "jie" => {
                let s1 = parts[1].parse::<Register>()?;
                let i = parts[3].parse::<i64>().expect("Bad int");
                Ok(Instruction::Jie(s1,i))
            }
            _ => Err("Unknown instruction in code"),
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    a: usize,
    b: usize,
    i: i64,
}

impl Program {

    // fn show_status(&self) {
    //     println!("{:2} {:6} {:6} {:6} {:6}", self.i + 1, self.a, self.b, self.c, self.d);
    // }

    fn get_value(&self, register: &Register) -> usize {
        match register {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn set_value(&mut self, register: &Register, value: usize) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }

    // fn get_value(&self, source: &Source) -> i64 {
    //     match source {
    //         Source::Immediate(i) => i.clone(),
    //         Source::Register(r) => match r {
    //             Register::A => self.a,
    //             Register::B => self.b,
    //             Register::C => self.c,
    //             Register::D => self.d,
    //         },
    //     }
    // }
    
    // fn set_value(&mut self, source: &Source, value: i64)  {
    //     match source {
    //         Source::Immediate(_) => {},
    //         Source::Register(r) => match r {
    //             Register::A => self.a = value,
    //             Register::B => self.b = value,
    //             Register::C => self.c = value,
    //             Register::D => self.d = value,
    //         }
    //     }
    // }

    fn run(&mut self) -> usize {
        loop {
            let i0 = self.i as usize;
            if self.i < 0 || i0 >= self.instructions.len() { return self.b }
            match self.instructions[i0].clone() {
                Instruction::Hlf(r) => { self.i += 1; self.set_value(&r, self.get_value(&r) >> 1) },
                Instruction::Inc(r) => { self.i += 1; self.set_value(&r, self.get_value(&r) + 1) },
                Instruction::Tpl(r) => { self.i += 1; self.set_value(&r, self.get_value(&r) * 3) },
                Instruction::Jmp(i) => { self.i += i },
                Instruction::Jio(r,i) => { self.i += if self.get_value(&r) == 1 { i } else { 1 }},
                Instruction::Jie(r,i) => { self.i += if self.get_value(&r) % 2 == 0 { i } else { 1 }},
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut program = Program {
        instructions: parsed_from_each_line(input),
        a: 0, b: 0,
        i: 0
    };

    let p1 = program.run();

    program = Program {
        instructions: parsed_from_each_line(input),
        a: 1, b: 0,
        i: 0
    };
    let p2 = program.run();
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}