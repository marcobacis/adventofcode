use std::{collections::HashSet, fs};
use advent_of_code::{coordinate::Coordinate, grid::Grid};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

pub fn is_obstacle(grid: &Grid<char>, pos: &Coordinate) -> bool {
    match grid.get(pos) {
        Some(c) => *c == '#',
        None => false,
    }
}

struct Guard {
    position: Coordinate,
    direction: Direction,
}

impl Guard {
    pub fn peek_next(&self) -> Coordinate {
        match self.direction {
            Direction::North => Coordinate {y : self.position.y - 1, x: self.position.x},
            Direction::East => Coordinate {y : self.position.y, x: self.position.x + 1},
            Direction::South => Coordinate {y : self.position.y + 1, x:self.position.x},
            Direction::West => Coordinate {y : self.position.y, x: self.position.x - 1},
        }
    }

    pub fn step(&mut self, grid: &Grid<char>) {
        let next = self.peek_next();
        if is_obstacle(grid,&next) {
            self.turn();
        } else {
            self.advance();
        }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn advance(&mut self) {
        self.position = self.peek_next();
    }
}

fn detect_loop(initial: Coordinate, obstacle: Coordinate, grid: &Grid<char>) -> bool {
    let mut grid = (*grid).clone();
    grid.set(obstacle, '#');

    let mut positions : HashSet<(Coordinate, Direction)> = HashSet::new();

    let mut guard = Guard {
        position: initial,
        direction: Direction::North,
    };

    guard.step(&grid);

    while grid.is_inside(&guard.position) && !positions.contains(&(guard.position, guard.direction)) {
        positions.insert((guard.position, guard.direction));
        guard.step(&grid);
    }

    grid.is_inside(&guard.position)
}



fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::<char>::new_chars(input);
    let initial = grid.find_first('^').unwrap();
    Some(guard_steps(initial, &grid).len() as u32)
}

fn guard_steps(initial: Coordinate, grid: &Grid<char>) -> HashSet<Coordinate> {
    let mut guard = Guard {
        position: initial,
        direction: Direction::North,
    };

    let mut positions : HashSet<Coordinate> = HashSet::new();

    while grid.is_inside(&guard.position) {
        positions.insert(guard.position);
        guard.step(grid);
    }

    positions
}

fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new_chars(input);

    let initial = grid.find_first('^').unwrap();

    let positions = guard_steps(initial, &grid);

    Some(positions.iter().filter(|pos| detect_loop(initial, **pos, &grid)).count() as u32)
}


fn main() {
    let input = fs::read_to_string("inputs/06.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(Some(41), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(Some(6), part_two(&input));
    }

}