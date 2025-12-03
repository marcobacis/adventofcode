use std::fs;

fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|bank| max_joltage(bank, 2)).sum())
}

fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|bank| max_joltage(bank, 12)).sum())
}

fn max_joltage(bank: &str, n_digits: usize) -> u64 {
    let digits = bank
        .as_bytes().iter()
        .map(|c| *c - 48)
        .collect::<Vec<_>>();

    let mut bank = &digits[..];
    let mut n : u64 = 0;

    // Greedy, get highest (fedasible, leaving enough space) digit
    for place in (0..n_digits).rev() {
        let digit = bank[..(bank.len() - place)].iter().max().unwrap();
        let idx = bank.iter().position(|x| *x == *digit).unwrap();
        bank = &bank[(idx + 1)..];
        n = n * 10 + (*digit as u64);
    }
    n
}

fn main() {
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/03.txt").unwrap();
        assert_eq!(Some(357), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/03.txt").unwrap();
        assert_eq!(Some(3121910778619), part_two(&input));
    }
}