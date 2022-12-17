use std::cmp::{max, min};
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut cave = load_map(input);
    let y_max: u32 = *(cave.iter().map(|(v, _)| v).max().unwrap()) + 2;

    let mut i: u32 = 0;
    while add_sand(&mut cave, (0, 500), false, y_max).is_some() {
        i += 1;
    }

    Some(i)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cave = load_map(input);
    let y_max: u32 = *(cave.iter().map(|(v, _)| v).max().unwrap()) + 2;
    let mut i: u32 = 0;

    loop {
        let rest_pos = add_sand(&mut cave, (0, 500), true, y_max);
        match rest_pos {
            Some((0, 500)) => return Some(i + 1),
            Some(_) => i += 1,
            None => return None,
        };
    }
}

fn can_go_to(map: &mut HashSet<(u32, u32)>, point: &(u32, u32), floor: bool, y_max: u32) -> bool {
    if floor {
        !map.contains(point) && point.0 < y_max
    } else {
        !map.contains(point)
    }
}

fn add_sand(
    map: &mut HashSet<(u32, u32)>,
    from: (u32, u32),
    floor: bool,
    y_max: u32,
) -> Option<(u32, u32)> {
    let (mut y, mut x) = from;
    while y <= y_max {
        if can_go_to(map, &(y + 1, x), floor, y_max) {
            y += 1;
        } else if can_go_to(map, &(y + 1, x - 1), floor, y_max) {
            y += 1;
            x -= 1;
        } else if can_go_to(map, &(y + 1, x + 1), floor, y_max) {
            y += 1;
            x += 1;
        } else {
            map.insert((y, x));
            return Some((y, x));
        }
    }
    None
}

fn load_map(input: &str) -> HashSet<(u32, u32)> {
    // Get paths
    let lines: Vec<Vec<(u32, u32)>> = input
        .lines()
        .map(|l| {
            l.split("->")
                .map(|w| w.trim().split_once(",").unwrap())
                .map(|w| (w.1.parse().unwrap(), w.0.parse().unwrap()))
                .collect()
        })
        .collect();

    // Fill map
    let mut points: HashSet<(u32, u32)> = HashSet::new();
    for line in lines {
        for i in 1..line.len() {
            let prev = line[i - 1];
            let curr = line[i];
            for y in min(prev.0, curr.0)..=max(prev.0, curr.0) {
                for x in min(prev.1, curr.1)..=max(prev.1, curr.1) {
                    points.insert((y, x));
                }
            }
        }
    }
    points
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
