struct Move {
    n: usize,
    from: usize,
    to: usize,
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = read_stacks(input);
    let moves = read_moves(input);

    for m in moves {
        for _ in 0..m.n {
            let val = stacks[m.from].pop().unwrap();
            stacks[m.to].push(val);
        }
    }

    return create_output(&stacks);
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks = read_stacks(input);
    let moves = read_moves(input);

    for m in moves {
        let idx = stacks[m.from].len() - m.n;
        let mut slice = stacks[m.from].split_off(idx);
        stacks[m.to].append(&mut slice);
    }

    return create_output(&stacks);
}

fn read_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Get all "stack" lines (not empty and not staring with move)
    // and remove the last (the one with indices)
    let input: Vec<&str> = input
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with("move"))
        .collect::<Vec<&str>>()
        .split_last()
        .unwrap()
        .1
        .to_vec();

    // Setup stacks
    for _ in 0..(input[0].len() / 4 + 1) {
        stacks.push(Vec::new());
    }

    // Read all stacks line by line (from bottom to top)
    for line in input.iter().rev() {
        let line: Vec<char> = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c[1])
            .collect();

        for (i, c) in line.iter().enumerate() {
            if *c != ' ' {
                stacks[i].push(*c);
            }
        }
    }

    return stacks;
}

fn read_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter(|l| l.starts_with("move"))
        .map(|l| {
            let split: Vec<_> = l.split(' ').map(str::to_owned).collect();
            Move {
                n: split[1].parse().unwrap(),
                from: split[3].parse::<usize>().unwrap() - 1,
                to: split[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

fn create_output(stacks: &Vec<Vec<char>>) -> Option<String> {
    let mut output = String::new();
    for stack in stacks {
        if stack.len() > 0 {
            output.push(stack[stack.len() - 1]);
        }
    }
    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
