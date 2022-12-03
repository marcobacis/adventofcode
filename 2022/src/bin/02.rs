pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        sum += compute_points_1(line);
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        sum += compute_points_2(line);
    }
    return Some(sum);
}

fn compute_points_1(line: &str) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let me = (chars[2].to_ascii_uppercase() as u32) - ('X' as u32);
    let other = (chars[0].to_ascii_uppercase() as u32) - ('A' as u32);

    let mut points = me + 1;
    if me == other {
        points += 3;
    } else if wins(me, other) {
        points += 6;
    }

    return points;
}

fn wins(me: u32, other: u32) -> bool {
    // 0 = Rock, 1 = Paper, 2 = Scissors
    return me == (other + 1).rem_euclid(3);
}

fn compute_points_2(line: &str) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let other = (chars[0].to_ascii_uppercase() as u32) - ('A' as u32);
    let result: char = chars[2];

    let result_points: u32 = match result {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    let result_dist: i32 = match result {
        'X' => -1,
        'Y' => 0,
        'Z' => 1,
        _ => 0,
    };

    let me: u32 = ((other as i32 + result_dist).rem_euclid(3)) as u32;

    println!("\t{me} + 1 + {result_points}");

    return me + 1 + result_points;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
