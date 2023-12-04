use std::{cmp::min, collections::HashSet};

struct Card {
    number: usize,
    matches: usize,
}

impl Card {
    pub fn new(line: &str) -> Self {
        let mut split = line.split(':');
        let number = split
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let (winning_nums, mine_nums) = split.next().unwrap().split_once('|').unwrap();

        let winning: HashSet<usize> = HashSet::from_iter(get_numbers(winning_nums));
        let mine: HashSet<usize> = HashSet::from_iter(get_numbers(mine_nums));
        let matches = winning.intersection(&mine).count();

        Self { number, matches }
    }

    pub fn points(&self) -> usize {
        if self.matches >= 1 {
            (2 as usize).pow((self.matches - 1) as u32)
        } else {
            0 as usize
        }
    }
}

fn get_numbers(str: &str) -> Vec<usize> {
    str.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| Card::new(l))
            .map(|card| card.points())
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let matches: Vec<usize> = input.lines().map(|l| Card::new(l).matches).collect();

    let mut cards: Vec<usize> = vec![1; matches.len()];

    for i in 0..matches.len() - 1 {
        let start = min(i + 1, matches.len() - 1);
        let end = min(i + 1 + matches[i], matches.len());

        for j in start..end {
            cards[j] += cards[i];
        }
    }

    Some(cards.iter().sum::<usize>() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(30));
    }
}
