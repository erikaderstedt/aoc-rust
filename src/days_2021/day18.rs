// https://adventofcode.com/2021/day/18
use crate::common::Solution;
use itertools::Itertools;
use std::iter::Peekable;
use std::fmt;

type Value = u16;
type Level = u16;

#[derive(Clone)]
struct Number {
    value: Value,
    level: Level,
}

#[derive(Clone)]
struct Snailfish {
    n: Vec<Number>,
}

impl Snailfish {

    fn from(c: &[u8]) -> Snailfish {
        let mut level = 0;
        let len = c.len();
        let mut nums = Vec::new();
        for i in 0..len {
            match c[i] {
                b'[' => { level += 1; },
                b']' => { level -= 1; },
                b',' => { },
                b'0'..=b'9' => { nums.push(Number { value: (c[i] - b'0') as Value, level: level})},
                _ => { panic!("??"); }
            }
        }
        Snailfish { n: nums }
    }

    fn magnitude(&self) -> Value {
        Snailfish::inner_magnitude(&mut self.n.iter().peekable(), 1)
    }

    fn next<'a, I>(numbers: &mut Peekable<I>, base_level: Level) -> Value
        where I: Iterator<Item=&'a Number> {
        if numbers.peek().unwrap().level > base_level {
            Snailfish::inner_magnitude(numbers, base_level + 1)
        } else { 
            numbers.next().unwrap().value as Value
        }
    }

    fn inner_magnitude<'a, I>(numbers: &mut Peekable<I>, base_level: Level) -> Value 
        where I: Iterator<Item=&'a Number> {
        Snailfish::next(numbers, base_level) * 3 + Snailfish::next(numbers, base_level) * 2
    }

    fn add(&self, other: &Snailfish) -> Snailfish {
        let mut combined: Vec<Number> = self.n.iter()
            .chain(other.n.iter())
            .cloned()        
            .collect();
        for f in combined.iter_mut() { f.level += 1; }

        loop {
            if let Some(i) = combined.iter().position(|x| x.level > 4) {
                if i > 0 {
                    combined[i-1].value += combined[i].value;
                }
                if i + 1 < combined.len()-1 {
                    combined[i+2].value += combined[i+1].value;
                }
                combined.remove(i+1);
                combined[i].level -= 1;
                combined[i].value = 0;
                continue;
            }

            if let Some(i) = combined.iter().position(|x| x.value >= 10) {
                let value = combined[i].value;
                combined[i].value = value/2;
                combined[i].level += 1;
                combined.insert(i+1, Number {
                    level: combined[i].level,
                    value: value - combined[i].value });
                continue;
            }
            break;
        }
        Snailfish { n: combined }
    }
}

pub fn solve(input: &str) -> Solution {
    let fishies: Vec<Snailfish> = input.lines().map(|s| Snailfish::from(s.as_bytes())).collect();
    let m1 = fishies.iter().fold(fishies[0].clone(), |f1, f2| f1.add(f2)).magnitude();
    let m2 = fishies.into_iter().permutations(2).map(|f| -> Value {
        f[0].add(&f[1]).magnitude()
    }).max().unwrap();
    
    Solution::new(m1,m2)
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        Snailfish::display(f, &mut self.n.iter().peekable(), 1).unwrap();
        write!(f, "]")?;
        Ok(())
    }
}

impl Snailfish {
    
    fn display<'a, I>(formatter: &mut fmt::Formatter, numbers: &mut Peekable<I>, base_level: Level) -> Result<(),std::fmt::Error> 
    where I: Iterator<Item=&'a Number>
    {
        if numbers.peek().unwrap().level > base_level {
            write!(formatter, "[")?;
            Snailfish::display(formatter, numbers, base_level + 1)?;
            write!(formatter, "]")?;
        } else { 
            write!(formatter, "{}", numbers.next().unwrap().value)?;
        }

        write!(formatter, ",")?;
    
        if numbers.peek().unwrap().level > base_level {
            write!(formatter, "[")?;
            Snailfish::display(formatter, numbers, base_level + 1)?;
            write!(formatter, "]")?;
        } else { 
            write!(formatter, "{}", numbers.next().unwrap().value)?;
        }

        Ok(())
    }

}
