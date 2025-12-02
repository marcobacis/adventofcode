use std::fs;

use itertools::Itertools;

fn part_one(input: &str) -> Option<u64> {
    let line = input.lines().next().unwrap();
    Some(
        line.split(",")
            .filter_map(|l| l.split_once("-"))
            .map(invalid_part_one)
            .sum::<u64>(),
    )
}

fn invalid_part_one(range: (&str, &str)) -> u64 {
    let (low, high) = range;
    let high: u64 = high.parse().unwrap();
    let low: u64 = low.parse().unwrap();

    let mut count = 0;
    for n in low..=high {
        let n_str = n.to_string();
        let (left, right) = n_str.split_at(n_str.len() / 2);
        if left == right {
            count += n;
        }
    }
    count
}

fn part_two(input: &str) -> Option<u64> {
    let line = input.lines().next().unwrap();
    Some(
        line.split(",")
            .filter_map(|l| l.split_once("-"))
            .map(invalid_part_two)
            .sum::<u64>(),
    )
}

fn invalid_part_two(range: (&str, &str)) -> u64 {
    let (low, high) = range;
    let high: u64 = high.parse().unwrap();
    let low: u64 = low.parse().unwrap();

    let mut count = 0;
    for n in low..=high {
        let n_str: Vec<char> = n.to_string().chars().collect();
        if has_repeating_sequence(&n_str) {
            count += n;
        }
    }
    count
}

fn has_repeating_sequence(n_str: &Vec<char>) -> bool {
    for s in 1..=(n_str.len() / 2) {
        let chunks: Vec<_> = n_str.chunks(s).collect();
        if chunks.len() > 1 && chunks.iter().all(|c| *c == chunks[0]) {
            return true;
        }
    }
    false
}

fn main() {
    let input = fs::read_to_string("inputs/02.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(Some(1227775554), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(Some(4174379265), part_two(&input));
    }
}
