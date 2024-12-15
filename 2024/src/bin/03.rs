use regex::Regex;
use std::fs;

fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results: Vec<(u32, u32)> = vec![];
    for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()));
    }

    Some(results.iter().map(|(n1, n2)| n1 * n2).sum())
}

fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(don't\(\)|do\(\)|mul\(([0-9]+),([0-9]+)\))").unwrap();

    let mut results: Vec<(u32, u32)> = vec![];
    let mut enabled = true;

    for c in re.captures_iter(input) {
        match c.get(0).map_or("", |m| m.as_str()) {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            &_ => {
                if enabled {
                    results.push((
                        c.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                        c.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                    ))
                }
            }
        };
    }
    Some(results.iter().map(|(n1, n2)| n1 * n2).sum())
}

fn main() {
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/03.txt").unwrap();
        assert_eq!(Some(161), part_one(&input));
    }

    #[rstest]
    #[case("don't()mul(3,2)do()", Some(0))]
    #[case("don't()do()mul(3,2)do()", Some(6))]
    fn test_part_two_cases(#[case] input: &str, #[case] res: Option<u32>) {
        assert_eq!(res, part_two(input));
    }

    #[test]
    fn part_two_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(Some(48), part_two(input));
    }
}
