pub fn part_one(input: &str) -> Option<u32> {
    let grid = read_grid(input);

    let mut num_visible: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_visible(&grid, y, x) {
                num_visible += 1;
            }
        }
    }

    Some(num_visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = read_grid(input);

    let mut max_score: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let tree_score = score(&grid, y, x);
            if tree_score > max_score {
                max_score = tree_score;
            }
        }
    }

    Some(max_score)
}

fn read_grid(input: &str) -> Vec<Vec<u32>> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| !c.is_ascii_whitespace())
                .map(|c| String::from(c).parse().unwrap())
                .collect()
        })
        .collect();

    grid
}

fn is_visible(grid: &Vec<Vec<u32>>, y: usize, x: usize) -> bool {
    let tree_height = grid[y][x];
    let height = grid.len();
    let width = grid[y].len();

    // West-East direction
    let mut visible_w_e: bool = true;
    for i in 0..x {
        if grid[y][i] >= tree_height {
            visible_w_e = false;
            break;
        }
    }

    // East-West direction
    let mut visible_e_w: bool = true;
    for i in (x + 1)..width {
        if grid[y][i] >= tree_height {
            visible_e_w = false;
            break;
        }
    }

    // North-South direction
    let mut visible_n_s: bool = true;
    for i in 0..y {
        if grid[i][x] >= tree_height {
            visible_n_s = false;
            break;
        }
    }

    // South-North direction
    let mut visible_s_n: bool = true;
    for i in (y + 1)..height {
        if grid[i][x] >= tree_height {
            visible_s_n = false;
            break;
        }
    }

    visible_e_w || visible_w_e || visible_n_s || visible_s_n
}

fn score(grid: &Vec<Vec<u32>>, y: usize, x: usize) -> u32 {
    let tree_height = grid[y][x];
    let height = grid.len();
    let width = grid[y].len();

    // West-East direction
    let mut score_w_e: u32 = 0;
    for i in (x + 1)..width {
        score_w_e += 1;
        if grid[y][i] >= tree_height {
            break;
        }
    }

    // East-West direction
    let mut score_e_w: u32 = 0;
    for i in (0..x).rev() {
        score_e_w += 1;
        if grid[y][i] >= tree_height {
            break;
        }
    }

    // North-South direction
    let mut score_n_s: u32 = 0;
    for i in (y + 1)..height {
        score_n_s += 1;
        if grid[i][x] >= tree_height {
            break;
        }
    }

    // South-North direction
    let mut score_s_n: u32 = 0;
    for i in (0..y).rev() {
        score_s_n += 1;
        if grid[i][x] >= tree_height {
            break;
        }
    }
    score_e_w * score_w_e * score_n_s * score_s_n
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
