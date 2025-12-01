use std::fs;

fn part_one(input: &str) -> Option<u32> {
    None
}

fn part_two(input: &str) -> Option<u32> {
    None
}


fn main() {
    let input = fs::read_to_string("inputs/{DAY}.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
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