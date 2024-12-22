use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
    fs,
};

use advent_of_code::{
    coordinate::{self, Coordinate},
    grid::Grid,
};

fn read_coordinates(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            let (xs, ys) = l.split_once(",").unwrap();
            let y = ys.parse().unwrap();
            let x = xs.parse().unwrap();
            Coordinate::new(y, x)
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    coordinate: Coordinate,
    cost: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Grid<bool>, start: Coordinate, end: Coordinate) -> Option<u32> {
    let mut costs = Grid::<u32>::initialize(grid.height, grid.width, u32::MAX);
    costs.set(start, 0);

    let mut heap = BinaryHeap::from([State {
        coordinate: start,
        cost: 0,
    }]);

    while let Some(state) = heap.pop() {
        let neighbours: Vec<State> = state
            .coordinate
            .around()
            .iter()
            .filter(|d| grid.get(&d).is_some_and(|v| *v))
            .map(|d| State {
                coordinate: *d,
                cost: state.cost + 1,
            })
            .collect();

        for next in neighbours {
            let min_cost = *costs.get(&next.coordinate).unwrap();

            if next.cost < min_cost {
                costs.set(next.coordinate, next.cost);
                heap.push(next);
            }
        }
    }

    let exit = *costs.get(&end).unwrap();
    if exit == u32::MAX {
        None
    } else {
        Some(exit)
    }
}


fn part_one(input: &str, height: usize, width: usize, steps: usize) -> Option<u32> {
    let coordinates = read_coordinates(input);
    let mut grid = Grid::initialize(height, width, true);
    for i in 0..steps {
        let coord = coordinates[i];
        grid.set(coord, false);
    }

    dijkstra(
        &grid,
        Coordinate::new(0, 0),
        Coordinate::new((height - 1) as i32, (width - 1) as i32),
    )
}

fn part_two(input: &str, height: usize, width: usize) -> Option<String> {
    let coordinates = read_coordinates(input);
    let mut grid = Grid::initialize(height, width, true);
    for i in 0..coordinates.len() {
        let coord = coordinates[i];
        grid.set(coord, false);

        if dijkstra(
            &grid,
            Coordinate::new(0, 0),
            Coordinate::new((height - 1) as i32, (width - 1) as i32),
        )
        .is_none()
        {
            return Some(format!("{},{}", coord.x, coord.y));
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("inputs/18.txt").unwrap();
    advent_of_code::solve(1, &input, |i| part_one(i, 71, 71, 1024));
    advent_of_code::solve(2, &input, |i| part_two(i, 71, 71));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/18.txt").unwrap();
        assert_eq!(Some(22), part_one(&input, 7, 7, 12));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/18.txt").unwrap();
        assert_eq!(Some("6,1".into()), part_two(&input, 7, 7));
    }
}
