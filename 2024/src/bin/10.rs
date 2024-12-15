use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use advent_of_code::{coordinate::Coordinate, grid::Grid};

fn get_starting_coordinates(grid: &Grid<i8>, c: i8) -> Vec<Coordinate> {
    let mut coordinates = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Coordinate::new(y as i32, x as i32);
            if *grid.get(&coord).unwrap() == c {
                coordinates.push(coord);
            }
        }
    }
    coordinates
}

// BFS, see how many 9s are reachable from any 0
fn score(grid: &Grid<i8>, start: &Coordinate) -> u32 {
    let directions = [
        Coordinate::new(-1, 0),
        Coordinate::new(0, -1),
        Coordinate::new(0, 1),
        Coordinate::new(1, 0),
    ];

    let mut q: VecDeque<Coordinate> = VecDeque::new();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut score = 0;

    visited.insert(*start);
    q.push_back(*start);

    while !q.is_empty() {
        let current = q.pop_front().unwrap();

        let current_val = grid.get(&current).unwrap();
        if *current_val == 9 {
            score += 1;
        }

        let neighbours: Vec<Coordinate> = directions
            .iter()
            .map(|c| current + *c)
            .filter(|c| grid.is_inside(c))
            .filter(|c| {
                let val = grid.get(c).unwrap();
                val - current_val == 1 && !visited.contains(c)
            })
            .collect();

        neighbours.iter().for_each(|c| {
            visited.insert(*c);
            q.push_back(*c);
        });
    }

    score
}

// DFS, sum all the 0s we get from a 9 (visiting some nodes multiple times, we don´t care)
fn rating(grid: &Grid<i8>, current: &Coordinate) -> u32 {
    let directions = [
        Coordinate::new(-1, 0),
        Coordinate::new(0, -1),
        Coordinate::new(0, 1),
        Coordinate::new(1, 0),
    ];
    let current_val = grid.get(current).unwrap();

    if *current_val == 0 {
        return 1;
    }

    let neighbours: Vec<Coordinate> = directions
        .iter()
        .map(|c| *current + *c)
        .filter(|c| grid.is_inside(c))
        .filter(|c| {
            let val = grid.get(c).unwrap();
            val - current_val == -1
        })
        .collect();

    neighbours.iter().map(|c| rating(grid, c)).sum()
}

fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::<i8>::new_numeric_chars(input);
    let start = get_starting_coordinates(&grid, 0);
    Some(start.iter().map(|c| score(&grid, c)).sum())
}

fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::<i8>::new_numeric_chars(input);
    let start = get_starting_coordinates(&grid, 9);
    Some(start.iter().map(|c| rating(&grid, c)).sum())
}

fn main() {
    let input = fs::read_to_string("inputs/10.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input: String = fs::read_to_string("examples/10.txt").unwrap();
        assert_eq!(Some(36), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input: String = fs::read_to_string("examples/10.txt").unwrap();
        assert_eq!(Some(81), part_two(&input));
    }
}
