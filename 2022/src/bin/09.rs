use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 10))
}

fn solve(input: &str, rope_size: usize) -> u32 {
    // (cmd, n)
    let commands = read_commands(&input);

    // (y,x)
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_size];
    for (cmd, n) in commands {
        for _ in 0..n {
            let tail = move_rope(&mut rope, cmd);
            positions.insert(tail);
        }
    }

    positions.len() as u32
}

fn read_commands(input: &str) -> Vec<(&str, i32)> {
    let commands: Vec<(&str, i32)> = input
        .lines()
        .map(|l| {
            let split = l.split_once(" ").unwrap();
            (split.0, split.1.parse().unwrap())
        })
        .collect();

    commands
}

fn move_rope(rope: &mut Vec<(i32, i32)>, cmd: &str) -> (i32, i32) {
    // Move head
    rope[0] = match cmd {
        "R" => (rope[0].0, rope[0].1 + 1),
        "L" => (rope[0].0, rope[0].1 - 1),
        "U" => (rope[0].0 + 1, rope[0].1),
        "D" => (rope[0].0 - 1, rope[0].1),
        _ => rope[0],
    };

    // Move all other points
    for i in 1..rope.len() {
        rope[i] = move_point(rope[i - 1], rope[i]);
    }

    // Return tail position
    rope[rope.len() - 1]
}

fn move_point(ahead: (i32, i32), point: (i32, i32)) -> (i32, i32) {
    let distv = ahead.0 - point.0;
    let disth = ahead.1 - point.1;
    if distv.abs() > 1 || disth.abs() > 1 {
        (point.0 + distv.signum(), point.1 + disth.signum())
    } else {
        point
    }
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
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
