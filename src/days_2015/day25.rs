use crate::common::Solution;
use std::str::FromStr;
use crate::common::parsed_from_each_line;

#[derive(Debug,Clone)]
enum Register {
    A, B, C, D
}

impl FromStr for Register {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err("Bad register!")
        }
    }
}

#[derive(Debug,Clone)]
enum Source {
    Immediate(i64),
    Register(Register),
}

impl FromStr for Source {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Register>() {
            Ok(r) => Ok(Source::Register(r)),
            Err(_e) => match s.parse::<i64>() {
                Ok(i) => Ok(Source::Immediate(i)),
                Err(_e) => Err("Bad source value!"),
            },
        }
    }
}

#[derive(Debug,Clone)]
enum AssembunnyInstruction {
    Cpy(Source,Source),
    Inc(Source),
    Dec(Source),
    Jnz(Source,Source),
    Tgl(Source),
    Out(Source),
}

impl FromStr for AssembunnyInstruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let s1 = parts[1].parse::<Source>()?;
        match parts[0] {
            "cpy" => {
                let s2 = parts[2].parse::<Source>()?;
                Ok(AssembunnyInstruction::Cpy(s1, s2))
            },
            "jnz" => {
                let s2 = parts[2].parse::<Source>()?;
                Ok(AssembunnyInstruction::Jnz(s1, s2))
            },
            "inc" => Ok(AssembunnyInstruction::Inc(s1)),
            "dec" => Ok(AssembunnyInstruction::Dec(s1)),
            "tgl" => Ok(AssembunnyInstruction::Tgl(s1)),
            "out" => Ok(AssembunnyInstruction::Out(s1)),
            _ => Err("Unknown instruction in assembunny code"),
        }
    }
}

#[derive(Debug,Clone)]
struct Program {
    instructions: Vec<AssembunnyInstruction>,
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    i: i64,
    last_transmission: i64,
    oscillations: usize,
}

const REQUIRED_NUM_OSCILLATIONS: usize = 20;

impl Program {

    // fn show_status(&self) {
    //     println!("{:2} {:6} {:6} {:6} {:6}", self.i + 1, self.a, self.b, self.c, self.d);
    // }

    fn get_value(&self, source: &Source) -> i64 {
        match source {
            Source::Immediate(i) => i.clone(),
            Source::Register(r) => match r {
                Register::A => self.a,
                Register::B => self.b,
                Register::C => self.c,
                Register::D => self.d,
            },
        }
    }
    
    fn set_value(&mut self, source: &Source, value: i64)  {
        match source {
            Source::Immediate(_) => {},
            Source::Register(r) => match r {
                Register::A => self.a = value,
                Register::B => self.b = value,
                Register::C => self.c = value,
                Register::D => self.d = value,
            }
        }
    }

    fn run(&mut self) -> bool {
        let mut v = vec![100i64; REQUIRED_NUM_OSCILLATIONS];
        loop {
            let i0 = self.i as usize;
            if self.i < 0 { panic!("Bad value for instruction pointer!") };
            if i0 >= self.instructions.len() { return false };
            match self.instructions[i0].clone() {
                AssembunnyInstruction::Inc(r) => { self.i += 1; self.set_value(&r, self.get_value(&r) + 1) },
                AssembunnyInstruction::Dec(r) => { self.i += 1; self.set_value(&r, self.get_value(&r) - 1) },
                AssembunnyInstruction::Cpy(s, r) => { self.i += 1; self.set_value(&r, self.get_value(&s)) },
                AssembunnyInstruction::Jnz(s, i) => if self.get_value(&s) != 0 { self.i += self.get_value(&i); } else { self.i += 1; }
                AssembunnyInstruction::Tgl(s) => { 
                    let delta = self.get_value(&s);
                    let j = delta + self.i; 
                    if delta != 0 && j >= 0 && (j as usize) < self.instructions.len()   {
                        let new_instruction = match self.instructions[j as usize].clone() {
                            AssembunnyInstruction::Inc(r) => AssembunnyInstruction::Dec(r.clone()),
                            AssembunnyInstruction::Dec(r) | AssembunnyInstruction::Tgl(r) | AssembunnyInstruction::Out(r) => AssembunnyInstruction::Inc(r.clone()),
                            AssembunnyInstruction::Jnz(s, i) => AssembunnyInstruction::Cpy(s,i),
                            AssembunnyInstruction::Cpy(s, r) => AssembunnyInstruction::Jnz(s, r),
                             
                        };
                        self.instructions[j as usize] = new_instruction;
                    }
                    self.i += 1; 
                },
                AssembunnyInstruction::Out(s) => {
                    let expected_value = 1 - self.last_transmission;
                    let received_value = self.get_value(&s);
                    if received_value != expected_value { 
                        return false;
                    } else {
                        v[self.oscillations] = expected_value;
                        self.last_transmission = expected_value;
                        self.oscillations += 1;
                        if self.oscillations == REQUIRED_NUM_OSCILLATIONS {
                            return true;
                        }
                    }
                    self.i += 1;
                }
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let instructions = parsed_from_each_line(input);
    let mut a = 1;

    loop {
        let mut program = Program {
            instructions: instructions.clone(),
            a: a, b: 0, c: 0, d: 0,
            i: 0,
            last_transmission: 1,
            oscillations: 0,
        };

        if program.run() {
            break;
        }
        a += 1;
    }


    
    Solution { part_1: a.to_string(), part_2: "".to_string() }
}