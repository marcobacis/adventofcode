use std::{collections::{HashMap, HashSet}, fs, sync::PoisonError};

#[derive(Clone)]
struct Grid {
    grid: Vec<char>,
    height: usize,
    width: usize,
}


impl Grid {
    pub fn new(input: &str) -> Self {
        let lines : Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().unwrap().len();

        Grid {
            height,
            width,
            grid: input.chars().filter(|c| !c.is_whitespace()).collect()
        }
    }

    fn get(&self, coord: &Coordinate) -> Option<char> {
        if !self.is_inside(coord) {
            return None;
        }
        Some(self.grid[coord.y as usize * self.width + coord.x as usize])
    }

    fn is_inside(&self, coordinate: &Coordinate) -> bool {
        coordinate.y >= 0 && coordinate.y < self.height as i32 && coordinate.x >= 0 && coordinate.x < self.width as i32
    }

    pub fn is_obstacle(&self, pos: &Coordinate) -> bool {
        match self.get(&pos) {
            Some(c) => c == '#',
            None => false,
        }
    }
    
    fn find_first(&self, arg: char) -> Option<Coordinate> {
        for y in 0..self.height {
            for x in 0..self.width {
                let coordinate =Coordinate{y: y as i32,x: x as i32};
                if self.get(&coordinate).unwrap() == arg {
                    return Some(coordinate)
                }
            }
        }
        None
    }
    
    fn set(&mut self, coordinate: Coordinate, c: char) {
        if !self.is_inside(&coordinate) {
            return;
        }

        self.grid[coordinate.y as usize * self.width + coordinate.x as usize] = c;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinate {
    y: i32,
    x: i32,
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

    pub fn step(&mut self, grid: &Grid) {
        let next = self.peek_next();
        if grid.is_obstacle(&next) {
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

fn detect_loop(initial: Coordinate, obstacle: Coordinate, grid: &Grid) -> bool {
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
    let grid = Grid::new(input);
    let initial = grid.find_first('^').unwrap();
    Some(guard_steps(initial, &grid).len() as u32)
}

fn guard_steps(initial: Coordinate, grid: &Grid) -> HashSet<Coordinate> {
    let mut guard = Guard {
        position: initial,
        direction: Direction::North,
    };

    let mut positions : HashSet<Coordinate> = HashSet::new();

    while grid.is_inside(&guard.position) {
        positions.insert(guard.position);
        guard.step(&grid);
    }

    positions
}

fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);

    let initial = grid.find_first('^').unwrap();

    let positions = guard_steps(initial, &grid);

    Some(positions.iter().filter(|pos| detect_loop(initial, **pos, &grid)).count() as u32)
}


fn main() {
    let input = fs::read_to_string("inputs/06.txt").unwrap();

    println!("Solutions 🎄");
    let result_part_one = part_one(&input);
    let result_part_two = part_two(&input);

    if let Some(res) = result_part_one {
        println!("Part 1: {}", res);
    }
    if let Some(res) = result_part_two {
        println!("Part 2: {}", res);
    }
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