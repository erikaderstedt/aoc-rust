// https://adventofcode.com/2022/day/19

use crate::common::{Solution, parsed_from_each_line};
use std::str::FromStr;

const NUM_MATERIALS: usize = 4;
const NUM_INGREDIENTS: usize = 4;
const FAR_FUTURE: u16 = 100;

type OreAmount = u16;
type RobotAmount = OreAmount;
type Recipe = [OreAmount; NUM_INGREDIENTS];

#[derive(Debug)]
struct Blueprint {
    recipes: [Recipe; NUM_MATERIALS],
}

#[derive(Debug)]
struct State {
    materials: [OreAmount; NUM_MATERIALS],
    robots: [RobotAmount; NUM_MATERIALS],
    time_left: u16,
}

impl Blueprint {

    // exhaustive dfs search
    fn simulate(&self, state: State, max_robots: &[RobotAmount; NUM_MATERIALS], max_geodes: &mut OreAmount) {
        let mut robot_was_built = false;

        // At each iteration, we branch into building each available robot that has not yet reached the maximum
        // number required. The path that we trace out is the order in which we build the robots.
        for robot_type in 0..NUM_MATERIALS {
            // Do we build a robot of this type next?
            if state.robots[robot_type] == max_robots[robot_type] {
                continue;
            }
            let recipe = &self.recipes[robot_type];
            let completion_time = (0..NUM_INGREDIENTS)
                .filter_map(|material_type| match recipe[material_type] {
                    0 => None, // This ore type is not required to build this robot type.
                    x if x <= state.materials[material_type] => Some(0), // We already have enough for this robot.
                    _ if state.robots[material_type] == 0 => Some(FAR_FUTURE), // There is no robot for this type of ore yet, so robot type 'i' is not available
                    _ => Some((recipe[material_type] - state.materials[material_type] + state.robots[material_type] - 1) / state.robots[material_type]),
                })
                .max()
                .unwrap() + 1;

            if completion_time >= state.time_left {
                continue;
            }
            let time_remaining_when_finished = state.time_left - completion_time;

            let mut new_materials = [0; NUM_MATERIALS];
            let mut new_robots = [0; NUM_MATERIALS];
            for o in 0..NUM_MATERIALS {
                new_materials[o] = state.materials[o] + state.robots[o] * completion_time - recipe[o];
                new_robots[o] = state.robots[o] + u16::from(o == robot_type);
            }

            // If we were to build only geode robots every turn after building the robot, could we beat the current max?
            if ((time_remaining_when_finished - 1) * time_remaining_when_finished) / 2
                + new_materials[3] + time_remaining_when_finished * new_robots[3]
                < *max_geodes
            {
                continue;
            }
            robot_was_built = true;
            self.simulate(
                State {
                    materials: new_materials,
                    robots: new_robots,
                    time_left: time_remaining_when_finished,
                },
                max_robots,
                max_geodes,
            );
        }
        if !robot_was_built {
            // We couldn't make new robots. Calculate the number of geodes we end up with if we let the clock run out
            *max_geodes = (*max_geodes).max(state.materials[3] + state.robots[3] * state.time_left as u16);
        }
    }

    fn run_simulation(&self, max_time: u16) -> OreAmount {
        let mut max_robots = [u16::MAX; NUM_MATERIALS];
        for i in 0..3 {
            max_robots[i] = self.recipes.iter().map(|r| r[i]).max().unwrap();
        }
        let mut max_geodes = 0;
        self.simulate(
            State {
                materials: [0; NUM_MATERIALS],
                robots: [1, 0, 0, 0],
                time_left: max_time
            },
            &max_robots,
            &mut max_geodes,
        );
        max_geodes
    }
}

pub fn solve(input: &str) -> Solution {
    let blueprints: Vec<Blueprint> = parsed_from_each_line(input);

    let p1: OreAmount = blueprints.iter()
        .enumerate()
        .map(|(index, blueprint)| blueprint.run_simulation(24) * (index as OreAmount + 1) )
        .sum();

    let p2: OreAmount = blueprints.iter()
        .take(3)
        .map(|blueprint| blueprint.run_simulation(32) )
        .product();

    Solution::new(p1,p2)
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = line.split(" ").collect();

        Ok(Blueprint { recipes: [
                [s[6].parse().unwrap(),0,0,0],
                [s[12].parse().unwrap(),0,0,0],
                [s[18].parse().unwrap(), s[21].parse().unwrap(),0,0],
                [s[27].parse().unwrap(), 0, s[30].parse().unwrap(),0]] })
    }
}
