pub fn part_one(input: &str) -> Option<u32> {
    let sizes = get_sizes(input);
    let max_size: usize = 100000;
    let sum = sizes
        .iter()
        .filter(|s| s < &&max_size)
        .fold(0, |acc, s| acc + s);

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sizes = get_sizes(input);
    let disk_usage: usize = *sizes.iter().max_by_key(|s| *s).unwrap();
    let min_size = 30000000 - (70000000 - disk_usage);

    let size = sizes
        .iter()
        .filter(|s| s > &&min_size)
        .min_by_key(|s| *s)
        .unwrap();

    Some(*size as u32)
}

fn get_sizes(input: &str) -> Vec<usize> {
    // Idea / Copied from @lhecker solution (https://github.com/lhecker/adventofcode_2022/blob/master/src/day07/main.rs)
    let mut sizes: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$") {
            if let Some(dir) = line.strip_prefix("$ cd ") {
                // cd .. -> add new dir size and to stack if not root
                if dir == ".." {
                    let size = stack.pop().unwrap_or(0);
                    sizes.push(size);
                    if let Some(upper) = stack.last_mut() {
                        *upper += size;
                    }
                } else {
                    // cd <name>, add a new dir to the stack (with 0 size for now)
                    stack.push(0);
                }
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            // Line inside ls indicating file size, add to last stack entry
            // <size> <name>
            let (size, _) = line.split_once(" ").unwrap();
            if let Some(last) = stack.last_mut() {
                *last += size.parse::<usize>().unwrap();
            }
        }
    }

    let mut last_size: usize = 0;
    while let Some(last) = stack.pop() {
        last_size += last;
        sizes.push(last_size);
    }

    return sizes;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
