// https://adventofcode.com/2024/day/24

use std::{collections::{HashMap, HashSet}, fmt::Debug};

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

    fn swap_output(self: &mut Gate, other: &mut Gate) {
        let w = self.output;
        self.output = other.output;
        other.output = w;
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

    // println!("{:?}", v);

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

    // let mut outputs: = gates.iter().map(|g| g.output.clone()).collect();

    // let mut outputs: HashMap<Wire,bool> = HashMap::new();

    // while v.len() < gates.len() + pure_inputs  {
    //     for gate in gates
    //     .iter() {
    //         if !v.contains_key(&gate.output) {
    //             match (v.get(&gate.inputs.0), v.get(&gate.inputs.1)) {
    //                 (Some(v1), Some(v2)) => {
    //                     v.insert(gate.output.clone(), 
    //                     match gate.gate {
    //                         Operation::And => *v1 && *v2,
    //                         Operation::Or => *v1 || *v2,
    //                         Operation::Xor => v1 ^ v2,
    //                     });
    //                 },
    //                 _ => {},
    //             }
    //         }
    //     }
    // }

    // let p1 = v.iter()
    // .filter(|(k,_)| **k >> 16 == (b'z' as u32))
    // .fold(0u64, |t,(k,v)| {
    //     if *v {
    //         // println!("got it!");
    //         t + (1u64 << index_for_wire(*k))
    //     } else {
    //         t
    //     }
    // });
    let p1 = 0;
    
    // 100111000100010110011101111110110001100000011
    // 111101010111100001110111111011000110000100001

    //     let a2:u64 = 0b110000001100011011111101110011010001000111001;
    // let b2:u64 = 0b100001000011000110111111011100001111010101111;
    //     println!("Hello, world! {} {} = {}", a2,b2,a2 + b2);
    // let c: u64 = 45213383376616;
    // println!("{:045b}", a2+b2); // 44663761780968
    // println!("{:045b}", c);
    // 1010001001111100010111101001111100000011101000
    // 1010010001111100001111101001111011111011101000

    // fgw is first Cin

    // Full adder
    // S = A XOR B XOR Cin
    // Cout = (A AND B) OR (Cin AND (A XOR B))

    // For each gate, identify:
    // 1. A XOR B
    // 2. A AND B
    // 3. Cin AND (A XOR B)
    // 4. Cout

    let mut cin = gates.iter().find(|g| g.has_inputs( &wire(b"x00"), &wire(b"y00"))).unwrap().output;


 
    // 9: cnk, qwf flipped (a xor b flipped with a and b)
    // 14: z14, vhm flipped (a and b flipped with (A XOR B) XOR cin)
    // 27: mps, z27 (s and cout flipped)
    // 39: msq z39 (s and (A XOR B) AND Cin flipped)

    // cnk, qwf z14, vhm, mps, msq,z27 z39
    // cnk,mps,msq,qwf,vhm,z14,z27,z39

    let mut wrong: Vec<Wire> = vec![];

    for i in 1..=44 {
        let x = wire(format!("x{:02}", i).as_bytes());
        let y = wire(format!("y{:02}", i).as_bytes());
        let z = wire(format!("z{:02}", i).as_bytes());

        // all xor and and gates seem to be present, there are 45 of each in my input
        let mut xor = gates.iter().find(|g| g.has_inputs(&x, &y) && g.gate == Operation::Xor).unwrap().clone();
        let mut and = gates.iter().find(|g| g.has_inputs(&x, &y) && g.gate == Operation::And).unwrap().clone();
        let mut output = gates.iter().find(|&g| g.has_input(cin) && g.gate == Operation::Xor);

        // (cin is not and-ed with a xor b) && (cin is not xor-ed with a xor b) -> either a xor b is wrong or prev cout was wrong
        // a and b is not or-ed with (cin and (a xor b)) -> either a and b is wrong or intermediate is wrong

        // println!("{} cin {}", i, wire_to_string(&cin));
        let cout = gates.iter().find(|&g| g.gate == Operation::Or && g.has_input(&and.output));
        let xor_out = gates.iter().find(|&g| g.gate == Operation::Xor && g.has_input(&xor.output));

        // 5 different outputs in each full adder
        // A XOR B  ---> (A XOR B) XOR Cin    ---> z
        //           --> (A XOR B) AND Cin    ---> intermediate
        // A AND B  ---> (A AND B) OR Intermediate ---> Cout

        // A XOR B must be used in both an XOR and in an AND, otherwise it is wrong
        // A AND B must be used in an OR, otherwise it is wrong
        // Cout must be used in an XOR and an AND (unless i == 44), otherwise it is wrong
        // z must be a product of cin XOR something, otherwise it is wrong
        // if exactly 1 wrong, then intermediate is wrong (this is not a given but seems to be true for my input - the swapped gates are always part of the same full adder)
        
        // correct error, then calculate new cin
        let and_used_in: Vec<Gate> = gates.iter().filter(|g| g.has_input(&and.output));
        let xor_used_in: Vec<Gate> = gates.iter().filter(|g| g.has_input(&xor.output));
        let cout_used_in: Vec<Gate> = gates.iter().filter(|g| g.has_input(&cin));
            
        let a_xor_b_correct = xor_used_in.len() == 2 &&
        ((xor_used_in[0].gate == Operation::Xor && xor_used_in[1].gate == Operation::And) ||
        (xor_used_in[1].gate == Operation::Xor && xor_used_in[0].gate == Operation::And));

        if !a_xor_b_correct {
            wrong.push(xor.output);
        }

        let cout_correct = cout_used_in.len() == 2 &&
        ((cout_used_in[0].gate == Operation::Xor && cout_used_in[1].gate == Operation::And) ||
        (cout_used_in[1].gate == Operation::Xor && cout_used_in[0].gate == Operation::And));

        if !cout_correct {
            wrong.push(cin.clone());
        }
        
        let a_and_b_correct = and_used_in.len() == 1 && and_used_in[0].gate == Operation::Or;

        if !a_and_b_correct {
            wrong.push(and.output);
        }
        
        let z_correct = output.output == z;

        match (a_and_b_correct, a_xor_b_correct, cout_correct, z_correct) {
            (false, false, true, true) => { 
                xor.swap_output(&mut and);
                wrong.push(xor.output.clone());
                wrong.push(and.output.clone());
            },
            (true, true, false, false) => { 
                let cout = gates.iter().find(|&g| g.has_input(&and.output) && g.gate == Operation::Or).unwrap();
                cout.swap_output(&output);
                wrong.push(cout.output.clone());
                wrong.push(output.output.clone());
            },
            (true, false, true, false) => { },
            (true, false, false, true) => { },
            (false, true, true, false) => { },
            (false, true, false, true) => { },
            (false, true, true, true) => { },
            (true, false, true, true) => { },
            (true, true, false, true) => { },
            (true, true, true, false) => { },
            _ => { panic!("Unsupported configuration.")},
            }
        }


    // 5 outputs per gate
        
        match (cout, xor_out, output) {

            (Some(cout), Some(xor_out), Some(output)) => {
                assert!(xor_out == output);
                // Does cin match the remaining input?
                // if
            }
        }
        // 
        // if xor is correct and z is correct
        // - previous cin was wrong
        //  
        if and_gate_is_not_used_by_or_gate && xor_is_not_used_by_xor {

        } else if and_gate_is_not_used_by_or_gate {
            // xor is correct
            if output_is_not_xor {

            } else {
                xor is 
            }

        } !xor_is_not_used_by_xor {
            // and is correct -> check here this uses OR -> get intermediate
            // 
            // Both are correct.


            // Look for intermediate
            let cout_is_used_by_two = gates.iter().find(|&g| g.gate == Operation::Xor && g.has_input(&xor.output)).is_none();

            // 
        }
        // If intermediate is ok, output is 

        // if two wrongs, flip them and evaluate
        // 
        
        if  {

        }




        // x14 XOR y14 -> rkm
        // ndq XOR rkm -> vhm
        // vhm mot z14

        // snv OR jgq -> z27 WRONG
        // should be mps 
        // bhb 

        // trn and gpm -> z39
        // x39 AND y39 -> mgb
        // y39 XOR x39 -> trn
        // gpm XOR trn -> msq
        // msq mot z39



    //     if s.has_inputs(&xor.output, &cin) {
            
    //         if let Some(intermediate) = gates.iter().find(|&g| g.gate == Operation::And && g.has_inputs(&xor.output, &cin)) {
    //             if let Some(cout) = gates.iter().find(|&g| g.gate == Operation::Or && g.has_inputs(&intermediate.output, &and.output)) {
    //                 cin = cout.output.clone();
    //             } else {
    //                 // and output is wrong, or intermediate output is wrong
    //                 println!("For {}, and output or intermediate output is wrong",i);
    //                 // possibly_wrong.push(and.output.clone());
    //                 // possibly_wrong.push(intermediate.output.clone());
    //             }
    //         } else {

    //             // xor output is wrong, or last cout was wrong
    //             panic!("For {}, xor or cin outputs worong???", i);

    //         }
    //     } else {
    //         println!("output does not match inputs for {}", i);
    //         if s.has_input(&xor.output) {
    //             println!("xor is fine, so previous cin was wrong. {}", i);
    //             wrong.push(cin);   
    //             cin = if s.inputs.0 == xor.output { s.inputs.1 } else { s.inputs.0 };
    //         } else if s.has_input(&cin) {
    //             println!("xor output is probably wrong for {}", i);
    //             wrong.push(xor.output);             
    //             xor.output = if s.inputs.0 == cin { s.inputs.1 } else { s.inputs.0 };

    //             if let Some(intermediate) = gates.iter().find(|&g| g.gate == Operation::And && g.has_inputs(&xor.output, &cin)) {
    //                 if let Some(cout) = gates.iter().find(|&g| g.gate == Operation::Or && g.has_inputs(&intermediate.output, &and.output)) {
    //                     cin = cout.output.clone();
    //                 } else {
    //                     // and output is wrong, or intermediate output is wrong
    //                     println!("2For {}, and output or intermediate output is wrong",i);
    //                     // possibly_wrong.push(and.output.clone());
    //                     // possibly_wrong.push(intermediate.output.clone());
    //                 }
    //             } else {
    
    //                 // xor output is wrong, or last cout was wrong
    //                 panic!("2For {}, xor or cin outputs worong???", i);
    
    //             }
    //         } else {
    //             panic!("two wrongs?");
    //         }
    //     }
        if wrong.len() == 8 {
            break;
        }
    }


    //     // if let Some(intermediate) 
    //     // let out = gates.iter().
    //     // println!("{}: intermediate {:?}", i, match intermediate { Some(g) => wire_to_string(&g.output), None => "???".to_string() });

    // }

    // bgb XOR Cin -> z01
    //         (fgw AND bgb) -> gww 
    // phj OR gww -> wwp (Cout)


    // x00 AND y00 -> fgw
    // x00 XOR y00 -> z00

    // y01 AND x01 -> pjh
    // x01 XOR y01 -> bgb
    // fgw XOR bgb -> z01

    // x02 XOR y02 -> wmw
    // x02 AND y02 -> wrc
    // fgw AND bgb -> gww
    // pjh OR gww -> wwp
    // wmw XOR wwp -> z02

    // wwp AND wmw -> dng

    // x03 XOR y03 -> tbk
    // y03 AND x03 -> qjg
    // qjg OR vph -> cpn
    // tbk XOR vmk -> z03
    // dng OR wrc -> vmk

    // ndn XOR njd -> z44

    let p2 = 0;

    Solution::new(p1, p2)
}
