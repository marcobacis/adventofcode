use std::fs;

#[derive(Clone, Copy, Debug)]
struct Range {
    low: u64,
    high: u64,
}

impl Range {
    pub fn size(&self) -> u64 {
        self.high - self.low + 1
    }

    pub fn contains(&self, n: u64) -> bool {
        n >= self.low && n <= self.high
    }

    pub fn overlap(&self, other: &Range) -> bool {
        self.low <= other.high && other.low <= self.high
    }

    pub fn merge(&self, other: &Range) -> Range {
        Range {
            low: if self.low < other.low {
                self.low
            } else {
                other.low
            },
            high: if self.high > other.high {
                self.high
            } else {
                other.high
            },
        }
    }
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut is_range = true;
    let mut ranges = vec![];
    let mut ingredients = vec![];
    for line in input.lines() {
        if line.is_empty() {
            is_range = false;
            continue;
        }

        if is_range {
            let (start, end) = line.split_once("-").unwrap();
            ranges.push(Range {
                low: start.parse().unwrap(),
                high: end.parse().unwrap(),
            });
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    (ranges, ingredients)
}

fn part_one(input: &str) -> Option<u32> {
    let (ranges, ingredients) = parse(input);

    Some(
        ingredients
            .iter()
            .filter(|i| ranges.iter().any(|r| r.contains(**i)))
            .count() as u32,
    )
}

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    // Assume the vec is sorted by Range.low
    let mut res = vec![];

    let mut prev = ranges[0];
    for i in 1..ranges.len() {
        let curr = ranges[i];
        if prev.overlap(&curr) {
            prev = prev.merge(&curr);
        } else {
            res.push(prev);
            prev = curr;
        }
    }
    res.push(prev);

    res.sort_by_key(|r| r.low);
    res
}

fn part_two(input: &str) -> Option<u64> {
    let mut ranges = parse(input).0;
    ranges.sort_by_key(|r| r.low);
    let mut prev_size = ranges.len();

    ranges = merge_ranges(&ranges);
    while ranges.len() != prev_size {
        prev_size = ranges.len();
        ranges = merge_ranges(&ranges);
    }

    Some(
        ranges
            .iter()
        .map(|r| {
                r.size()
            })
            .sum::<u64>() as u64,
    )
}

fn main() {
    let input = fs::read_to_string("inputs/05.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(Some(3), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(Some(14), part_two(&input));
    }
}
