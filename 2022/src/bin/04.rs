use regex::Regex;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    return Some(
        input
            .lines()
            .map(|l| parse_line(l))
            .filter(|(a, b)| a.is_subset(b) || b.is_subset(a))
            .count() as u32,
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(
        input
            .lines()
            .map(|l| parse_line(l))
            .filter(|(a, b)| a.intersection(b).count() > 0)
            .count() as u32,
    );
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let caps = re.captures(line).expect("not matched!");

    let a: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let b: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let c: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
    let d: u32 = caps.get(4).unwrap().as_str().parse().unwrap();

    return ((a..=b).collect(), (c..=d).collect());
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
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
