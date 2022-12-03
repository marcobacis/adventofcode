use std::thread::current;

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_sum: u32 = 0;
    let mut max_sum: u32 = 0;

    let lines = input.lines();
    for line in lines {
        if line.is_empty() {
            max_sum = if current_sum > max_sum {
                current_sum
            } else {
                max_sum
            };
            current_sum = 0;
        } else {
            current_sum += line.trim().parse::<u32>().expect("not a number");
        }
    }

    max_sum = if (current_sum > max_sum) {
        current_sum
    } else {
        max_sum
    };

    return Some(max_sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sums: Vec<u32> = Vec::new();

    let lines = input.lines();
    let mut current_sum: u32 = 0;
    for line in lines {
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.trim().parse::<u32>().expect("not a number");
        }
    }
    sums.push(current_sum);

    sums.sort();
    sums.reverse();

    return Some(sums[0] + sums[1] + sums[2]);
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
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
