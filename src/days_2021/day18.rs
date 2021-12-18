// https://adventofcode.com/2021/day/18
use crate::common::Solution;
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

    fn from(bytes: &[u8]) -> Snailfish {
        let mut level = 0;
        Snailfish { n: bytes.iter().filter_map(|c|
            match c {
                b'[' => { level += 1; None },
                b']' => { level -= 1; None },
                b',' => { None },
                b'0'..=b'9' => { Some(
                    Number { value: (c - b'0') as Value, level: level } ) },
                _ => { panic!("??"); }
            }).collect() }
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
        Snailfish::next(numbers, base_level) * 3 + 
        Snailfish::next(numbers, base_level) * 2
    }

    fn explode<const PRE: bool>(n: &mut Vec<Number>) {
        let level = if PRE { 3 } else { 4 };
        while let Some(i) = n.iter().position(|x| x.level > level) {
            if i < n.len()-2 {
                n[i+2].value += n[i+1].value;
            } else if PRE {
                break
            }

            if i > 0 {
                n[i-1].value += n[i].value;
            }

            n.remove(i+1);
            n[i] = Number { level: n[i].level - 1, value: 0 };
        }
    }

    fn split(n: &mut Vec<Number>) -> bool {
        if let Some(i) = n.iter().position(|x| x.value >= 10) {
            let level = n[i].level + 1;
            let value = n[i].value;
            if level > 4 {
                // This pair will explode. Do it directly without the insert and
                // remove operations.
                if i > 0 { n[i-1].value += value/2; }
                if i < n.len()-1 { n[i+1].value += value - value/2; }
                n[i].value = 0;
            } else {
                n[i] = Number { level, value: value/2 };
                n.insert(i+1, Number { level, value: value - value/2 });
            }
            true
        } else {
            false
        }
    }
    
    fn add(&self, other: &Snailfish) -> Snailfish {
        let mut combined: Vec<Number> = self.n.iter()
            .chain(other.n.iter())
            .map(|n| Number { value: n.value, level: n.level + 1 })
            .collect();

        loop {
            Snailfish::explode::<false>(&mut combined);
            if Snailfish::split(&mut combined) { continue }
            break;
        }
        Snailfish { n: combined }
    }
}


pub fn solve(input: &str) -> Solution {
    let fishies: Vec<Snailfish> = input.lines().map(|s| Snailfish::from(s.as_bytes())).collect();
    let m1 = fishies.iter().fold(fishies[0].clone(), |f1, f2| f1.add(f2)).magnitude();

    let right_side = fishies.clone();
    let m2 = fishies.into_iter().map(|mut fish| {
        Snailfish::explode::<true>(&mut fish.n);
        right_side.iter().map(|o| fish.add(o).magnitude()).max().unwrap()
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
