/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn get_numbers(str: &str) -> Vec<usize> {
    str.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}
