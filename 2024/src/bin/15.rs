use std::{fs, thread::sleep, time::Duration};

use advent_of_code::{
    coordinate::{Coordinate, EAST, NORTH, SOUTH, WEST},
    grid::Grid,
};
use colored::Colorize;
use itertools::Itertools;

const EMPTY: char = '.';
const SMALL_BOX: char = 'O';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
const OBSTACLE: char = '#';
const ROBOT: char = '@';

fn is_valid(c: char) -> bool {
    c == OBSTACLE || c == EMPTY || c == SMALL_BOX || c == BOX_LEFT || c == BOX_RIGHT || c == ROBOT
}

fn read_input(input: &str) -> (Grid<char>, Vec<Coordinate>, Coordinate) {
    let lines: Vec<&str> = input.lines().collect();

    let grid_input = lines
        .iter()
        .take_while(|&&line| {
            line.chars().all(is_valid)
        })
        .copied()
        .join("\n");

    let movements = lines
        .iter()
        .skip_while(|&&line| {
            line.len() == 0
                || line
                    .chars()
                    .all(is_valid)
        })
        .join("");

    let grid = Grid::<char>::new_chars(&grid_input);

    let movements = movements
        .chars()
        .map(|c| match c {
            '^' => NORTH,
            'v' => SOUTH,
            '<' => WEST,
            '>' => EAST,
            _ => Coordinate::new(0, 0),
        })
        .collect();

    let robot = grid.find_first(ROBOT).unwrap();

    (grid, movements, robot)
}

fn is_vertical(direction: &Coordinate) -> bool {
    *direction == NORTH || *direction == SOUTH
}

fn can_move(grid: &Grid<char>, position: Coordinate, direction: Coordinate) -> bool {
    let dest = position + direction;
    let value = *grid.get(&dest).unwrap();
    match value {
        SMALL_BOX => can_move(grid, dest, direction),
        OBSTACLE => false,
        BOX_LEFT => can_move(grid, dest, direction) && (!is_vertical(&direction) || can_move(grid, dest + EAST, direction)),
        BOX_RIGHT => can_move(grid, dest, direction) && (!is_vertical(&direction) || can_move(grid, dest + WEST, direction)),
        _ => true,
    }
}

fn do_move(grid: &mut Grid<char>, position: Coordinate, direction: Coordinate) -> Coordinate {
    let start_value = *grid.get(&position).unwrap();

    let dest = position + direction;
    let end_value = *grid.get(&dest).unwrap();
    match end_value {
        SMALL_BOX => {
            do_move(grid, dest, direction);
            grid.set(dest, start_value);
        },
        BOX_RIGHT => {
            if is_vertical(&direction) {
                do_move(grid, dest + WEST, direction);
                grid.set(dest + WEST, EMPTY);
            }
            do_move(grid, dest, direction);
            grid.set(dest, start_value);
        },
        BOX_LEFT => {
            if is_vertical(&direction) {
                do_move(grid, dest + EAST, direction);
                grid.set(dest + EAST, EMPTY);
            }
            do_move(grid, dest, direction);
            grid.set(dest, start_value);
        },
        OBSTACLE => {
            return position;
        },
        _ => {
            grid.set(dest, start_value);
        },
    }
    return dest;
}

fn step(grid: &mut Grid<char>, robot: Coordinate, direction: Coordinate) -> Coordinate {
    if can_move(grid, robot, direction) {
        let robot_dest = do_move(grid, robot, direction);
        grid.set(robot, EMPTY);
        robot_dest
    } else {
        robot
    }
}

fn gps_coordinates_sum(grid: &Grid<char>) -> u32 {
    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Coordinate::new(y as i32, x as i32);
            let value = *grid.get(&coord).unwrap();
            if value == SMALL_BOX || value == BOX_LEFT {
                sum += coord.y * 100 + coord.x;
            }
        }
    }

    sum as u32
}

fn solve(input: &str) -> Option<u32> {
    let (mut grid, movements, mut robot) = read_input(input);
    for movement in movements {
        robot = step(&mut grid, robot, movement);
    }
    Some(gps_coordinates_sum(&grid))
}

fn print_color(grid: &Grid<char>) {
    print!("\x1b[2J\x1b[H");

    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = *grid.get(&Coordinate::new(y as i32, x as i32)).unwrap();

            let c = match c {
                ROBOT =>c.to_string().red(),
                SMALL_BOX | BOX_LEFT | BOX_RIGHT => c.to_string().green(),
                OBSTACLE => c.to_string().yellow(),
                c => c.to_string().white(),
            };
            print!("{}", c);
        }
        println!();
    }

    println!();
}

fn solve_pretty(input: &str) -> Option<u32> {
    let (mut grid, movements, mut robot) = read_input(input);

    for movement in movements {
        print_color(&grid);
        robot = step(&mut grid, robot, movement);
        sleep(Duration::from_millis(20));
    }
    print_color(&grid);

    Some(gps_coordinates_sum(&grid))
}

fn main() {
    let input = fs::read_to_string("inputs/15_1.txt").unwrap();
    advent_of_code::solve(1, &input, solve);

    let input = fs::read_to_string("inputs/15_2.txt").unwrap();
    advent_of_code::solve(2, &input, solve);

    // let input = fs::read_to_string("inputs/15_2.txt").unwrap();
    // solve_pretty(&input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test_1() {
        let input = fs::read_to_string("examples/15_1.txt").unwrap();
        assert_eq!(Some(2028), solve(&input));
    }

    #[test]
    fn part_one_test_2() {
        let input = fs::read_to_string("examples/15_2.txt").unwrap();
        assert_eq!(Some(10092), solve(&input));
    }

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("inputs/15.txt").unwrap();
        assert_eq!(Some(1457740), solve(&input));
    }

    #[test]
    fn part_two_test_2() {
        let input = fs::read_to_string("examples/15_4.txt").unwrap();
        assert_eq!(Some(9021), solve(&input));
    }
}
