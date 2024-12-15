use std::{
    collections::{HashMap, HashSet},
    fs,
};

use advent_of_code::{coordinate::Coordinate, grid::Grid};

fn load_inputs(input: &str) -> (Grid<char>, HashMap<char, Vec<Coordinate>>) {
    let grid = Grid::new_chars(input);

    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Coordinate::new(y as i32, x as i32);
            let value = grid.get(&coord).unwrap();
            if *value != '.' {
                if antennas.contains_key(value) {
                    let group = antennas.get_mut(value).unwrap();
                    group.push(coord);
                } else {
                    antennas.insert(*value, vec![coord]);
                }
            }
        }
    }

    (grid, antennas)
}

fn generate_antinodes(
    grid: &Grid<char>,
    antennas: &HashMap<char, Vec<Coordinate>>,
    resonance: bool,
) -> HashSet<Coordinate> {
    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    // Generate antinodes for each pair of antennas of same frequency
    for group in antennas.values() {
        for a in group.iter() {
            for b in group.iter() {
                if a != b {
                    let nodes = compute_antinodes(grid, *a, *b, resonance);
                    nodes.iter().for_each(|c| {
                        antinodes.insert(*c);
                    });
                }
            }
        }
    }

    // Keep only antinodes inside grid
    antinodes
        .iter()
        .filter_map(|c| if grid.is_inside(c) { Some(*c) } else { None })
        .collect()
}

fn compute_antinodes(
    grid: &Grid<char>,
    a: Coordinate,
    b: Coordinate,
    resonance: bool,
) -> Vec<Coordinate> {
    let diff = Coordinate {
        y: a.y - b.y,
        x: a.x - b.x,
    };

    if resonance {
        let mut nodes: Vec<Coordinate> = vec![];

        // Outer from a
        let mut n = a;
        while grid.is_inside(&n) {
            nodes.push(n);
            n = n + diff;
        }

        // Outer from b
        n = b;
        while grid.is_inside(&n) {
            nodes.push(n);
            n = n - diff;
        }

        nodes
    } else {
        vec![a + diff, b - diff]
    }
}

fn part_one(input: &str) -> Option<u32> {
    let (grid, antennas) = load_inputs(input);
    let antinodes = generate_antinodes(&grid, &antennas, false);
    Some(antinodes.len() as u32)
}

fn part_two(input: &str) -> Option<u32> {
    let (grid, antennas) = load_inputs(input);
    let antinodes = generate_antinodes(&grid, &antennas, true);
    Some(antinodes.len() as u32)
}

fn main() {
    let input = fs::read_to_string("inputs/08.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/08.txt").unwrap();
        assert_eq!(Some(14), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/08.txt").unwrap();
        assert_eq!(Some(34), part_two(&input));
    }
}
