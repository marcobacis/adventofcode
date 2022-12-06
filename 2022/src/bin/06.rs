use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn solve(input: &str, n: usize) -> Option<u32> {
    let start_idx = n - 1;
    let chars: Vec<char> = input.chars().collect();

    for i in start_idx..chars.len() {
        let last_chars: HashSet<&char> = chars[(i - start_idx)..(i + 1)].iter().collect();
        if last_chars.len() == n {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
