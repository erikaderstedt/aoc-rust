use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Operation {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Instruction {
    pub op: Operation,
    pub a: usize,
    pub b: usize,
    pub out: usize,
}

impl Operation {
    pub fn from(s: &str) -> Operation {
        match s {
            "addi" => Operation::Addi,
            "addr" => Operation::Addr,
            "muli" => Operation::Muli,
            "mulr" => Operation::Mulr,
            "seti" => Operation::Seti,
            "setr" => Operation::Setr,
            "gtrr" => Operation::Gtrr,
            "gtir" => Operation::Gtir,
            "gtri" => Operation::Gtri,
            "eqrr" => Operation::Eqrr,
            "eqir" => Operation::Eqir,
            "eqri" => Operation::Eqri,
            "borr" => Operation::Borr,
            "bori" => Operation::Bori,
            "banr" => Operation::Banr,
            "bani" => Operation::Bani,
            _ => panic!("Unknown operation {}", s),
        }
    }
}

impl Instruction {
    pub fn execute<const N: usize>(&self, data: &mut [usize; N]) {
        match self.op {
            Operation::Addi => data[self.out] = data[self.a] + self.b,
            Operation::Addr => data[self.out] = data[self.a] + data[self.b],
            Operation::Muli => data[self.out] = data[self.a] * self.b,
            Operation::Mulr => data[self.out] = data[self.a] * data[self.b],
            Operation::Bani => data[self.out] = data[self.a] & self.b,
            Operation::Banr => data[self.out] = data[self.a] & data[self.b],
            Operation::Bori => data[self.out] = data[self.a] | self.b,
            Operation::Borr => data[self.out] = data[self.a] | data[self.b],
            Operation::Eqir => data[self.out] = if self.a == data[self.b] { 1 } else { 0 },
            Operation::Eqri => data[self.out] = if data[self.a] == self.b { 1 } else { 0 },
            Operation::Eqrr => data[self.out] = if data[self.a] == data[self.b] { 1 } else { 0 },
            Operation::Gtir => data[self.out] = if self.a > data[self.b] { 1 } else { 0 },
            Operation::Gtri => data[self.out] = if data[self.a] > self.b { 1 } else { 0 },
            Operation::Gtrr => data[self.out] = if data[self.a] > data[self.b] { 1 } else { 0 },
            Operation::Setr => data[self.out] = data[self.a],
            Operation::Seti => data[self.out] = self.a,
        }
    }

    pub fn parse_program(s: &str) -> (usize, Vec<Instruction>) {
        let (first_line, rest) = s.split_once("\n").unwrap();
        let ip_register = first_line
            .split(' ')
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .next()
            .unwrap();

        let instructions: Vec<Instruction> = rest
            .lines()
            .map(|s| {
                let (op, a, b, out) = s.split(' ').collect_tuple().unwrap();
                let op = Operation::from(op);
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                let out = out.parse::<usize>().unwrap();
                Instruction { op, a, b, out }
            })
            .collect();

        (ip_register, instructions)
    }
}
