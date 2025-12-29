// https://adventofcode.com/2018/day/7

use crate::common::Solution;

const NUM_WORKERS: usize = 5;
const STEP_DURATION_MIN: usize = 60;
const INDEPENDENT: u32 = 1u32 << 31;

fn sequence(mut requirements: [u32; 26]) -> String {
    let mut m1: Vec<char> = Vec::new();
    while requirements.iter().any(|&v| v > 0) {
        let next_step = requirements.iter().position(|&v| v == INDEPENDENT).unwrap() as u8;
        for m in requirements.iter_mut() {
            *m &= !(1u32 << next_step);
        }
        m1.push((next_step + ('A' as u8)) as char);
        requirements[next_step as usize] &= 0x7fffffff;
    }

    m1.into_iter().collect()
}

type WorkItem = usize;

#[derive(Debug, Copy, Clone)]
struct AssignedTask {
    minutes_left: usize,
    item: WorkItem,
}

impl AssignedTask {
    fn start(item: WorkItem) -> AssignedTask {
        AssignedTask {
            minutes_left: item + STEP_DURATION_MIN,
            item: item,
        }
    }
    fn worked(&self) -> AssignedTask {
        AssignedTask {
            minutes_left: self.minutes_left - 1,
            item: self.item,
        }
    }

    fn is_completed(&self) -> bool {
        self.minutes_left == 0
    }
}

pub fn solve(input: &str) -> Solution {
    let mut requirements = [INDEPENDENT; 26];

    for line in input.lines() {
        let must_be_finished = line.as_bytes()[5] - ('A' as u8);
        let can_begin = line.as_bytes()[36] - ('A' as u8);
        requirements[can_begin as usize] |= 1 << must_be_finished;
    }

    let p1 = sequence(requirements.clone());

    let mut workers: [Option<AssignedTask>; NUM_WORKERS] = [None; NUM_WORKERS];
    let mut task_queue: Vec<WorkItem> = requirements
        .iter()
        .enumerate()
        .filter(|v| *(v.1) == INDEPENDENT)
        .map(|v| v.0)
        .collect();
    let mut p2 = 0;
    let mut completed_tasks = 0;

    while completed_tasks < 26 && p2 < 1000 {
        for worker in workers.iter_mut() {
            match *worker {
                Some(w) if !w.is_completed() => *worker = Some(w.worked()),
                Some(w) => {
                    // Check requirements to see which tasks to push on the queue.
                    completed_tasks = completed_tasks + 1;
                    for (i, r) in requirements.iter_mut().enumerate() {
                        if (*r & (1 << w.item)) > 0 {
                            *r ^= 1 << w.item;
                            if *r == INDEPENDENT {
                                task_queue.push(i);
                            }
                        }
                    }
                    *worker = None;
                }
                None => {}
            }
        }
        for worker in workers.iter_mut() {
            match worker {
                None => {
                    *worker = match task_queue.pop() {
                        Some(available_task) => Some(AssignedTask::start(available_task)),
                        None => None,
                    };
                }
                _ => {}
            }
        }
        p2 = p2 + 1;
    }

    p2 = p2 - 1;

    Solution::new(p1, p2)
}
