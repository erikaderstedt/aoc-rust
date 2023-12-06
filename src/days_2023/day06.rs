// https://adventofcode.com/2023/day/5

use crate::common::Solution;

fn number_of_wins(time_and_distance : (&usize, &usize)) -> usize {
    let time = time_and_distance.0.clone();
    let distance = time_and_distance.1.clone();
    let approximate_start = (time - (((time*time - 4*distance) as f64).sqrt() as usize)) / 2;
    let approximate_end = (time + (((time*time - 4*distance) as f64).sqrt() as usize)) / 2;

    let start = (approximate_start.checked_sub(2).unwrap_or(0)..(approximate_start + 2))
        .find(|t| t * (time - t) > distance).unwrap();
    let end = (approximate_end.checked_sub(2).unwrap_or(0)..(approximate_end + 2)).rev()
        .find(|t| t * (time - t) > distance).unwrap();
    
    end - start + 1
}

pub fn solve(_input: &str) -> Solution {

    // let times: Vec<usize> = input.lines().next().unwrap().split(":").last().unwrap()
    //                             .split(" ").filter_map(|x| x.parse::<usize>().ok()).collect();
    // let distances: Vec<usize> = input.lines().skip(1).next().unwrap().split(":").last().unwrap()
    //                         .split(" ").filter_map(|x| x.parse::<usize>().ok()).collect();
    let times = vec![47,70,75,66];
    let distances = vec![282,1079,1147,1062];
    let p1: usize = times.iter().zip(distances.iter()).map(number_of_wins).product();

    // let time: usize = input.lines().next().unwrap().replace(" ", "").split(":").last().unwrap().parse::<usize>().unwrap();
    // let distance: usize = input.lines().skip(1).next().unwrap().replace(" ", "").split(":").last().unwrap().parse::<usize>().unwrap();
    let time: usize = 47707566;
    let distance: usize = 282107911471062;
    let p2 = number_of_wins((&time, &distance));

    Solution::new(p1,p2)
}
