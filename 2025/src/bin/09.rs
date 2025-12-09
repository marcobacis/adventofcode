use std::{
    cmp,
    fs::{self},
};

use advent_of_code::coordinate::Coordinate;

fn parse(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            let split = l.split_once(',').unwrap();
            Coordinate::new(
                split.1.parse::<i32>().unwrap(),
                split.0.parse::<i32>().unwrap(),
            )
        })
        .collect()
}

struct Rectangle {
    a: Coordinate,
    b: Coordinate,
}

impl Rectangle {
    pub fn area(&self) -> u64 {
        (self.a.y.abs_diff(self.b.y) as u64 + 1) * (self.a.x.abs_diff(self.b.x) as u64 + 1)
    }

    pub fn contains(&self, la: &Coordinate, lb: &Coordinate) -> bool {
        let left = cmp::min(self.a.x, self.b.x);
        let right = cmp::max(self.a.x, self.b.x);
        let up = cmp::min(self.a.y, self.b.y);
        let down = cmp::max(self.a.y, self.b.y);

        // Assumption: the lines can be only vertical or horizontal
        let outside = (la.x <= left && lb.x <= left)
            || (la.x >= right && lb.x >= right)
            || (la.y <= up && lb.y <= up)
            || (la.y >= down && lb.y >= down);

        !outside
    }
}

fn rectangles(coords: &[Coordinate]) -> impl Iterator<Item = Rectangle> + '_ {
    (0..coords.len()).flat_map(move |i| {
        (0..coords.len()).map(move |j| Rectangle {
            a: coords[i],
            b: coords[j],
        })
    })
}

fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input);

    rectangles(&coords).map(|rect| rect.area()).max()
}

fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input);

    let mut lines = vec![];
    for i in 0..(coords.len() - 1) {
        lines.push((coords[i], coords[i + 1]));
    }
    lines.push((coords[coords.len() - 1], coords[0]));

    rectangles(&coords)
        .filter(|rect| lines.iter().all(|l| !rect.contains(&l.0, &l.1)))
        .map(|rect| rect.area())
        .max()
}

fn main() {
    let input = fs::read_to_string("inputs/09.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(Some(50), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(Some(24), part_two(&input));
    }
}
