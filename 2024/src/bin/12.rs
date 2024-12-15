use std::fs;

use advent_of_code::grid::Grid;

fn part_one(input: &str) -> Option<u32> {

    let grid = Grid::new(input);


    // TODO Find areas
    // TODO For each area, find perimeters
    // TODO  Sum of area * perimeter of each area

    None
}

fn part_two(input: &str) -> Option<u32> {
    None
}


fn main() {
    let input = fs::read_to_string("inputs/12.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/12.txt").unwrap();
        assert_eq!(Some(1930), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/12.txt").unwrap();
        assert_eq!(None, part_two(&input));
    }

}