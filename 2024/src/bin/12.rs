use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use advent_of_code::{
    coordinate::{Coordinate, EAST, NORTH, SOUTH, WEST},
    grid::Grid,
};
use itertools::Itertools;

// BFS starting from a given coordinate
fn find_area(
    grid: &Grid<char>,
    start: Coordinate,
    to_visit: &mut HashSet<Coordinate>,
) -> Vec<Coordinate> {
    let mut queue: VecDeque<Coordinate> = VecDeque::new();

    let mut area: Vec<Coordinate> = Vec::new();

    let value = *grid.get(&start).unwrap();
    to_visit.remove(&start);
    queue.push_back(start);
    area.push(start);

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();
        coord
            .around()
            .iter()
            .filter(|c| grid.is_inside(c))
            .filter(|c| *grid.get(c).unwrap() == value)
            .for_each(|c| {
                if to_visit.contains(c) {
                    queue.push_back(*c);
                    to_visit.remove(c);
                    area.push(*c);
                }
            });
    }
    area
}

fn find_areas(grid: &Grid<char>) -> Vec<Vec<Coordinate>> {
    let mut areas: Vec<Vec<Coordinate>> = Vec::new();

    let mut to_visit = HashSet::<Coordinate>::with_capacity(grid.height * grid.width);
    for y in 0..grid.height {
        for x in 0..grid.width {
            to_visit.insert(Coordinate::new(y as i32, x as i32));
        }
    }

    // BFS for every possible area
    while !to_visit.is_empty() {
        let start = *to_visit.iter().take(1).collect::<Vec<&Coordinate>>()[0];
        let area = find_area(grid, start, &mut to_visit);
        areas.push(area);
    }

    areas
}

fn compute_area(area: &Vec<Coordinate>) -> u32 {
    area.len() as u32
}

fn compute_perimeter(area: &Vec<Coordinate>) -> u32 {
    let coords: HashSet<Coordinate> = HashSet::from_iter(area.iter().cloned());

    area.iter()
        .map(|c| c.around().iter().filter(|n| !coords.contains(n)).count() as u32)
        .sum()
}

fn compute_corners(area: &Vec<Coordinate>) -> u32 {
    let coords: HashSet<Coordinate> = HashSet::from_iter(area.iter().cloned());

    let corner_cases = [
        vec![NORTH, WEST, NORTH + WEST],
        vec![NORTH, EAST, NORTH + EAST],
        vec![SOUTH, WEST, SOUTH + WEST],
        vec![SOUTH, EAST, SOUTH + EAST],
    ];

    coords
        .iter()
        .map(|c| {
            // Check for inner/outer corner in each direction
            corner_cases
                .iter()
                .map(|directions| {
                    let (o1, o2, o3) = directions
                        .iter()
                        .map(|d| coords.contains(&(*c + *d)))
                        .collect_tuple()
                        .unwrap();

                    let outer = if (o1, o2) == (false, false) { 1 } else { 0 };
                    let inner = if (o1, o2, o3) == (true, true, false) {
                        1
                    } else {
                        0
                    };

                    outer + inner
                })
                .sum::<u32>()
        })
        .sum()
}

fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_chars(input);

    let areas = find_areas(&grid);

    Some(
        areas
            .iter()
            .map(|area| compute_area(area) * compute_perimeter(area))
            .sum(),
    )
}

fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new_chars(input);

    let areas = find_areas(&grid);

    Some(
        areas
            .iter()
            .map(|area| compute_area(area) * compute_corners(area))
            .sum(),
    )
}

fn main() {
    let input = fs::read_to_string("inputs/12.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![(0,0), (0,1), (1,0), (1,1)], 4)]
    #[case(vec![(0,0)], 4)]
    #[case(vec![(0,1),(0,2),(1,2),(2,0),(2,1),(2,2)], 8)]
    fn test_corners(#[case] area: Vec<(i32, i32)>, #[case] sides: u32) {
        let area = area.iter().map(|(y, x)| Coordinate::new(*y, *x)).collect();
        assert_eq!(sides, compute_corners(&area));
    }

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/12.txt").unwrap();
        assert_eq!(Some(1930), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/12.txt").unwrap();
        assert_eq!(Some(1206), part_two(&input));
    }
}
