use advent_of_code::helpers::get_numbers;
use std::cmp::{min, max};

struct Converter {
    pub map: Vec<(ExtRange, ExtRange)>,
}

impl Converter {
    pub fn new(str: &str) -> Self {
        let map = str
            .lines()
            .skip(1)
            .map(|l| get_numbers(&l))
            .map(|nums| (ExtRange::new(nums[1], nums[2]), ExtRange::new(nums[0], nums[2])))
            .collect();

        Converter { map }
    }

    pub fn convert(&self, source: usize) -> usize {
        let find_result = self.map.iter().find(|(range, _)| range.contains(source));
        if let Some((range, dest)) = find_result {
            return dest.start + (source - range.start);
        }
        source
    }

    pub fn convert_range(&self, source: ExtRange) -> Vec<ExtRange> {
        let mut output : Vec<ExtRange> = Vec::new();
        let mut intersections : Vec<ExtRange> = Vec::new();

        for (src, dest) in &self.map {
            if let Some(intersection) = src.intersection(&source) {
                output.push(ExtRange {
                    start: dest.start + (intersection.start - src.start),
                    end: dest.start + (intersection.end - src.start),
                });
                intersections.push(intersection);
            }
        }

        output.extend(source.remove_ranges(&intersections));
        output
    }
}

#[derive(Eq, Debug, PartialEq, PartialOrd, Clone)]
struct ExtRange {
    pub start: usize, 
    pub end: usize,
}

impl ExtRange {
    pub fn new(start:usize, size: usize) -> Self {
        ExtRange {
            start,
            end: start+size,
        }
    }

    pub fn intersection(&self, other: &ExtRange) -> Option<ExtRange> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if end > start {
            Some(ExtRange {start, end})
        } else {
            None
        }
    }

    pub fn contains(&self, value: usize) -> bool {
        value >= self.start && value < self.end
    }

    pub fn remove_ranges(&self, to_remove: &Vec<ExtRange>) -> Vec<ExtRange> {
        let mut to_remove_sorted = to_remove.to_vec();
        to_remove_sorted.sort_by(|a, b| a.cmp(&b));

        let mut output : Vec<ExtRange> = Vec::new();
        let mut pos = self.start;
        for r in to_remove_sorted.iter() {
            if r.start > pos {
                output.push(ExtRange {start: pos, end: r.start});
                pos = r.end;
                continue;
            }
            if r.end > pos {
                pos = r.end;
            }
        }
        if pos < self.end {
            output.push(ExtRange {start: pos, end: self.end});
        }

        output
    }
}

impl Ord for ExtRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        if self.end < other.end  {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
}

fn find_min_convert(seeds: Vec<usize>, converters: &Vec<Converter>) -> u32 {
    let min_val = seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for converter in converters {
                value = converter.convert(value);
            }
            value
        })
        .min()
        .unwrap();

    min_val as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");

    let seeds = get_numbers(split.next().unwrap());
    let converters: Vec<Converter> = split.map(|s| Converter::new(s)).collect();

    Some(find_min_convert(seeds, &converters))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut split = input.split("\n\n");

    let seeds : Vec<ExtRange> = get_numbers(split.next().unwrap())
        .chunks(2)
        .map(|n| ExtRange::new(n[0], n[1]))
        .collect();
        
    let converters: Vec<Converter> = split.map(|s| Converter::new(s)).collect();

    let mut ranges = seeds.to_vec();
    for converter in &converters {
        ranges = ranges.iter().flat_map(|range| converter.convert_range(range.clone())).collect();
    }

    Some(ranges.iter().map(|r| r.start).min().unwrap() as u32)
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_intersection() {
        let r = ExtRange{start: 10, end: 20};
        let a = ExtRange {start: 7, end: 12};
        let b = ExtRange {start: 20, end: 23};

        assert_eq!(Some(ExtRange{start: 10, end: 12}), r.intersection(&a));
        assert_eq!(None, r.intersection(&b));
    }

    #[test]
    fn test_range_removal() {
        let r = ExtRange{start: 5, end: 20};
        let ranges = vec![ExtRange{start: 2, end: 7}, ExtRange{start: 10, end: 15}];
        assert_eq!(r.remove_ranges(&ranges), vec![ExtRange{start: 7, end: 10}, ExtRange{start: 15, end: 20}]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(46));
    }
}
