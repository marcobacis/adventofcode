use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let split = line.split_at(line.len() / 2);
        let first_set: HashSet<char> = split.0.chars().collect();
        let second_set: HashSet<char> = split.1.chars().collect();

        let common = &first_set & &second_set;
        let common: Vec<&char> = common.iter().collect();
        let common: u32 = *(common[0]) as u32;

        if common >= 'a' as u32 && common <= 'z' as u32 {
            sum += common - ('a' as u32) + 1;
        } else {
            sum += common - ('A' as u32) + 27;
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for chunk in lines.chunks(3) {
        let first_set: HashSet<char> = chunk[0].chars().collect();
        let second_set: HashSet<char> = chunk[1].chars().collect();
        let third_set: HashSet<char> = chunk[2].chars().collect();

        let common: HashSet<char> = &(&first_set & &second_set) & &third_set;
        let common: Vec<&char> = common.iter().collect();
        let common: u32 = *(common[0]) as u32;

        if common >= 'a' as u32 && common <= 'z' as u32 {
            sum += common - ('a' as u32) + 1;
        } else {
            sum += common - ('A' as u32) + 27;
        }
    }
    return Some(sum);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
