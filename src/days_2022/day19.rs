// https://adventofcode.com/2022/day/19

use crate::common::Solution;
use std::str::FromStr;

const NUM_MATERIALS: usize = 4;
const NUM_INGREDIENTS: usize = 4;
const FAR_FUTURE: u16 = 100;
const GEODE: usize = 3;

type Amount = u16;
type RobotAmount = Amount;
type Recipe = [Amount; NUM_INGREDIENTS];

#[derive(Debug)]
struct Blueprint {
    costs: [Recipe; NUM_MATERIALS],
    max_robots: [RobotAmount; NUM_MATERIALS],
    best_geode_result: Amount
}

impl Blueprint {

    fn simulate(&mut self, materials: [Amount; NUM_MATERIALS], robots: [RobotAmount; NUM_MATERIALS], time_left: u16) {
        let mut robot_was_built = false;

        // At each iteration, we branch into building each available robot that has not yet reached the maximum
        // number required. The path that we trace out is the order in which we build the robots.
        for robot_type in 0..NUM_MATERIALS {
            // Do we build a robot of this type next?
            if robots[robot_type] == self.max_robots[robot_type] {
                // We don't need any more robots of this type.
                continue;
            }
            let recipe = &self.costs[robot_type];
            let completion_time = (0..NUM_INGREDIENTS)
                .map(|material_type| 
                    if recipe[material_type] <= materials[material_type] {
                        0 // We already have enough for this robot.
                    } else if robots[material_type] == 0 {
                        FAR_FUTURE // There is no robot for this type of ore yet, so robot type 'robot_type' is not available
                    } else {
                        (recipe[material_type] - materials[material_type] + robots[material_type] - 1) / robots[material_type]
                    }
                )
                .max()
                .unwrap() + 1;

            if completion_time >= time_left {
                continue;
            }
            let time_remaining_when_finished = time_left - completion_time;

            let mut new_materials = [0; NUM_MATERIALS];
            let mut new_robots = [0; NUM_MATERIALS];
            for o in 0..NUM_MATERIALS {
                new_materials[o] = materials[o] + robots[o] * completion_time - recipe[o];
                new_robots[o] = robots[o] + u16::from(o == robot_type);
            }

            // If we were to build only geode robots every turn after building the robot, could we beat the current max?
            // (and, if the other three types of robots are maxed out, this is the case)
            if ((time_remaining_when_finished - 1) * time_remaining_when_finished) / 2  // Aritmetic series series
                + new_materials[GEODE]  // What we have now
                + time_remaining_when_finished * new_robots[GEODE] // What the current number of robots are producing
                <= self.best_geode_result {
                continue;
            }
            robot_was_built = true;
            self.simulate(new_materials, new_robots, time_remaining_when_finished);
        }
        if !robot_was_built {
            // We couldn't make new robots. Calculate the number of geodes we end up with if we let the clock run out
            self.best_geode_result = self.best_geode_result.max(materials[GEODE] + robots[GEODE] * time_left as u16);
        }
    }

    fn run_simulation(&mut self, max_time: u16) -> Amount {
        self.simulate([0; NUM_MATERIALS], [1, 0, 0, 0], max_time);
        self.best_geode_result
    }
}

pub fn solve(input: &str) -> Solution {
    let p1: Amount = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .enumerate()
        .map(|(index, mut blueprint)| blueprint.run_simulation(24) * (index as Amount + 1) )
        .sum();

    let p2: Amount = input
        .lines()
        .take(3)
        .map(|line| line.parse::<Blueprint>().unwrap())
        .map(|mut blueprint| blueprint.run_simulation(32) )
        .product();

    Solution::new(p1,p2)
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = line.split(" ").collect();

        let costs = [[s[6].parse().unwrap(),0,0,0],
                    [s[12].parse().unwrap(),0,0,0],
                    [s[18].parse().unwrap(), s[21].parse().unwrap(),0,0],
                    [s[27].parse().unwrap(), 0, s[30].parse().unwrap(),0]];

        let mut max_robots = [u16::MAX; NUM_MATERIALS];
        for i in 0..3 {
            max_robots[i] = costs.iter().map(|r| r[i]).max().unwrap();
        }

        Ok(Blueprint { costs, max_robots, best_geode_result: 0 })
    }
}
