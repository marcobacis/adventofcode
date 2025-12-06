use std::fs;

fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().collect();
    let nums: Vec<Vec<u64>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operations: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let cols = operations.len();

    let res = (0..cols)
        .filter_map(|idx| {
            match operations[idx] {
                '+' => Some(nums.iter().map(|l| l[idx]).sum()),
                '*' => Some(nums.iter().map(|l| l[idx]).product::<u64>()),
                _ => None
            }
        })
        .sum();

    Some(res)
}

fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let mut sum = 0;

    let mut numbers: Vec<u64> = vec![];
    for idx in (0..lines[0].len()).rev() {
        let n = parse_num_col(&lines, idx);
        if let Some(n) = n {
            numbers.push(n);
        }

        let res = match lines[lines.len() - 1][idx] {
            b'*' => Some(numbers.iter().product::<u64>()),
            b'+' => Some(numbers.iter().sum()),
            _ => None,
        };

        if let Some(res) = res {
            sum += res;
            numbers.clear();
        }
    }

    Some(sum)
}

fn parse_num_col(lines: &Vec<&[u8]>, idx: usize) -> Option<u64> {
    if lines.iter().all(|l| l[idx].is_ascii_whitespace()) {
        None
    } else {
        Some(
            lines
                .iter()
                .take(lines.len() - 1)
                .filter_map(|l| {
                    if l[idx] == b' ' {
                        None
                    } else {
                        Some((l[idx] - b'0') as u64)
                    }
                })
                .rev()
                .enumerate()
                .map(|(idx, d)| d * 10_u64.pow(idx as u32))
                .sum(),
        )
    }
}

fn main() {
    let input = fs::read_to_string("inputs/06.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(Some(4277556), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(Some(3263827), part_two(&input));
    }
}
