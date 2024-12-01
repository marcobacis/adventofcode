use std::str::FromStr;
use std::iter::zip;
use std::env;
use std::fs;

fn part_one(input: &str) -> Option<u32> {
    None
}

fn part_two(input: &str) -> Option<u32> {
    None
}


fn main() {
    let input = fs::read_to_string("inputs/{DAY}.txt").unwrap();

    println!("Solutions 🎄");
    let result_part_one = part_one(&input);
    let result_part_two = part_two(&input);

    if let Some(res) = result_part_one {
        println!("Part 1: {}", res);
    }
    if let Some(res) = result_part_two {
        println!("Part 2: {}", res);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/{DAY}.txt").unwrap();
        assert_eq!(None, part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/{DAY}.txt").unwrap();
        assert_eq!(None, part_two(&input));
    }

}