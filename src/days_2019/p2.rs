use std::env;
use std::fs;
use std::path::Path;
fn run_program(mut p: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut i: usize = 0;
    p[1] = noun;
    p[2] = verb;
    while p[i] != 99 {
        i = match p[i] {
            1 => {
                let d = p[i+3];
                p[d] = p[p[i+2]] + p[p[i+1]];
                i + 4
            },
            2 => {
                let d = p[i+3];
                p[d] = p[p[i+2]] * p[p[i+1]];
                i + 4
            },
            _ => i + 1,
        };
    }
    return p[0];
}
fn main() {
    let filename = env::args().skip(1).next().expect("Not enough arguments");
    let path = Path::new(&filename);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let ints: Vec<usize> = contents.trim().split(",").map({ |x| x.parse::<usize>().unwrap() }).collect();
    println!("Pt 1: {:?}", run_program(ints.clone(), 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(ints.clone(), noun, verb) == 19690720 {
                println!("Pt 2: {:?}", noun*100+verb); 
            }
        }
    }
}