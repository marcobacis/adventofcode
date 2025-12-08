use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Edge {
    a: usize,
    b: usize,
    distance: u64,
}

impl Coordinate {
    pub fn new(line: &str) -> Self {
        let fields: Vec<u64> = line.split(',').filter_map(|f| f.parse().ok()).collect();
        assert_eq!(3, fields.len());
        Coordinate {
            x: fields[0],
            y: fields[1],
            z: fields[2],
        }
    }

    pub fn distance(&self, other: &Coordinate) -> u64 {
        let dist_x = self.x.abs_diff(other.x).pow(2) as u64;
        let dist_y = self.y.abs_diff(other.y).pow(2) as u64;
        let dist_z = self.z.abs_diff(other.z).pow(2) as u64;

        dist_x + dist_y + dist_z
    }
}

impl Edge {
    pub fn new(coords: &Vec<Coordinate>, a: usize, b: usize) -> Self {
        Self {
            a,
            b,
            distance: coords[a].distance(&coords[b]),
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.distance < other.distance {
            std::cmp::Ordering::Greater
        } else if self.distance > other.distance {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

struct SetsContainer {
    sets: Vec<HashSet<usize>>,
}

impl SetsContainer {
    pub fn new() -> Self {
        Self {
            sets: vec![],
        }
    }

    pub fn insert(&mut self, edge: &Edge) {
        let contains_a = self.sets.iter().position(|c| c.contains(&edge.a));
        let contains_b = self.sets.iter().position(|c| c.contains(&edge.b));


        match (contains_a, contains_b) {
            (Some(a), Some(b)) if a == b => (), //nothing happens
            (Some(a), Some(b)) => {
                // union
                let b_component = self.sets[b].clone();
                for elem in b_component {
                    self.sets[a].insert(elem);
                }
                self.sets.remove(b);
            }, 
            (Some(a), None) => {self.sets[a].insert(edge.b);}, 
            (None, Some(b)) => {self.sets[b].insert(edge.a);}, // add a to b
            (None, None) => {self.sets.push(HashSet::from([edge.a,edge.b]));}, // cretae new component
        };
    }

    pub fn biggest_sizes(&self, k: usize) -> Vec<u64> {
        self.sets.iter().map(|s| s.len() as u64).sorted_unstable().rev().take(k).collect()
    }

    fn covers_everything(&self, num_items: usize) -> bool {
        self.sets.len() == 1 && self.sets[0].len() == num_items
    }
}

fn solve_part_one(input: &str, num_connections: usize) -> Option<u64> {
    // Parse coordinates
    let coords: Vec<Coordinate> = input.lines().map(Coordinate::new).collect();

    // Create edges
    let mut edges_heap = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            edges_heap.push(Edge::new(&coords, i, j));
        }
    }

    // Create components
    let mut components = SetsContainer::new();
    let mut count = 0;
    while count < num_connections && !edges_heap.is_empty() {
        let edge = edges_heap.pop().unwrap();
        components.insert(&edge);
        count += 1;
    }

    // Take 3 biggest components and multiply sizes
    Some(components.biggest_sizes(3).iter().product::<u64>() as u64)
}

fn part_one(input: &str) -> Option<u64> {
    solve_part_one(input, 1000)
}

fn part_two(input: &str) -> Option<u64> {
    // Parse coordinates
    let coords: Vec<Coordinate> = input.lines().map(Coordinate::new).collect();

    // Create edges
    let mut edges_heap = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            edges_heap.push(Edge::new(&coords, i, j));
        }
    }

    // Create components
    let mut components = SetsContainer::new();
    let mut edge = *edges_heap.peek().unwrap();
    while !components.covers_everything(coords.len()) && !edges_heap.is_empty() {
        edge = edges_heap.pop().unwrap();
        components.insert(&edge);
    }
    
    Some((coords[edge.a].x as u64) * (coords[edge.b].x as u64))
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
        assert_eq!(Some(40), solve_part_one(&input, 10));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/08.txt").unwrap();
        assert_eq!(Some(25272), part_two(&input));
    }
}
