use num::abs;

fn solve(input: &str, expansion: usize) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    // Read galaxies
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '#' {
                galaxies.push((y, x));
            }
        }
    }

    // Get empty space
    let empty_rows: Vec<usize> = (0..height)
        .filter(|y| (0..width).all(|x| grid[*y][x] == '.'))
        .collect();

    let empty_cols: Vec<usize> = (0..width)
        .filter(|x| (0..height).all(|y| grid[y][*x] == '.'))
        .collect();

    // Expand galaxies position
    let new_galaxies: Vec<(usize, usize)> = galaxies
        .iter()
        .map(|(y, x)| {
            (
                y + empty_rows.iter().filter(|row| *row < y).count() * (expansion - 1),
                x + empty_cols.iter().filter(|col| *col < x).count() * (expansion - 1),
            )
        })
        .collect();

    // Sum their manhattan distance
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            if i != j {
                let distance = abs(new_galaxies[i].0 as isize - new_galaxies[j].0 as isize)
                    + abs(new_galaxies[i].1 as isize - new_galaxies[j].1 as isize);

                sum += distance;
            }
        }
    }

    Some(sum as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 1000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(374));
    }

    #[test]
    fn test_part_two_10() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(solve(&input, 10), Some(1030));
    }

    #[test]
    fn test_part_two_100() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(solve(&input, 100), Some(8410));
    }
}
