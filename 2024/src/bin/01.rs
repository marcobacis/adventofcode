use std::fs;
use std::iter::zip;
use std::str::FromStr;

fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(n1, n2)| (i32::from_str(n1).unwrap(), i32::from_str(n2).unwrap()))
                .unwrap()
        })
        .unzip();

    left.sort();
    right.sort();

    Some(
        zip(left, right)
            .map(|(n1, n2)| (n1 - n2).abs())
            .sum::<i32>() as u32,
    )
}

fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(n1, n2)| (i32::from_str(n1).unwrap(), i32::from_str(n2).unwrap()))
                .unwrap()
        })
        .unzip();

    Some(
        left.iter()
            .map(|l| (*l as usize) * right.iter().filter(|r| l == *r).count())
            .sum::<usize>() as u32,
    )
}

fn main() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(Some(11), part_one(input));
    }

    #[test]
    fn part_two_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(Some(31), part_two(input));
    }
}
