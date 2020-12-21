use crate::common::Solution;

pub fn solve(input: &str) -> Solution {

    let earliest = input.lines().take(1).next().unwrap().parse::<usize>().expect("");
    let buses: Vec<(usize,usize)> = input.lines().skip(1).take(1).next().unwrap().split(',')
        .enumerate()
        .filter(|(_,v)| *v != "x")
        .map(|(i,c)| (i, c.parse::<usize>().unwrap()))
        .collect();

    let p1 = buses.iter()
        .map(|&b| {
            let missed_by =  earliest % b.1;
            let minutes_left = b.1 - missed_by;
            (minutes_left, b.1)
        })
        .min_by(|a,b| a.0.cmp(&b.0))
        .unwrap();

    let p2 = buses.iter()
        .fold((0,1), |(solution_so_far, product_so_far), (remainder, bus_id)| {
            let mut m = solution_so_far;
            while (m+remainder) % bus_id != 0 {
                m += product_so_far;
            }
            (m, product_so_far*bus_id)
        })
        .0;

    Solution { part_1: (p1.0 * p1.1).to_string(), part_2: p2.to_string() }
}
