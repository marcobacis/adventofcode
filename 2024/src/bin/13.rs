use std::fs;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

impl Machine {
    pub fn parse(lines: Vec<&str>) -> Self {
        let input = lines.join("");

        let re = Regex::new(r"\d+").unwrap();
        let numbers: Vec<u64> = re
            .find_iter(&input)
            .filter_map(|m| m.as_str().parse::<u64>().ok())
            .collect();

        Self {
            a: (numbers[1], numbers[0]),
            b: (numbers[3], numbers[2]),
            prize: (numbers[5], numbers[4]),
        }
    }
}

fn min_prize_moves(machine: &Machine) -> Option<u64> {
    // Cramer rule, see wikipedia
    let a1 = machine.a.0 as i64;
    let a2 = machine.a.1 as i64;
    let b1 = machine.b.0 as i64;
    let b2 = machine.b.1 as i64;
    let c1 = machine.prize.0 as i64;
    let c2 = machine.prize.1 as i64;

    let a = (b2 * c1 - b1 * c2) as f64 / (a1 * b2 - b1 * a2) as f64;
    let b = (a1 * c2 - a2 * c1) as f64 / (a1 * b2 - b1 * a2) as f64;

    if a >= 0f64 && b >= 0f64 && a.floor() == a && b.floor() == b {
        Some((a * 3f64 + b).floor() as u64)
    } else {
        None
    }
}

fn part_one(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| Machine::parse(chunk.collect()))
        .collect();
    Some(machines.iter().filter_map(min_prize_moves).sum())
}

fn part_two(input: &str) -> Option<u64> {
    let machines: Vec<Machine> = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|chunk| Machine::parse(chunk.collect()))
        .map(|machine| Machine {
            a: machine.a,
            b: machine.b,
            prize: (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            ),
        })
        .collect();
    Some(machines.iter().filter_map(min_prize_moves).sum())
}

fn main() {
    let input = fs::read_to_string("inputs/13.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/13.txt").unwrap();
        assert_eq!(Some(480), part_one(&input));
    }
}
