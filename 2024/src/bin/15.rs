use std::{fs, thread::sleep, time::Duration};

use advent_of_code::{
    coordinate::{Coordinate, EAST, NORTH, SOUTH, WEST},
    grid::Grid,
};
use colored::Colorize;
use itertools::Itertools;

const EMPTY: char = '.';
const BOX: char = 'O';
const OBSTACLE: char = '#';
const ROBOT: char = '@';

fn read_input(input: &str) -> (Grid<char>, Vec<Coordinate>, Coordinate) {
    let lines: Vec<&str> = input.lines().collect();

    let grid_input = lines
        .iter()
        .take_while(|&&line| {
            line.chars()
                .all(|c| c == '#' || c == '.' || c == 'O' || c == '@')
        })
        .copied()
        .join("\n");

    let movements = lines
        .iter()
        .skip_while(|&&line| {
            line.len() == 0
                || line
                    .chars()
                    .all(|c| c == '#' || c == '.' || c == 'O' || c == '@')
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

fn move_robot(grid: &mut Grid<char>, robot: Coordinate, direction: Coordinate) -> Coordinate {
    let robot_dest = robot + direction;
    if !grid.is_inside(&robot_dest) {
        return robot;
    }

    if *grid.get(&robot_dest).unwrap() == OBSTACLE {
        return robot;
    }

    if *grid.get(&robot_dest).unwrap() == BOX {
        // Check if it can move
        let mut box_pos = robot_dest;
        while *grid.get(&box_pos).unwrap() != OBSTACLE && *grid.get(&box_pos).unwrap() != EMPTY {
            box_pos = box_pos + direction;
        }

        // Cannot move boxes
        if *grid.get(&box_pos).unwrap() == OBSTACLE {
            return robot;
        }

        // Move boxes
        let mut box_pos = robot_dest;
        grid.set(box_pos, EMPTY);
        box_pos = box_pos + direction;
        while *grid.get(&box_pos).unwrap() != OBSTACLE && *grid.get(&box_pos).unwrap() != EMPTY {
            grid.set(box_pos, BOX);
            box_pos = box_pos + direction;
        }
        grid.set(box_pos, BOX);
    }

    grid.set(robot, EMPTY);
    grid.set(robot_dest, ROBOT);

    robot_dest
}

fn gps_coordinates_sum(grid: &Grid<char>) -> u32 {
    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Coordinate::new(y as i32, x as i32);
            if *grid.get(&coord).unwrap() == BOX {
                sum += coord.y * 100 + coord.x;
            }
        }
    }

    sum as u32
}

fn part_one(input: &str) -> Option<u32> {
    let (mut grid, movements, mut robot) = read_input(input);

    for movement in movements {
        robot = move_robot(&mut grid, robot, movement);
    }

    Some(gps_coordinates_sum(&grid))
}
fn print(grid: &Grid<char>) {
    print!("\x1b[2J\x1b[H");

    for y in 0..grid.height {
        for x in 0..grid.height {
            let c = *grid.get(&Coordinate::new(y as i32, x as i32)).unwrap();

            let c = match c {
                ROBOT =>c.to_string().red(),
                BOX => c.to_string().blue(),
                OBSTACLE => c.to_string().yellow(),
                c => c.to_string().white(),
            };
            print!("{}", c);
        }
        println!();
    }

    println!();
}

fn part_one_pretty(input: &str) -> Option<u32> {
    let (mut grid, movements, mut robot) = read_input(input);

    for movement in movements {
        print(&grid);
        robot = move_robot(&mut grid, robot, movement);
        sleep(Duration::from_millis(20));
    }
    print(&grid);

    Some(gps_coordinates_sum(&grid))
}

fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = fs::read_to_string("inputs/15.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test_1() {
        let input = fs::read_to_string("examples/15_1.txt").unwrap();
        assert_eq!(Some(2028), part_one(&input));
    }

    #[test]
    fn part_one_test_2() {
        let input = fs::read_to_string("examples/15_2.txt").unwrap();
        assert_eq!(Some(10092), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/15.txt").unwrap();
        assert_eq!(None, part_two(&input));
    }
}
