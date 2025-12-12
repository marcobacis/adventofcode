use std::fs;

fn part_one(input: &str) -> Option<u32> {
    // Parse shapes areas and tree requirements
    let mut lines = input.lines();
    let mut areas: Vec<u64> = vec![];
    let mut trees: Vec<(u64, Vec<u64>)> = vec![];
    while let Some(line) = lines.next() {
        if line.contains("x") {
            let mut terms= line
                .split(|c: char| !c.is_ascii_digit())
                .filter_map(|s| if !s.is_empty() {s.parse().ok()} else {None});
            trees.push((terms.next().unwrap() * terms.next().unwrap(), terms.collect()));
            continue;
        }

        if line.contains(":") {
            let area = (0..3).map(|_| lines.next().unwrap())
                .map(|l| l.as_bytes().iter().filter(|c| **c == b'#').count() as u64)
                .sum();
            areas.push(area);
        }
    }

    // Count only trees that can fit the presents
    let count = trees
        .iter()
        .filter(|(area, shapes)| {
            let required = shapes.iter().zip(&areas).map(|(n, a)| (*n) * a).sum::<u64>();

            *area >= required
        })
        .count();

    Some(count as u32)
}

fn main() {
    let input = fs::read_to_string("inputs/12.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
}