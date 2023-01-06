use std::collections::HashMap;

#[derive(Default, Debug)]
struct Node<'a> {
    name: &'a str,
    operation: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> HashMap<&str, Node> {
    let nodes: HashMap<&str, Node> = input
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once(":").unwrap();
            let fields: Vec<_> = rest.trim().split_whitespace().collect();

            if fields.len() == 1 {
                (
                    name,
                    Node {
                        name,
                        operation: fields[0],
                        left: "",
                        right: "",
                    },
                )
            } else {
                (
                    name,
                    Node {
                        name,
                        operation: fields[1],
                        left: fields[0],
                        right: fields[2],
                    },
                )
            }
        })
        .collect();
    nodes
}

fn compute(nodes: &HashMap<&str, Node>, name: &str) -> i64 {
    let node = &nodes[name];

    match node.operation {
        "+" => compute(nodes, node.left) + compute(nodes, node.right),
        "*" => compute(nodes, node.left) * compute(nodes, node.right),
        "-" => compute(nodes, node.left) - compute(nodes, node.right),
        "/" => compute(nodes, node.left) / compute(nodes, node.right),
        _ => node.operation.parse::<i64>().unwrap(),
    }
}

fn has_human(nodes: &HashMap<&str, Node>, name: &str) -> bool {
    let node = &nodes[name];
    if name == "humn" {
        return true;
    }

    match node.operation {
        "+" => has_human(nodes, node.left) || has_human(nodes, node.right),
        "*" => has_human(nodes, node.left) || has_human(nodes, node.right),
        "-" => has_human(nodes, node.left) || has_human(nodes, node.right),
        "/" => has_human(nodes, node.left) || has_human(nodes, node.right),
        _ => false,
    }
}

// Solve ... x ... = value
fn solve(nodes: &HashMap<&str, Node>, name: &str, value: i64) -> i64 {
    let node = &nodes[name];

    let operation = if node.name == "root" {
        "="
    } else {
        node.operation
    };

    // Final case, humn op right = value, or left op humn = value
    if node.left == "humn" {
        match operation {
            "+" => value - compute(nodes, node.right),
            "-" => value + compute(nodes, node.right),
            "*" => value / compute(nodes, node.right),
            "/" => value * compute(nodes, node.right),
            "=" => compute(nodes, node.right),
            _ => panic!("Unexpected operation"),
        }
    } else if node.right == "humn" {
        match operation {
            "+" => value - compute(nodes, node.right),
            "-" => compute(nodes, node.right) - value,
            "*" => value / compute(nodes, node.right),
            "/" => compute(nodes, node.right) / value,
            "=" => compute(nodes, node.right),
            _ => panic!("Unexpected operation"),
        }
    } else {
        // General case confront left tree and right tree and propagate value on the humn branch
        if has_human(nodes, node.left) {
            let right_val = compute(nodes, node.right);
            let new_value = match operation {
                "+" => value - right_val,
                "-" => value + right_val,
                "*" => value / right_val,
                "/" => value * right_val,
                "=" => right_val,
                _ => panic!("Unexpected operation"),
            };
            solve(nodes, node.left, new_value)
        } else {
            let left_val = compute(nodes, node.left);
            let new_value = match operation {
                "+" => value - left_val,
                "-" => left_val - value,
                "*" => value / left_val,
                "/" => left_val / value,
                "=" => left_val,
                _ => panic!("Unexpected operation"),
            };
            solve(nodes, node.right, new_value)
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let nodes = parse(input);
    Some(compute(&nodes, "root"))
}

pub fn part_two(input: &str) -> Option<i64> {
    let nodes = parse(input);
    Some(solve(&nodes, "root", 0))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
