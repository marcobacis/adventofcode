use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let (heights, _, s, e) = parse_input(input);
    let distances = dijkstra(&heights, s, |c, n| n <= c + 1);
    Some(distances[e.0][e.1] as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (heights, size, _, e) = parse_input(input);
    let (width, height) = size;

    let distances = dijkstra(&heights, e, |c, n| c == 0 || n >= c - 1);

    // Get min distance from 'a' point to 'E'
    let mut dist = usize::MAX;
    for y in 0..height {
        for x in 0..width {
            if heights[y][x] == 0 && distances[y][x] < dist {
                dist = distances[y][x];
            }
        }
    }

    Some(dist as u32)
}

fn dijkstra(
    heights: &Vec<Vec<usize>>,
    start: (usize, usize),
    f: impl Fn(usize, usize) -> bool, // Filter for neighbours validity
) -> Vec<Vec<usize>> {
    let height = heights.len();
    let width = heights[0].len();

    // Initialize unvisited set
    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            unvisited.insert((y, x));
        }
    }

    // Initialize distances grid
    let mut distances = vec![vec![usize::MAX as usize; width]; height];
    distances[start.0][start.1] = 0;

    // Main loop
    let mut current = start;
    while !unvisited.is_empty() {
        let (y, x) = current;

        // Get unvisited and valid neighbours
        let neighbours: Vec<(usize, usize)> = [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .iter()
            .map(|(my, mx)| ((y as i32 + *my), (x as i32 + *mx)))
            .filter(|(my, mx)| *my >= 0 && *my < height as i32 && *mx >= 0 && *mx < width as i32)
            .map(|(my, mx)| (my as usize, mx as usize))
            .filter(|(my, mx)| unvisited.contains(&(*my, *mx)))
            .filter(|(my, mx)| f(heights[y][x], heights[*my][*mx]))
            .collect();

        // Update distances
        if distances[y][x] != usize::MAX {
            for (ny, nx) in neighbours {
                distances[ny][nx] = distances[ny][nx].min(distances[y][x] + 1);
            }
        }

        // Remove current
        unvisited.remove(&current);
        if unvisited.is_empty() {
            break;
        }

        // Set next point to check
        current = *(unvisited
            .iter()
            .min_by_key(|(ky, kx)| distances[*ky][*kx])
            .unwrap());
    }

    distances
}

fn parse_input(
    input: &str,
) -> (
    Vec<Vec<usize>>, // heights
    (usize, usize),  // size
    (usize, usize),  // 'S' coordinate
    (usize, usize),  // 'E' coordinate
) {
    // Read input file, map to heights from a to z
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let heights: Vec<Vec<usize>> = input
        .iter()
        .map(|row| {
            row.iter()
                .map({
                    |col| match col {
                        'S' => 0,
                        'E' => 'z' as usize - 'a' as usize,
                        c => *c as usize - 'a' as usize,
                    }
                })
                .collect()
        })
        .collect();
    let width = input[0].len();
    let height = input.len();

    let mut s_coord = (0, 0);
    let mut e_coord = (0, 0);

    for y in 0..height {
        for x in 0..width {
            match input[y][x] {
                'S' => s_coord = (y, x),
                'E' => e_coord = (y, x),
                _ => (),
            };
        }
    }
    (heights, (width, height), s_coord, e_coord)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
