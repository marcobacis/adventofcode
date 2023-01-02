use std::collections::{HashSet, VecDeque};

enum Dimension {
    X,
    Y,
    Z,
}
#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();

        for dim in [Dimension::X, Dimension::Y, Dimension::Z] {
            for offset in [-1, 1] {
                let mut coord = self.clone();
                match dim {
                    Dimension::X => coord.x += offset,
                    Dimension::Y => coord.y += offset,
                    Dimension::Z => coord.z += offset,
                }
                coords.push(coord);
            }
        }
        coords
    }

    fn is_inside(&self, bounds: (Coord, Coord)) -> bool {
        let (min, max) = bounds;
        self.x >= min.x
            && self.y >= min.y
            && self.z >= min.z
            && self.x <= max.x
            && self.y <= max.y
            && self.z <= max.z
    }
}

fn bounds(coords: &HashSet<Coord>) -> (Coord, Coord) {
    let min = coords
        .iter()
        .fold(Coord { x: 0, y: 0, z: 0 }, |acc, coord| Coord {
            x: acc.x.min(coord.x - 1),
            y: acc.y.min(coord.y - 1),
            z: acc.z.min(coord.z - 1),
        });
    let max = coords
        .iter()
        .fold(Coord { x: 0, y: 0, z: 0 }, |acc, coord| Coord {
            x: acc.x.max(coord.x + 1),
            y: acc.y.max(coord.y + 1),
            z: acc.z.max(coord.z + 1),
        });
    (min, max)
}

fn flood_fill(from: Coord, bounds: (Coord, Coord), cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let mut coords: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<Coord> = VecDeque::new();

    coords.insert(from);
    q.push_back(from);

    while let Some(coord) = q.pop_front() {
        let neighbours: Vec<Coord> = coord.neighbours();
        let filtered: Vec<Coord> = neighbours
            .iter()
            .filter(|n| !cubes.contains(n) && n.is_inside(bounds))
            .map(|n| *n)
            .collect();
        for neighbour in filtered {
            if coords.insert(neighbour) {
                q.push_back(neighbour);
            }
        }
    }

    coords
}

fn parse(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l.split(",").map(|v| v.parse().unwrap()).collect();
            Coord {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    // Cube surface area = 6 - #cubes with two coordinates equal and another with distance = 1
    let cubes = parse(input);
    let mut surface_area = 0;
    for cube in &cubes {
        surface_area += cube
            .neighbours()
            .iter()
            .filter(|n| !cubes.contains(n))
            .count();
    }

    Some(surface_area as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Use flood-fill (https://en.wikipedia.org/wiki/Flood_fill)
    // to get the "outside" cubes, then get only sides of the cubes which
    // are in contact with the outside
    // Special case: all the inputs are positive and don't contain (0,0,0), so we start from there

    let cubes = parse(input);
    let bounds = bounds(&cubes);
    let outside = flood_fill(Coord { x: 0, y: 0, z: 0 }, bounds, &cubes);

    let mut surface_area = 0;
    for cube in &cubes {
        surface_area += cube
            .neighbours()
            .iter()
            .filter(|n| outside.contains(n))
            .count();
    }

    Some(surface_area as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
