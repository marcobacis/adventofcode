use std::collections::HashMap;

use num::Integer;
use regex::Regex;

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let nodes: HashMap<&str, (&str, &str)> = lines
        .map(|l| {
            let cap = re.captures(l).unwrap();
            (
                cap.get(1).unwrap().as_str(),
                (cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str()),
            )
        })
        .collect();

    (instructions, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, nodes) = parse_input(input);

    let mut current = "AAA";
    let mut num = 0;
    while current != "ZZZ" {
        let instruction = instructions[num % instructions.len()];

        if instruction == 'L' {
            current = nodes[current].0;
        } else {
            current = nodes[current].1;
        }

        num = num + 1;
    }

    Some(num as u32)
}

fn solve_part_two_single_path(
    start: &str,
    instructions: &Vec<char>,
    nodes: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut current = start;
    let mut num = 0;
    while !current.ends_with("Z") {
        let instruction = instructions[num % instructions.len()];

        if instruction == 'L' {
            current = nodes[current].0;
        } else {
            current = nodes[current].1;
        }

        num = num + 1;
    }

    num
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse_input(input);

    // Tried bruteforcing, but with cycles its better to solve each one independently,
    // then find the lowest common multuplier to get where the loops will end together
    let current: Vec<&&str> = nodes.keys().filter(|n| n.ends_with("A")).collect();
    let lengths: Vec<usize> = current
        .iter()
        .map(|start| solve_part_two_single_path(start, &instructions, &nodes))
        .collect();

    let mut num = lengths[0];
    for i in 1..lengths.len() {
        num = num.lcm(&lengths[i]);
    }

    Some(num as u64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_one_2() {
        let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        assert_eq!(part_two(&input), Some(4));
    }
}
