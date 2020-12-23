use crate::common::Solution;

const LOWEST: u32 = 1;

fn play(cups: &mut Vec<u32>, number_of_moves: usize, mut current_cup: u32) {
    let highest = (cups.len()-1) as u32;
    for _m in 0..number_of_moves {
        // cups: cup # => next cup.
        let p1 = cups[current_cup as usize];
        let p2 = cups[p1 as usize];
        let p3 = cups[p2 as usize];
        cups[current_cup as usize] = cups[p3 as usize];
        let picked_up = [p1,p2,p3];

        let mut destination_cup = current_cup - 1;
        if destination_cup < LOWEST { destination_cup = highest; }
        while picked_up.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup < LOWEST {
                destination_cup = highest;
            }
        }
        let a = cups[destination_cup as usize];
        cups[destination_cup as usize] = p1;
        cups[p3 as usize] = a;

        current_cup = cups[current_cup as usize];
    }
}

const MY_INPUT:&str = "315679824";
pub fn solve(_input: &str) -> Solution {
    let data = MY_INPUT;
    let cup_v: Vec<u32> = data.as_bytes().iter().map(|b| (b - ('0' as u8)) as u32).collect();
    let p1 = {
        let mut cups: Vec<u32> = (1..=data.len()).map(|i| {
            let j = (cup_v.iter().position(|&v| v == (i as u32)).unwrap() + 1) % cup_v.len();
            cup_v[j]
        }).collect();
        cups.insert(0, 0);

        play(&mut cups, 100, 3);
        let mut current = 1;
        let mut p1 = "".to_string();
        for _i in 0..(cup_v.len()-1) {
            p1.push((cups[current] as u8  + ('0' as u8)) as char);
            current = cups[current] as usize;
        }
        p1
    };

    let p2 = { 
        let mut cups: Vec<u32> = (0..1)
            .chain((1..=cup_v.len()).map(|i| {
                let j = (cup_v.iter().position(|&v| v == (i as u32)).unwrap() + 1) % cup_v.len();
                cup_v[j]
            }))
            .chain(11..=1_000_000)
            .chain(cup_v[0]..(cup_v[0]+1))
            .collect();
        cups[cup_v[cup_v.len()-1] as usize] = 10;

        play(&mut cups, 10_000_000, 3);

        (cups[1] as usize) * (cups[cups[1] as usize] as usize)
    };
 
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}