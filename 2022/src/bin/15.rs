// Given the size of the input, I decided to clean the input files manually
// and just a have a list Sx,Sy,Bx,By

use core::cmp::{max, min};
use std::collections::HashSet;

struct SensorBeacon {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

pub fn part_one(input: &str) -> Option<u32> {
    let row = 2000000; // Use 2000000 to solve, 10 for tests
    let records = parse_input(input);

    let ranges = get_ranges(&records, row);

    let mut sum: u32 = 0;
    for (low, high) in ranges {
        sum += (high - low).abs() as u32 + 1;
    }

    let beacons = records
        .iter()
        .filter_map(|r| {
            if r.beacon.0 == row {
                Some(r.beacon.1)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .iter()
        .count() as u32;

    Some(sum - beacons)
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_coord = 4000000; // Use 20 for tests, 4000000 for solution
    let records = parse_input(input);

    for y in 0..=max_coord {
        let mut ranges: Vec<(i32, i32)> = get_ranges(&records, y)
            .iter()
            .map(|(l, h)| (*l, *h))
            .collect();
        ranges.sort();

        for i in 1..ranges.len() {
            let (_, h1) = ranges[i - 1];
            let (l2, _) = ranges[i];
            if l2 > h1 + 1 && (h1 + 1 < max_coord) && (h1 + 1 > 0) {
                return Some((h1 + 1) as u64 * 4000000 + y as u64);
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<SensorBeacon> {
    let records: Vec<SensorBeacon> = input
        .lines()
        .map(|l| {
            let vals: Vec<i32> = l.split(",").map(|v| v.parse().unwrap()).collect();
            SensorBeacon {
                sensor: (vals[1], vals[0]),
                beacon: (vals[3], vals[2]),
            }
        })
        .collect();
    records
}

fn get_ranges(records: &Vec<SensorBeacon>, row: i32) -> HashSet<(i32, i32)> {
    let mut ranges: HashSet<(i32, i32)> = HashSet::new();
    for sb in records {
        match get_range(sb, row) {
            Some(range) => add_range(&mut ranges, range),
            None => (),
        };
    }
    ranges
}

fn get_range(record: &SensorBeacon, row: i32) -> Option<(i32, i32)> {
    let sensor = record.sensor;
    let beacon = record.beacon;
    let x_steps = (sensor.1 - beacon.1).abs();
    let y_steps = (sensor.0 - beacon.0).abs();
    let total_steps = x_steps + y_steps;

    let y_delta = (sensor.0 - row).abs();
    if y_delta > total_steps {
        return None;
    }

    let remaining = total_steps - y_delta;
    Some((sensor.1 - remaining, sensor.1 + remaining))
}

fn add_range(ranges: &mut HashSet<(i32, i32)>, range: (i32, i32)) {
    let mut curr = range;
    ranges.insert(range);
    loop {
        let (curr_low, curr_high) = curr;
        let overlapping: Option<&(i32, i32)> = ranges
            .iter()
            .filter(|(l, u)| (curr != (*l, *u)) && curr_low <= *u && curr_high >= *l)
            .next();
        if overlapping.is_none() {
            break;
        }
        let overlapping = overlapping.unwrap().clone();

        let (other_low, other_high) = overlapping;
        ranges.remove(&overlapping);
        ranges.remove(&curr);

        let new_range = (min(curr_low, other_low), max(curr_high, other_high));
        ranges.insert(new_range);
        curr = new_range;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
