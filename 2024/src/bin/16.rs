use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs, u32,
};

use advent_of_code::{
    coordinate::{Coordinate, EAST, NORTH, SOUTH, WEST},
    grid::Grid,
};
use itertools::Itertools;

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    coordinate: Coordinate,
    direction: Coordinate,
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

fn dijkstra(
    grid: &Grid<char>,
    start: Coordinate,
) -> (
    Option<u32>,
    HashMap<(Coordinate, Coordinate), HashSet<(Coordinate, Coordinate)>>,
) {
    let mut costs = Grid::<u32>::initialize(grid.height, grid.width, u32::MAX);
    costs.set(start, 0);

    let mut heap = BinaryHeap::from([State {
        coordinate: start,
        cost: 0,
        direction: EAST,
    }]);

    let mut prev: HashMap<(Coordinate, Coordinate), HashSet<(Coordinate, Coordinate)>> =
        HashMap::new();

    while let Some(state) = heap.pop() {
        let value = grid.get(&state.coordinate).unwrap();

        if *value == END {
            return (Some(state.cost), prev);
        }

        if state.cost > *costs.get(&state.coordinate).unwrap() {
            continue;
        }

        let next: Vec<State> = [NORTH, EAST, WEST, SOUTH]
            .iter()
            .filter(|d| {
                grid.get(&(state.coordinate + **d))
                    .is_some_and(|v| *v != WALL)
            })
            .filter(|d| **d != state.direction.opposite())
            .map(|d| State {
                coordinate: state.coordinate + *d,
                direction: *d,
                cost: state.cost + (if *d == state.direction { 1 } else { 1001 }),
            })
            .collect();

        for next_state in next {
            let min_cost = *costs.get(&next_state.coordinate).unwrap();

            if next_state.cost < min_cost {
                costs.set(next_state.coordinate, next_state.cost);
                heap.push(next_state);

                prev.insert(
                    (next_state.coordinate, next_state.direction),
                    HashSet::from([(state.coordinate, next_state.direction)]),
                );
            } else if next_state.cost == min_cost {
                prev.entry((next_state.coordinate, state.direction))
                    .or_default()
                    .insert((state.coordinate, state.direction));
            }
        }
    }
    (None, prev)
}

fn count_paths(
    prev: &mut HashMap<(Coordinate, Coordinate), HashSet<(Coordinate, Coordinate)>>,
    start: Coordinate,
) -> HashSet<Coordinate> {
    let mut visited: HashSet<_> = HashSet::new();
    let mut queue: VecDeque<_> = VecDeque::new();

    let mut end = prev.entry((start, NORTH)).or_default().clone();
    end.extend(prev.entry((start, EAST)).or_default().iter().cloned());
    end.extend(prev.entry((start, WEST)).or_default().iter().cloned());
    end.extend(prev.entry((start, SOUTH)).or_default().iter().cloned());

    for s in end.iter() {
        queue.push_back(s);
    }

    while let Some(current) = queue.pop_front() {
        if let Some(next) = prev.get(current) {
            for c in next.iter() {
                if !visited.contains(&c.0) {
                    queue.push_back(c);
                    visited.insert(c.0);
                }
            }
        }
    }

    visited
}

fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = Grid::new_chars(input);
    let start = grid.find_first(START).unwrap();

    let (result, _) = dijkstra(&grid, start);
    result
}

fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid<char> = Grid::new_chars(input);
    let start = grid.find_first(START).unwrap();
    let end = grid.find_first(END).unwrap();

    let (_, mut prev) = dijkstra(&grid, start);

    let visited = count_paths(&mut prev, end);

    for c in visited.iter() {
        let v = *grid.get(c).unwrap();

        if v != START && v != END {
            grid.set(*c, 'O');
        }
    }

    // grid.print("");

    Some(visited.len() as u32)
}

fn main() {
    let input = fs::read_to_string("inputs/16.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test_1() {
        let input = fs::read_to_string("examples/16_1.txt").unwrap();
        assert_eq!(Some(7036), part_one(&input));
    }

    #[test]
    fn part_one_test_2() {
        let input = fs::read_to_string("examples/16_2.txt").unwrap();
        assert_eq!(Some(11048), part_one(&input));
    }

    #[test]
    fn part_two_test_1() {
        let input = fs::read_to_string("examples/16_1.txt").unwrap();
        assert_eq!(Some(45), part_two(&input));
    }

    #[test]
    fn part_two_test_2() {
        let input = fs::read_to_string("examples/16_2.txt").unwrap();
        assert_eq!(Some(64), part_two(&input));
    }
}
