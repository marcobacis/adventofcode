use std::{collections::HashMap, fs};

use cached::proc_macro::cached;

fn part_one(input: &str) -> Option<u32> {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let (node, outputs) = l.split_once(": ").unwrap();
            let outputs: Vec<&str> = outputs.split(" ").collect();
            (node, outputs)
        })
        .collect();

    let mut stack = vec!["you"];
    let mut count = 0;

    while let Some(curr) = stack.pop() {
        if curr == "out" {
            count += 1;
            continue;
        }

        for node in graph[curr].clone() {
            stack.push(node);
        }
    }

    Some(count)
}

#[cached(
    key = "(String, bool, bool)",
    convert = r#"{ (curr.clone(), visited_dac, visited_fft) }"#
)]
fn dfs_part_2(
    graph: &HashMap<String, Vec<String>>,
    curr: String,
    visited_dac: bool,
    visited_fft: bool,
) -> u64 {
    if curr == "out" {
        if visited_dac && visited_fft {
            return 1;
        }
        return 0;
    }

    graph[curr.as_str()]
        .iter()
        .map(|node| {
            if node == "dac" {
                dfs_part_2(graph, node.clone(), true, visited_fft)
            } else if node == "fft" {
                dfs_part_2(graph, node.clone(), visited_dac, true)
            } else {
                dfs_part_2(graph, node.clone(), visited_dac, visited_fft)
            }
        })
        .sum()
}

fn part_two(input: &str) -> Option<u64> {
    let graph: HashMap<String, Vec<String>> = input
        .lines()
        .map(|l| {
            let (node, outputs) = l.split_once(":").unwrap();
            let outputs: Vec<String> = outputs.split_whitespace().map(|s| s.to_string()).collect();
            (node.to_string(), outputs)
        })
        .collect();

    Some(dfs_part_2(&graph, String::from("svr"), false, false))
}

fn main() {
    let input = fs::read_to_string("inputs/11.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/11_1.txt").unwrap();
        assert_eq!(Some(5), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/11_2.txt").unwrap();
        assert_eq!(Some(2), part_two(&input));
    }
}
