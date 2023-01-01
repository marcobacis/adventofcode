// Solution "inspired" (I studied and then implemented it again)
// from https://nickymeuleman.netlify.app/garden/aoc2022-day16
// and others similar
// First, it lists all distances between "flowing" valves (flow > 0)
// Then if searches for the maximum flow by simulating all possible paths (DFS)
// and pruning the partial paths with the same open valves and relieved flow
// at the same time.

use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow: u32,
    neighbours: HashSet<&'a str>,
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Path<'a> {
    opened: BTreeSet<&'a str>,
    elapsed: u32,
    relieved: u32,
    curr: &'a str,
}

fn load_valves(input: &str) -> HashMap<&str, Valve> {
    let mut valves: HashMap<&str, Valve> = HashMap::new();
    let lines = input.lines();
    for l in lines {
        let (valve, neighbours) = l.split_once(";").unwrap();
        let valve = valve.strip_prefix("Valve ").unwrap();
        let (valve, flow) = valve.split_once(" has flow rate=").unwrap();
        let flow = flow.parse().unwrap();
        // I changed the input files to match this prefix, for simplicity
        let neighbours = neighbours.strip_prefix(" tunnels lead to valves ").unwrap();
        let neighbours = neighbours.split_terminator(", ").collect();

        valves.insert(
            &valve,
            Valve {
                name: &valve,
                flow: flow,
                neighbours: neighbours,
            },
        );
    }

    valves
}

fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Node {
        cost: 0,
        curr: from,
    });

    visited.insert(from);

    while let Some(Node { cost, curr }) = queue.pop() {
        if curr == to {
            return cost;
        }

        for neighbour in map[curr].neighbours.iter() {
            if visited.insert(neighbour) {
                queue.push(Node {
                    cost: cost + 1,
                    curr: &neighbour,
                })
            }
        }
    }
    u32::MAX
}

// map distance from every valve (with flow > 0, or AA) to every other valve
fn distances<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            // AA <-> name1
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));
            // AA <-> name2
            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));

            // name1 <-> name2
            let dist = min_cost(name1, name2, map);
            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);

            acc
        })
}

fn compute_relieved(
    from_time: u32,
    to_time: u32,
    relieved: u32,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Valve>,
) -> u32 {
    let time = to_time - from_time;
    let flow: u32 = opened.iter().map(|&name| map[name].flow).sum();
    flow * time + relieved
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = load_valves(input);
    let distances = distances(&valves);

    // Exhaustive search of all possible paths (of flowing valves)
    // We don't care about the actual path, only the opened valves, the time and the total flow
    // To optimize, we don't follow possible duplicates (kept track using the Path set)
    let flowing_valves: HashSet<_> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(&n, _)| n)
        .collect();

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back(Path {
        curr: "AA",
        elapsed: 0,
        relieved: 0,
        opened: BTreeSet::new(),
    });
    visited.insert((BTreeSet::new(), 0, 0));

    let mut max_relieved: u32 = 0;

    while let Some(Path {
        opened,
        curr,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        // If all valves opened -> compute total relieved flow, check if maximum then skip
        if opened.len() == flowing_valves.len() || elapsed >= 30 {
            let final_relieve = compute_relieved(elapsed, 30, relieved, &opened, &valves);
            max_relieved = max_relieved.max(final_relieve);
            continue;
        }

        // For every valve to open, add them (if not already visited and not) with new elapsed time and new relieved value
        let unopened = flowing_valves.iter().filter(|v| !opened.contains(*v));

        for dest in unopened {
            // If over max time, do as above -> compute total flow, check if maximum and skip
            let cost = distances[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            if new_elapsed >= 30 {
                let final_relieve = compute_relieved(elapsed, 30, relieved, &opened, &valves);
                max_relieved = max_relieved.max(final_relieve);
                continue;
            }

            let new_relieved = compute_relieved(elapsed, new_elapsed, relieved, &opened, &valves);
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if visited.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                q.push_back(Path {
                    opened: new_opened,
                    curr: dest,
                    relieved: new_relieved,
                    elapsed: new_elapsed,
                });
            }
        }
    }

    Some(max_relieved)
}

pub fn part_two(input: &str) -> Option<u32> {
    let valves = load_valves(input);
    let distances = distances(&valves);

    // Exhaustive search of all possible paths (of flowing valves) without optimizing.
    // Also accept partial paths, as they will be merged later (player + elephant)
    let flowing_valves: HashSet<_> = valves
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(&n, _)| n)
        .collect();

    let mut q = VecDeque::new();

    q.push_back(Path {
        curr: "AA",
        elapsed: 0,
        relieved: 0,
        opened: BTreeSet::new(),
    });

    let mut max_relieved_paths: HashMap<BTreeSet<&str>, u32> = HashMap::new();

    while let Some(Path {
        opened,
        curr,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        let relieved_at_end = compute_relieved(elapsed, 26, relieved, &opened, &valves);

        max_relieved_paths
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);
        if opened.len() == flowing_valves.len() || elapsed > 26 {
            continue;
        }

        let unopened = flowing_valves.iter().filter(|v| !opened.contains(*v));

        for dest in unopened {
            // If over max time, do as above -> compute total flow, check if maximum and skip
            let cost = distances[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            if new_elapsed >= 26 {
                continue;
            }

            let new_relieved = compute_relieved(elapsed, new_elapsed, relieved, &opened, &valves);
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            q.push_back(Path {
                opened: new_opened,
                curr: dest,
                relieved: new_relieved,
                elapsed: new_elapsed,
            });
        }
    }

    // Try all combinations of paths (1 for human, 1 for elephant)
    // which are disjoint, and take the maximum flow one
    let max_final: u32 = max_relieved_paths
        .iter()
        .tuple_combinations() // All combinations of 2 paths
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();

    Some(max_final)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
