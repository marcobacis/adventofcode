use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs};

use petgraph::{dot::{Config, Dot}, graph::NodeIndex, Graph};

fn load_inputs(input: &str) -> (HashSet<(u32,u32)>, Vec<Vec<u32>>){
    let mut lines = input.lines();

    let mut rules : HashSet<(u32,u32)> = HashSet::new();
    
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (prev, next) = line.split_once('|').unwrap();
        let prev : u32 = prev.parse().unwrap();
        let next : u32 = next.parse().unwrap();

        rules.insert((prev,next));
    }

    let sequences : Vec<Vec<u32>> = lines.map(|line| line.split(',').map(|s| s.parse::<u32>().unwrap()).collect()).collect();

    (rules, sequences)
}

fn comparator<'d>(rules: &'d HashSet<(u32,u32)>) -> impl Fn(&u32,&u32) -> Ordering + 'd {
    move |a: &u32,b: &u32| { if rules.contains(&(*a,*b)) {
            return Ordering::Less;
        } else if rules.contains(&(*b,*a)) {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

fn part_one(input: &str) -> Option<u32> {
    let (rules, sequences) = load_inputs(input);
    let comparator = comparator(&rules);
    Some(sequences.iter()
        .filter(|s| s.is_sorted_by(|a,b| comparator(&a,&b).is_le()))
        .map(|s| s[s.len()/2]).sum())
}

fn part_two(input: &str) -> Option<u32> {
    let (rules, sequences) = load_inputs(input);

    let comparator = comparator(&rules);

    Some(sequences.iter().filter(|s| !s.is_sorted_by(|a,b| comparator(&a,&b).is_le()))
        .map(|s| {
            let mut s = s.to_vec();
            s.sort_by(&comparator);
            s
        })
        .map(|s| s[s.len()/2]).sum())
}


fn main() {
    let input = fs::read_to_string("examples/05.txt").unwrap();

    // println!("Solutions 🎄");
    // let result_part_one = part_one(&input);
    // let result_part_two = part_two(&input);

    // if let Some(res) = result_part_one {
    //     println!("Part 1: {}", res);
    // }
    // if let Some(res) = result_part_two {
    //     println!("Part 2: {}", res);
    // }

    let mut graph = Graph::<_, ()>::new();

    let (rules, _) = load_inputs(&input);

    let nodes : HashSet<u32> = rules.iter().flat_map(|(a,b)| vec![*a,*b]).collect();
    let nodes :HashMap<u32,NodeIndex<u32>>= nodes.iter().map(|n| (*n, graph.add_node(*n))).collect();


    for (prev,next) in rules {
        graph.add_edge(*nodes.get(&prev).unwrap(), *nodes.get(&next).unwrap(), ());        
    }

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(Some(143), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(Some(123), part_two(&input));
    }

}