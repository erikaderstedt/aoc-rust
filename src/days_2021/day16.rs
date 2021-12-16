// https://adventofcode.com/2021/day/16
use crate::common::Solution;

enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl OperatorType {
    fn from(v: u8) -> OperatorType {
        match v {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::Equal,
            _ => panic!("Unexpected operator type"),
        }
    }
}

enum PacketType {
    Literal(u64),
    Operator(OperatorType,Vec<Packet>),
}

struct Packet {
    version: u8,
    length: u64,
    packet_type: PacketType,
}

fn to_bits(input: &str) -> Vec<u8> {
    let mut v = Vec::with_capacity(input.len()*4);
    for c in input.as_bytes() {
        let v1 = match c { b'0'..=b'9' => c - b'0', b'A'..=b'F' => c - b'A' + 10, _ => panic!("Bad hexadecimal") };
        v.push(if (v1 & 0b1000) > 0 { 1 } else { 0 });
        v.push(if (v1 & 0b0100) > 0 { 1 } else { 0 });
        v.push(if (v1 & 0b0010) > 0 { 1 } else { 0 });
        v.push(if (v1 & 0b0001) > 0 { 1 } else { 0 });
    }
    v
}

const PACKET_TYPE_LITERAL: u8 = 0b100;

// fn get_bits(x: &[u8], start: u8, num_bits: u8) -> u8 {
//     // 1, 5
//     // 
//     if 8-start < num_bits {
//         println!("{:08b}{:08b} {} {} {:>05b} {:08b}", x[0], x[1],(num_bits - (8-start)), (8 - (num_bits - (8-start))),
//         (x[0] & ((1 << (8-start))-1)) << (num_bits - (8-start)), (x[1] >> (8 - (num_bits - (8-start))))
//     );
//         (x[0] & ((1 << (8-start))-1)) << (num_bits - (8-start)) +
//         (x[1] >> (8 - (num_bits - (8-start))))
//     } else {
//         (x[0] >> (8 - start - num_bits)) & ((1 << num_bits) - 1)
//     }
// }

// impl Packet {

//     fn from(x:&[u8]) -> (Option<Packet>, usize) {

//         let version = x[0] >> 5;
//         let t = (x[0] >> 2) & 0x7;
//         println!("Version: {:>03b}. Type: {:>03b} {:>08b}", version,t, x[0]);
//             },
//             _ => { PacketType::Operator },
//         };
//         (Some(Packet { version, packet_type }), 0)
//     }
// }

impl Packet {

    fn from(x: &Vec<u8>, index: &mut usize) -> Packet {
        let start = index.clone();
        let version = read(x, index, 3) as u8;
        let t = read(x, index, 3) as u8;
        // let mut length = 6;
        let packet_type = match t {
            PACKET_TYPE_LITERAL => {
                // Read groups of 5 bits.
                let mut v: u64 = 0;
                loop {
                    v = v << 4;
                    let block = read(&x, index, 5);
                    v += (block & 0b1111) as u64;
                    // length += 5;
                    if (block & 0b10000) == 0 { break }
                };
                // println!("Literal {}", v);
                PacketType::Literal(v)
            },
            t => {
                let operator_type = OperatorType::from(t);
                let mut sub_packets = Vec::new();
                let length_type = read(x, index, 1);
                // length += 1;
                match length_type {
                    0 => {
                        let total_length = read(x, index, 15);
                        // length += total_length + 15;
                        let mut sub_packet_length = 0;
                        while sub_packet_length < total_length {
                            let p = Packet::from(x, index);
                            sub_packet_length += p.length;
                            sub_packets.push(p);
                        }
                    },
                    1 => {
                        let num_packets = read(x, index, 11);
                        // length += 11;
                        for _ in 0..num_packets {
                            let p = Packet::from(x, index);
                            // length += p.length;
                            sub_packets.push(p);
                        }
                    },
                    _ => { panic!("no!") },
                };
                PacketType::Operator(operator_type, sub_packets) },
        };
        let length = (*index - start) as u64;
        Packet { version, length, packet_type }
    }

    fn total_version(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as u64,
            PacketType::Operator(_, sub_packets) => sub_packets.iter()
                    .fold(self.version as u64, |s,p| s + p.total_version()),
        }
    }

    fn value(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(v) => v.clone(),
            PacketType::Operator(t, sub_packets) => {
                let sub_packet_values: Vec<u64> = sub_packets.iter().map(|p| p.value()).collect();
                match t {
                    OperatorType::Sum => sub_packet_values.into_iter().sum(),
                    OperatorType::Product => sub_packet_values.into_iter().product(),
                    OperatorType::Equal => if sub_packet_values[0] == sub_packet_values[1] { 1 } else { 0 },
                    OperatorType::Minimum => sub_packet_values.into_iter().min().unwrap(),
                    OperatorType::Maximum => sub_packet_values.into_iter().max().unwrap(),
                    OperatorType::GreaterThan => if sub_packet_values[0] > sub_packet_values[1] { 1 } else { 0 },
                    OperatorType::LessThan => if sub_packet_values[0] < sub_packet_values[1] { 1 } else { 0 },
                }
            }
        }
    }
}

fn read(x: &Vec<u8>, index: &mut usize, num_bits: usize) -> u64 {
    (0..num_bits).fold(0u64, |v, _| { 
        let a = (v << 1) + (x[*index] as u64);
        *index += 1;
        a})
}

pub fn solve(input: &str) -> Solution {
    let x = to_bits(input);
    let mut i = 0;
    let p = Packet::from(&x, &mut i);
    
    let m1 = p.total_version();
    let m2 = p.value();

    Solution::new(m1,m2)
}
