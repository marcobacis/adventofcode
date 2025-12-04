use std::fs;

use advent_of_code::{coordinate::Coordinate, grid::Grid};

fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new_chars(input);
    Some(accessible(&grid).iter().count() as u32)
}

fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::new_chars(input);

    let mut count = 0;
    let mut coords = accessible(&grid);
    while coords.iter().count() > 0 {
        count += coords.iter().count();
        coords.iter().for_each(|c| grid.set(c, '.'));
        coords = accessible(&grid);
    }

    Some(count as u32)
}

fn accessible(grid: &Grid<char>) -> Vec<Coordinate> {
    let mut coordinates = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Coordinate::new(y as i32, x as i32);
            if grid.get(&coord) != Some(&'@') {
                continue;
            }

            let rolls_around = coord
                .around_all()
                .iter()
                .filter(|c| grid.get(c) == Some(&'@'))
                .count();

            if rolls_around < 4 {
                coordinates.push(coord);
            }
        }
    }
    coordinates
}

fn main() {
    let input = fs::read_to_string("inputs/04.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(Some(13), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(Some(43), part_two(&input));
    }
}
