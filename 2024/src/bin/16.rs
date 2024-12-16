use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fs, u32};

use advent_of_code::{coordinate::{Coordinate, EAST, NORTH, SOUTH, WEST}, grid::Grid};

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

fn dijkstra(grid: &Grid<char>, start: Coordinate) -> (Option<u32>,HashMap<Coordinate, HashSet<Coordinate>>) {
    
    let mut costs = Grid::<u32>::initialize(grid.height, grid.width, u32::MAX);
    costs.set(start, 0);
    
    let mut heap = BinaryHeap::from([State {coordinate: start, cost: 0, direction: EAST}]);

    let mut prev : HashMap<Coordinate, HashSet<Coordinate>> = HashMap::new();

    while let Some(state) = heap.pop() {
        let value = grid.get(&state.coordinate).unwrap();

        if *value == END {
            return (Some(state.cost), prev);
        }

        if state.cost > *costs.get(&state.coordinate).unwrap() {
            continue;
        }


        let next : Vec<State> = vec![NORTH, EAST, WEST, SOUTH].iter()
            .filter(|d| grid.get(&(state.coordinate + **d)).is_some_and(|v| *v != WALL) )
            .filter(|d| **d != state.direction.opposite())
            .map(|d| State {coordinate: state.coordinate + *d, direction: *d, cost: state.cost + (if *d == state.direction { 1 } else { 1001 })})
            .collect();
            

        for next_state in next {
            prev.entry(next_state.coordinate).or_default().insert(state.coordinate);

            if next_state.cost < *costs.get(&next_state.coordinate).unwrap() {
                costs.set(next_state.coordinate, next_state.cost);
                heap.push(next_state);
            }
        }
    }
    (None, prev)
}


fn count_paths(prev: &HashMap<Coordinate, HashSet<Coordinate>> , start: Coordinate) -> HashSet<Coordinate> {
    let mut visited:  HashSet<Coordinate> = HashSet::new();
    let mut queue: VecDeque<Coordinate> = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        let next  = match prev.get(&current) {
            Some(c) => c,
            None => &HashSet::new(),
        };

        for c in next {
            if !visited.contains(&c) {
                queue.push_back(*c);
                visited.insert(*c);
            }
        }
    }

    visited
}

fn part_one(input: &str) -> Option<u32> {
    let grid : Grid<char> = Grid::new_chars(input);
    let start = grid.find_first(START).unwrap();

    let (result, _) = dijkstra(&grid, start);
    result
}

fn part_two(input: &str) -> Option<u32> {
    let mut grid : Grid<char> = Grid::new_chars(input);
    let start = grid.find_first(START).unwrap();
    let end = grid.find_first(END).unwrap();

    let (_, prev) = dijkstra(&grid, start);

    let visited = count_paths(&prev, end);

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
