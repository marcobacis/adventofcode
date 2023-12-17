use std::collections::{HashMap, HashSet};

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut new_grid = Grid {
            grid: input.lines().map(|l| l.chars().collect()).collect(),
        };

        new_grid
    }

    fn starting_pos(&self) -> (usize, usize) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.grid[y][x] == 'S' {
                    return (y, x);
                }
            }
        }
        (0, 0)
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (y, x): (i32, i32) = (pos.0 as i32, pos.1 as i32);

        let directions = match self.grid[y as usize][x as usize] {
            'S' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            '|' => vec![(-1, 0), (1, 0)],
            '-' => vec![(0, -1), (0, 1)],
            'L' => vec![(-1, 0), (0, 1)],
            'J' => vec![(-1, 0), (0, -1)],
            '7' => vec![(1, 0), (0, -1)],
            'F' => vec![(1, 0), (0, 1)],
            _ => vec![],
        };

        directions
            .iter()
            .filter(|d| self.is_connected((y, x), **d))
            .map(|(dy, dx)| ((y + dy) as usize, (x + dx) as usize))
            .collect()
    }

    fn is_connected(&self, pos: (i32, i32), direction: (i32, i32)) -> bool {
        let (y, x) = (pos.0 + direction.0, pos.1 + direction.1);

        if y < 0 || y >= self.grid.len() as i32 || x < 0 || x >= self.grid[0].len() as i32 {
            return false;
        }

        let val = self.grid[y as usize][x as usize];

        match direction {
            // Up
            (-1, 0) => val == '|' || val == '7' || val == 'F' || val == 'S',
            // Down
            (1, 0) => val == '|' || val == 'L' || val == 'J' || val == 'S',
            // Left
            (0, -1) => val == '-' || val == 'L' || val == 'F' || val == 'S',
            // Right
            (0, 1) => val == '-' || val == '7' || val == 'J' || val == 'S',
            _ => false,
        }
    }

    pub fn get_loop_nodes(&self) -> Vec<(usize, usize)> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut loop_nodes = vec![];

        let mut current = self.starting_pos();
        loop {
            visited.insert(current);
            loop_nodes.push(current);

            let n = self.neighbors(current);
            let n = n.iter().filter(|n| !visited.contains(n)).next();

            match n {
                None => return loop_nodes,
                Some(neighbor) => {
                    current = *neighbor;
                }
            }
        }
    }

    pub fn get_loop_length(&self) -> usize {
        self.get_loop_nodes().len()
    }

    fn is_enclosed(&self, coord: (usize, usize), loop_nodes: &Vec<(usize, usize)>) -> bool {
        if loop_nodes.contains(&coord) {
            return false;
        }

        // Check if the loop goes around the point
        // Use a window to get pair of coordinates.
        // Whenever we pass over the point from the left we flip a flag
        let mut loop_nodes = loop_nodes.clone();
        loop_nodes.push(loop_nodes[0]);

        let (y, x) = coord;
        let mut enclosed: bool = false;
        loop_nodes.windows(2).for_each(|pair| {
            let (y1, _x1) = pair[0];
            let (y2, x2) = pair[1];
            if (y2 > y) != (y1 > y) && x < x2 {
                enclosed = !enclosed;
            }
        });
        enclosed
    }

    pub fn get_enclosed_points(&self) -> Vec<(usize, usize)> {
        let loop_nodes = self.get_loop_nodes();

        let mut enclosed = vec![];
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.is_enclosed((y, x), &loop_nodes) {
                    enclosed.push((y, x));
                }
            }
        }
        enclosed
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let length = grid.get_loop_length() as u32;

    Some((length + 1) / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let enclosed = grid.get_enclosed_points();
    Some(enclosed.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let input = "-L|F7\n\
                    7S-7|\n\
                    L|7||\n\
                    -L-J|\n\
                    L|-JF";
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_one_complex() {
        let input = "7-F7-\n\
                    .FJ|7\n\
                    SJLL7\n\
                    |F--J\n\
                    LJ.LJ";
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two_simple() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_part_two_complex() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part_two(&input), Some(8));
    }
}
