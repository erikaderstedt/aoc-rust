// https://adventofcode.com/2024/day/9

use crate::common::Solution;

fn sum_of_range(start: usize, length: usize) -> usize {
    start * length + length * (length - 1) / 2
}

fn defrag_p1(input: &str) -> usize {
let mut disk_map: Vec<u8> = input.trim().as_bytes().into_iter()
        .map(|b| b - 48)
        .collect();

    let num_files = (disk_map.len() + 1) >> 1;

    let mut p1 = 0;
    let mut position = 0;
    let mut last_file_id = num_files - 1;
    let mut index_of_last_file = disk_map.len() - 1;

    for file_id in 0..num_files {
        let length = disk_map[file_id*2] as usize;

        p1 += file_id * sum_of_range(position, length);
        position += length;

        if index_of_last_file == file_id*2 {
            break;
        }

        let mut empty_block_length_remaining = disk_map[file_id*2 + 1] as usize;
        
        while empty_block_length_remaining > 0 {
            let last_file_length = disk_map[index_of_last_file] as usize;
            if empty_block_length_remaining >= last_file_length {
                // Consume all of it
                p1 += last_file_id * sum_of_range(position, last_file_length);
                position += last_file_length;
                // Pop two items off disk_map
                disk_map.pop();
                disk_map.pop();
                last_file_id -= 1;
                index_of_last_file -= 2;
                empty_block_length_remaining -= last_file_length;
            } else {
                // Consume part of it.
                p1 += last_file_id * sum_of_range(position, empty_block_length_remaining);
                disk_map[index_of_last_file] -= empty_block_length_remaining as u8;
                position += empty_block_length_remaining;            
                empty_block_length_remaining = 0;
            }

            if last_file_id <= file_id {
                break;
            }
        }

        if last_file_id <= file_id {
            break;
        }
    }

    p1
}

struct File {
    id: usize,
    position: usize,
    length: usize,
}

struct EmptySpace {
    position: usize,
    length: usize,
}

impl File {
    fn value(&self) -> usize {
        self.id * sum_of_range(self.position, self.length)
    }
}

pub fn solve(input: &str) -> Solution {
    let p1 = defrag_p1(input);

    let disk: Vec<(usize, (usize, usize))> = input.trim().as_bytes().into_iter()
        .scan(0usize, |position, length| {
            let start = position.clone();
            let length = (*length as usize) - 48;
            *position = *position + length;
            Some((start, length))
        })
        .enumerate()
        .collect();

    let mut files: Vec<File> = disk.iter()
        .step_by(2)
        .map(|(id, (position, length))| 
            File { id: id >> 1, position: *position, length: *length })
        .collect();
    let mut empty_places: Vec<EmptySpace> = disk.iter()
        .skip(1)
        .step_by(2)
        .map(|(_, (position, length))| 
            EmptySpace { position: *position, length: *length })
        .collect();

    let mut p2 = 0;
    for last_file in files.iter_mut().rev() {
        if let Some(available_slot) = empty_places.iter_mut()
            .take(last_file.id)
            .find(|e| e.length >= last_file.length) {
            last_file.position = available_slot.position;
            available_slot.position += last_file.length;
            available_slot.length -= last_file.length;
        }
        p2 += last_file.value();
    }

    Solution::new(p1, p2)
}
