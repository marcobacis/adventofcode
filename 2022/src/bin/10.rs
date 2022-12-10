pub fn part_one(input: &str) -> Option<i32> {
    let signals = get_signals(input);

    let indexes: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut sum: i32 = 0;
    for idx in indexes {
        sum += idx * signals[idx as usize - 1];
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let signals = get_signals(input);

    let mut grid = [['.' as char; 40]; 6];
    for (i, signal) in signals.iter().enumerate() {
        let x = i % 40;
        let y = i / 40;
        if (x as i32 - signal).abs() < 2 {
            grid[y][x] = '#';
        }
    }

    let mut result: String = String::new();
    for y in 0..6 {
        for x in 0..40 {
            result.push(grid[y][x]);
        }
        result.push('\n');
    }

    Some(result)
}

fn get_signals(input: &str) -> Vec<i32> {
    let mut signals: Vec<i32> = Vec::new();
    let mut x: i32 = 1;
    signals.push(x);

    for line in input.lines() {
        if line.contains("noop") {
            signals.push(x);
        } else {
            let n: i32 = line.split_once(" ").unwrap().1.parse().unwrap();
            signals.push(x);
            x += n;
            signals.push(x);
        }
    }
    signals
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let expected: String = String::from(
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n",
        );
        assert_eq!(part_two(&input), Some(expected));
    }
}
