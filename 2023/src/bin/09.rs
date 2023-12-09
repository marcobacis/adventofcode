fn differences(input: &Vec<i32>) -> Vec<i32> {
    (1..input.len()).map(|i| input[i] - input[i - 1]).collect()
}

fn generate_differences_list(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs: Vec<Vec<i32>> = vec![input];

    // Generate triangle
    loop {
        diffs.push(differences(&diffs[diffs.len() - 1]));
        if diffs[diffs.len() - 1].iter().all(|v| *v == 0) {
            break;
        }
    }
    diffs
}

fn next_value(input: Vec<i32>) -> i32 {
    let mut diffs = generate_differences_list(input);
    let num_lines = diffs.len();

    diffs[num_lines - 1].push(0);

    for i in (0..(num_lines - 1)).rev() {
        let new_val = diffs[i + 1].last().unwrap() + diffs[i].last().unwrap();
        diffs[i].push(new_val);
    }

    *diffs[0].last().unwrap()
}

fn prev_value(input: Vec<i32>) -> i32 {
    let mut diffs = generate_differences_list(input);
    let num_lines = diffs.len();

    diffs[num_lines - 1].push(0);

    for i in (0..(num_lines - 1)).rev() {
        let new_val = diffs[i][0] - diffs[i + 1][0];
        diffs[i].insert(0, new_val);
    }

    diffs[0][0]
}

pub fn part_one(input: &str) -> Option<i32> {
    let sum = input
        .lines()
        .map(|l| advent_of_code::helpers::get_numbers_signed(l))
        .map(|v| next_value(v))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let sum = input
        .lines()
        .map(|l| advent_of_code::helpers::get_numbers_signed(l))
        .map(|v| prev_value(v))
        .sum();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(114));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(2));
    }
}
