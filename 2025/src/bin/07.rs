use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use advent_of_code::{coordinate::Coordinate, grid::Grid};

fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_chars(input);

    let mut count = 0;
    let mut front = vec![false; grid.width];

    front[grid.find_first('S').unwrap().x as usize] = true;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if front[x] {
                let value = *grid
                    .get(&Coordinate {
                        y: y as i32,
                        x: x as i32,
                    })
                    .unwrap();
                if value == '^' {
                    count += 1;
                    if x > 0 {
                        front[x - 1] = true;
                    }
                    if x < grid.width - 1 {
                        front[x + 1] = true;
                    }
                    front[x] = false;
                }
            }
        }
    }

    Some(count)
}


fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new_chars(input);

    let mut front = vec![0; grid.width];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let value = *grid
                .get(&Coordinate {
                    y: y as i32,
                    x: x as i32,
                })
                .unwrap();
            if value == 'S' {
                front[x] = 1;
            }
            if value == '^' {
                if x > 0 {
                    front[x - 1] += front[x];
                }
                if x < grid.width - 1 {
                    front[x + 1] += front[x];
                }
                front[x] = 0;
            }
        }
    }

    Some(front.iter().sum())
}

fn main() {
    let input = fs::read_to_string("inputs/07.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(Some(21), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(Some(40), part_two(&input));
    }
}
