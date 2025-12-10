use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct Machine {
    goal: u16,
    buttons: Vec<Vec<u8>>,
    requirements: Vec<u16>,
}

impl Machine {
    pub fn from_str(line: &str) -> Self {
        let splits: Vec<_> = line.split(" ").collect();

        let goal_str = splits[0].as_bytes();
    
        let mut goal = 0;
        for i in 0..goal_str.len() {
            if goal_str[i] == b'#' {
                let mask = 1 << (i-1);
                goal ^= mask;
            }
        }

        let buttons = splits[1..splits.len() - 1]
            .iter()
            .map(|g| {
                g[1..g.len() - 1]
                    .to_string()
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        let requirements = splits[splits.len() - 1][1..(splits[splits.len() - 1].len() - 1)]
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            goal,
            buttons,
            requirements,
        }
    }
}

fn next_lights(state: u16, button: &Vec<u8>) -> u16 {
    let mut next = state;
    for &light in button.iter() {
        next ^= 1 << light;
    }
    next
}

fn solve_part_one(machine: Machine) -> u32 {
    let mut visited: HashSet<u16> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((0, 0));

    while let Some((steps, state)) = queue.pop_front() {
        if visited.contains(&state) {
            continue;
        }

        if state == machine.goal {
            return steps;
        }

        visited.insert(state.clone());

        for button in &machine.buttons {
            queue.push_back((steps + 1, next_lights(state, button)));
        }
    }

    unreachable!("Did not find a solution")
}

fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Machine::from_str)
            .map(solve_part_one)
            .sum(),
    )
}

fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = fs::read_to_string("inputs/10.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/10.txt").unwrap();
        assert_eq!(Some(7), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/10.txt").unwrap();
        assert_eq!(None, part_two(&input));
    }
}
