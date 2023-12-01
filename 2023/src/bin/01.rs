use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
            .map(|l| {
                format!("{}{}", l[0], l[l.len() - 1])
                    .parse::<u32>()
                    .unwrap()
            })
            .sum(),
    )
}

fn extract_digits(s: &str) -> Vec<u32> {
    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digits = Vec::new();
    for idx in 0..s.len() {
        let first_char = s.chars().nth(idx).unwrap();

        if first_char.is_numeric() {
            digits.push(first_char.to_digit(10).unwrap());
        } else {
            for i in 0..words.len() {
                if s[idx..].starts_with(words[i]) {
                    digits.push(i.try_into().unwrap());
                    break;
                }
            }
        }
    }
    digits
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| extract_digits(&l))
            .map(|d| {
                format!("{}{}", d[0], d[d.len() - 1])
                    .parse::<u32>()
                    .unwrap()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part_one(&input), Some(142));
    }

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits("js5df41qr1jks3hdv"), vec![5, 4, 1, 1, 3]);
        assert_eq!(
            extract_digits("one2three4five6seven8nine"),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(281));
    }
}
