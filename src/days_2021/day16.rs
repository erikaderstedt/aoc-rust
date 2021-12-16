// https://adventofcode.com/2021/day/16
use crate::common::Solution;

struct Bitstream {
    stream: Vec<u64>, // 4 bits per u8
    bit: usize,
    index: usize,
}

impl Bitstream {

    fn counter(&self) -> usize {
        self.index * 64 + self.bit
    }

    fn from(input: &str) -> Bitstream {
        let mut stream: Vec<u64> = input.as_bytes().chunks(16)
            .map(|i| i.iter().fold(0u64, |v, x| (v << 4) + (match x {
                            b'0'..=b'9' => x - b'0',
                            b'A'..=b'F' => x - b'A' + 10,
                            _ => panic!("Not a hexadecimal number")}) as u64))
            .collect();
        let last = stream.len()-1;
        stream[last] <<= 4*(16 - (input.len()%16));

        Bitstream { stream, bit: 0, index: 0 }
    }

    fn extract<const N:usize>(&mut self) -> u64 {
        if self.bit > 64 - N {
            // Combine two values
            let bits_in_this_index = 64 - self.bit;
            let bits_in_next_index = N - bits_in_this_index;
            let mut value = (self.stream[self.index] >> self.bit) << bits_in_next_index;
            self.index += 1;
            value |= self.stream[self.index] >> (64 - bits_in_next_index);
            self.stream[self.index] <<= bits_in_next_index;
            self.bit = bits_in_next_index;
            value
        } else {
            let value = self.stream[self.index] >> (64-N);
            self.stream[self.index] <<= N;
            self.bit += N;
            value
        }
    }

}

enum PacketType {
    Literal(u64),
    Operator(u8,Vec<Packet>),
}

struct Packet {
    version: u8,
    packet_type: PacketType,
}

const PACKET_TYPE_LITERAL: u8 = 0b100;

impl Packet {

    fn from<'a>(bitstream: &mut Bitstream) -> Packet {
        let version = bitstream.extract::<3>() as u8;
        let t = bitstream.extract::<3>() as u8;
        let packet_type = match t {
            PACKET_TYPE_LITERAL => {
                let mut v: u64 = 0;
                loop {
                    let block = bitstream.extract::<5>();
                    v = (v << 4) + ((block & 0b1111) as u64);
                    if (block & 0b10000) == 0 { break }
                };
                PacketType::Literal(v)
            },
            operator_type => {
                let mut sub_packets = Vec::new();
                let length_type = bitstream.extract::<1>();
                match length_type {
                    0 => {
                        let total_length = bitstream.extract::<15>() as usize;
                        let start = bitstream.counter();
                        while start + total_length > bitstream.counter() {
                            let p = Packet::from(bitstream);
                            sub_packets.push(p);
                        }
                    },
                    1 => {
                        let num_packets = bitstream.extract::<11>();
                        for _ in 0..num_packets {
                            let p = Packet::from(bitstream);
                            sub_packets.push(p);
                        }
                    },
                    _ => { panic!("no!") },
                };
                PacketType::Operator(operator_type, sub_packets) },
        };
        Packet { version, packet_type }
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
                    0 => sub_packet_values.into_iter().sum(),
                    1 => sub_packet_values.into_iter().product(),
                    7 => if sub_packet_values[0] == sub_packet_values[1] { 1 } else { 0 },
                    2 => sub_packet_values.into_iter().min().unwrap(),
                    3 => sub_packet_values.into_iter().max().unwrap(),
                    5 => if sub_packet_values[0] > sub_packet_values[1] { 1 } else { 0 },
                    6 => if sub_packet_values[0] < sub_packet_values[1] { 1 } else { 0 },
                    _ => panic!("Unrecognized operator type"),
                }
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut bitstream = Bitstream::from(input);
    let p = Packet::from(&mut bitstream);
    
    let m1 = p.total_version();
    let m2 = p.value();
    
    Solution::new(m1,m2)
}
