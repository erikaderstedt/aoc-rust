use crate::common::Solution;
use itertools::Itertools;

pub fn solve(lines: &[String]) -> Solution {
    let data: Vec<u8> = lines[0].chars().map(|x| x.to_string().parse::<u8>().expect("Not a digit")).collect();
    let width: usize = 25;
    let height: usize = 6;
    let chunk_size: usize = width * height;
    let frames: Vec<Vec<u8>> = data
        .chunks(chunk_size)
        .filter(|c| c.len() == chunk_size)
        .map(|x| x.to_vec()).collect();
    
    let fewest_zeroes = frames.iter().min_by_key(|x| {
        x.iter().filter(|y| **y == 0u8).count()
    }).unwrap();

    let num_1_times_num_2 = 
        fewest_zeroes.iter().filter(|x| **x == 1u8).count() *
        fewest_zeroes.iter().filter(|x| **x == 2u8).count();

    let image_data = frames.into_iter().rev().fold(vec![2u8;chunk_size], |render, frame| {
        // Replace the existing value except if a '2' (transparent)
        render.iter().zip(frame.iter()).map(|(r,f)| {
            if *f != 2u8 { f.clone() } else { r.clone() }
        }).collect()
    });

    let mut rendered_image = (0..height).map(|row| {
        image_data[(row*width)..((row+1)*width)].iter().map(|v: &u8| -> String {
            (match v {
                0u8 => " ",
                1u8 => "*",
                2u8 => ".",
                _ => "?",
            }).to_string()
        }).join("")
    }).join("\n");
    rendered_image.insert(0, '\n');
    
    (num_1_times_num_2.to_string(), rendered_image.to_string())
}