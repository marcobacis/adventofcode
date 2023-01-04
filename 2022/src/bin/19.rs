// Using BFS. Started with ugly code and not pruning (really slow)
// Later adapted to legobmw99 solution on reddit (https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0v1sul/?utm_source=share&utm_medium=web2x&context=3)

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Default, Debug)]
struct Blueprint {
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

impl Blueprint {
    fn cost(&self, robot: Robot) -> RobotCost {
        match robot {
            Robot::Ore => self.ore_robot_cost,
            Robot::Clay => self.clay_robot_cost,
            Robot::Obsidian => self.obsidian_robot_cost,
            Robot::Geode => self.geode_robot_cost,
        }
    }

    fn can_build(&self, state: &State, robot: Robot) -> bool {
        let RobotCost {
            ore,
            clay,
            obsidian,
        } = self.cost(robot);

        state.ore >= ore && state.clay >= clay && state.obsidian >= obsidian
    }

    fn should_build(&self, state: &State, robot: Robot, built: bool) -> bool {
        // Always build geodes
        if robot == Robot::Geode {
            return true;
        }

        /*
           Assuming to build one robot per minute, we just need enough material every minute
           Prune branches in which we produce more than [ore,clay,obsidian]/minute than we need.
        */
        let max_material = [
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost,
            self.geode_robot_cost,
        ]
        .iter()
        .map(|cost| cost.cost(robot))
        .max()
        .unwrap();

        let needed = state.robots(robot) < max_material;

        if built {
            needed
        } else {
            // Did we skip it already? If we skipped it before, we should skip it now too
            let prev_state = state.revert_mine();
            let skipped_before = self.can_build(&prev_state, robot);
            needed && !skipped_before
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct RobotCost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl RobotCost {
    fn cost(self, robot: Robot) -> usize {
        match robot {
            Robot::Ore => self.ore,
            Robot::Clay => self.clay,
            Robot::Obsidian => self.obsidian,
            Robot::Geode => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,

    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl State {
    fn new() -> State {
        State {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn robots(&self, robot: Robot) -> usize {
        match robot {
            Robot::Ore => self.ore_robot,
            Robot::Clay => self.clay_robot,
            Robot::Obsidian => self.obsidian_robot,
            Robot::Geode => self.geode_robot,
        }
    }

    fn _mine(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }

    fn revert_mine(self) -> State {
        let mut new = self;
        new.ore -= new.ore_robot;
        new.clay -= new.clay_robot;
        new.obsidian -= new.obsidian_robot;
        new.geode -= new.geode_robot;
        new
    }

    fn mine(self) -> State {
        let mut other = self;
        other._mine();
        other
    }

    fn build(self, blueprint: &Blueprint, robot: Robot) -> State {
        let mut new_state = self;

        let RobotCost {
            ore,
            clay,
            obsidian,
        } = blueprint.cost(robot);

        new_state.ore -= ore;
        new_state.clay -= clay;
        new_state.obsidian -= obsidian;

        new_state._mine();

        match robot {
            Robot::Ore => new_state.ore_robot += 1,
            Robot::Clay => new_state.clay_robot += 1,
            Robot::Obsidian => new_state.obsidian_robot += 1,
            Robot::Geode => new_state.geode_robot += 1,
        };

        new_state
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            // Get all numbers on the line
            let mut nums = l
                .split_ascii_whitespace()
                .filter(|s| s.parse::<usize>().is_ok())
                .map(|s| s.parse::<usize>().unwrap());
            Blueprint {
                ore_robot_cost: RobotCost {
                    ore: nums.next().unwrap(),
                    ..Default::default()
                },
                clay_robot_cost: RobotCost {
                    ore: nums.next().unwrap(),
                    ..Default::default()
                },
                obsidian_robot_cost: RobotCost {
                    ore: nums.next().unwrap(),
                    clay: nums.next().unwrap(),
                    ..Default::default()
                },
                geode_robot_cost: RobotCost {
                    ore: nums.next().unwrap(),
                    obsidian: nums.next().unwrap(),
                    ..Default::default()
                },
            }
        })
        .collect()
}

fn search(blueprint: &Blueprint, max_time: usize) -> usize {
    // State, time, has build in last iteration
    let mut queue: VecDeque<(State, usize, bool)> = VecDeque::new();

    // maximum geodes per minute
    let mut max_geodes: HashMap<usize, usize> = HashMap::new();
    for i in 0..=max_time {
        max_geodes.insert(i, 0);
    }

    queue.push_back((State::new(), 0, false));

    while let Some((state, time, built)) = queue.pop_front() {
        // Cut branches in which we are not producing enough geodes
        let &prev_geodes = max_geodes.get(&time).unwrap();
        if state.geode < prev_geodes {
            continue;
        }
        max_geodes.insert(time, prev_geodes.max(state.geode));

        // End condition, time passed, check number of geodes created
        if time == max_time {
            continue;
        }

        // Greedy, if we can produce a geode do it and skip
        if blueprint.can_build(&state, Robot::Geode) {
            queue.push_back((state.build(blueprint, Robot::Geode), time + 1, true));
            continue;
        }

        // Only mine, no robot production
        queue.push_back((state.mine(), time + 1, false));

        // Production of other robots
        for robot in [Robot::Obsidian, Robot::Clay, Robot::Ore] {
            if blueprint.can_build(&state, robot) && blueprint.should_build(&state, robot, built) {
                queue.push_back((state.build(&blueprint, robot), time + 1, true));
            }
        }
    }

    max_geodes[&max_time]
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse(input);
    Some(
        blueprints
            .iter()
            .enumerate()
            .map(|(idx, b)| (idx + 1) * search(b, 24))
            .fold(0, |acc, el| acc + el) as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse(input);
    Some(
        blueprints
            .iter()
            .take(3)
            .map(|b| search(b, 32))
            .fold(1, |acc, el| acc * el) as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(56 * 62));
    }
}
