use num::integer::lcm;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: char,
    operand: String,
    divisible: u32,
    throw_true: usize,
    throw_false: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 20, true))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 10000, false))
}

fn solve(input: &str, rounds: u32, part1: bool) -> u64 {
    let mut monkeys = parse_input(input);

    let mut inspections: Vec<u64> = Vec::new();
    for _ in &monkeys {
        inspections.push(0);
    }

    // For part2, to "keep the worry levels acceptable" we modulo by the lowest common multiple
    // of the "divisible by" numbers.
    let base = monkeys
        .iter()
        .fold(1, |acc, m| lcm(acc, m.divisible as u64));

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // Ugly solution to avoid reference/borrowing issues
            let monkey = monkeys[i].clone();

            for item in monkey.items.iter() {
                inspections[i] += 1;
                // Update worry level
                let operand: u64 = match monkey.operand.as_str() {
                    "old" => *item,
                    _ => monkey.operand.parse().unwrap(),
                };

                let new: u64 = match monkey.operation {
                    '*' => item * operand,
                    '+' => item + operand,
                    '-' => item - operand,
                    '/' => item / operand,
                    _ => *item,
                };

                let new = if part1 { new / 3 } else { new % base };

                let throw_idx = if new % monkey.divisible as u64 == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };

                // Update current and "thrown" monkey
                monkeys[i].items.clear();
                monkeys[throw_idx as usize].items.push(new);
            }
        }
    }
    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

// Very ugly parsing code
fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    for chunk in lines.chunks(7) {
        let items: Vec<u64> = chunk[1]
            .rsplit_once(':')
            .unwrap()
            .1
            .split(",")
            .map(|i| i.trim().parse().unwrap())
            .collect();

        let op_line: Vec<&str> = chunk[2].split_ascii_whitespace().collect();
        let operand = String::from(op_line[op_line.len() - 1]);
        let operation = op_line[op_line.len() - 2].chars().collect::<Vec<char>>()[0];

        let divisible: u32 = chunk[3]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let throw_true = chunk[4]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let throw_false = chunk[5]
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let monkey = Monkey {
            items,
            operation,
            operand,
            divisible,
            throw_true,
            throw_false,
        };
        monkeys.push(monkey);
    }

    monkeys
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
