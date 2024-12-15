use std::{collections::HashSet, fs};

use advent_of_code::coordinate::Coordinate;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Limits {
    height: u32,
    width: u32,
}

impl Limits {
    pub fn constrain(&self, coord: &Coordinate) -> Coordinate {
        let y = coord.y;
        let x = coord.x;
        let h = self.height as i32;
        let w = self.width as i32;

        let x = ((x % w) + w) % w;
        let y = ((y % h) + h) % h;

        Coordinate::new(y, x)
    }

    pub fn quadrants(&self) -> Vec<Quadrant> {
        let w = self.width as i32;
        let h = self.height as i32;
        vec![
            Quadrant {
                start: Coordinate::new(0, 0),
                end: Coordinate::new(h / 2, w / 2),
            },
            Quadrant {
                start: Coordinate::new(0, w / 2 + 1),
                end: Coordinate::new(h / 2, w),
            },
            Quadrant {
                start: Coordinate::new(h / 2 + 1, w / 2 + 1),
                end: Coordinate::new(h, w),
            },
            Quadrant {
                start: Coordinate::new(h / 2 + 1, 0),
                end: Coordinate::new(h, w / 2),
            },
        ]
    }
}

#[derive(Debug)]
struct Robot {
    position: Coordinate,
    velocity: Coordinate,
}

impl Robot {
    pub fn parse(line: &str) -> Self {
        let re = Regex::new(r"-?\d+").unwrap();
        let numbers: Vec<i32> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse::<i32>().ok())
            .collect();

        Self {
            position: Coordinate::new(numbers[1], numbers[0]),
            velocity: Coordinate::new(numbers[3], numbers[2]),
        }
    }

    pub fn step(&self, limits: &Limits) -> Self {
        let new_coordinate = limits.constrain(&(self.position + self.velocity));
        Self {
            position: new_coordinate,
            velocity: self.velocity,
        }
    }

    pub fn is_in(&self, quadrant: &Quadrant) -> bool {
        self.position.x >= quadrant.start.x
            && self.position.x < quadrant.end.x
            && self.position.y >= quadrant.start.y
            && self.position.y < quadrant.end.y
    }
}

#[derive(Debug)]
struct Quadrant {
    start: Coordinate,
    end: Coordinate,
}

fn part_one_helper(input: &str, limits: Limits) -> Option<u32> {
    let initial: Vec<Robot> = input.lines().map(Robot::parse).collect();
    let robots = (0..100).fold(initial, |acc, _| {
        acc.iter().map(|r| r.step(&limits)).collect()
    });
    Some(
        limits
            .quadrants()
            .iter()
            .map(|quadrant| robots.iter().filter(|r| r.is_in(quadrant)).count() as u32)
            .product(),
    )
}

fn part_one(input: &str) -> Option<u32> {
    part_one_helper(
        input,
        Limits {
            height: 103,
            width: 101,
        },
    )
}

fn print(step: u32, robots: &Vec<Robot>, limits: &Limits) {
    let positions: HashSet<Coordinate> = HashSet::from_iter(robots.iter().map(|r| r.position));

    // Clear the screen and move the cursor to the top-left
    print!("\x1b[2J\x1b[H");

    println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");

    let density = density(robots);

    println!("Step {} - density {}", step, density);

    for y in 0..limits.height {
        for x in 0..limits.width {
            if positions.contains(&Coordinate::new(y as i32, x as i32)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("------------------------------------------------------------------------------------------------------------------------------------------------------------");
}

fn density(robots: &Vec<Robot>) -> u32 {
    let positions: HashSet<Coordinate> = HashSet::from_iter(robots.iter().map(|r| r.position));
    positions
        .iter()
        .map(|p| p.around().iter().filter(|c| positions.contains(*c)).count() as u32)
        .sum()
}

fn part_two(input: &str) -> Option<u32> {
    let limits = Limits {
        height: 103,
        width: 101,
    };
    let initial: Vec<Robot> = input.lines().map(Robot::parse).collect();

    let mut robots: Vec<Robot> = initial.iter().map(|r| r.step(&limits)).collect();

    let mut scores: Vec<(u32, u32)> = vec![];

    // Tree seen in the first 10000 steps as a small/dense picture in a frame.
    // Search for the step with the most density (number of positions with adjacent positions)
    for step in 0..10000 {
        scores.push((step, density(&robots)));
        robots = robots.iter().map(|r| r.step(&limits)).collect();
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    Some(scores[0].0 + 1)
}

fn main() {
    let input = fs::read_to_string("inputs/14.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/14.txt").unwrap();
        assert_eq!(
            Some(12),
            part_one_helper(
                &input,
                Limits {
                    height: 7,
                    width: 11
                }
            )
        );
    }
}
