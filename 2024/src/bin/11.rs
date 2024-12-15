use std::{collections::HashMap, fs};

fn blink_single(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }
    let digits = n.ilog10() + 1;
    if digits % 2 == 0 {
        let left = n / (10u64.pow(digits / 2));
        let right = n % (10u64.pow(digits / 2));
        return vec![left, right];
    }
    vec![n * 2024]
}

fn blink(input: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();

    input.iter().for_each(|(k, c)| {
        let new_stones = blink_single(*k);
        for stone in new_stones {
            result
                .entry(stone)
                .and_modify(|count| *count += c)
                .or_insert(*c);
        }
    });

    result
}

fn solve(input: &str, num_blinks: usize) -> u64 {
    let input: Vec<u64> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let mut entries = input.iter().fold(HashMap::new(), |mut map, n| {
        map.entry(*n).and_modify(|c| *c += 1).or_insert(1u64);
        map
    });

    for _ in 0..num_blinks {
        entries = blink(entries);
    }

    entries.iter().map(|s| *s.1).sum::<u64>()
}

fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

fn main() {
    let input = fs::read_to_string("inputs/11.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/11.txt").unwrap();
        assert_eq!(Some(55312), part_one(&input));
    }
}
