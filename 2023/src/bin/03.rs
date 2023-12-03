use std::ops::Range;

#[derive(Debug)]
struct Number {
    y: usize,
    x: Range<usize>,
    value: u32,
}

impl Number {
    pub fn is_adj(&self, s: &Symbol) -> bool {
        let y0 = (self.y as isize - 1).max(0) as usize;
        let yrange = y0..(self.y + 2);

        let x0 = (self.x.start as isize - 1).max(0) as usize;
        let xrange = x0..(self.x.end + 1);

        yrange.contains(&s.y) && xrange.contains(&s.x)
    }
}

#[derive(Debug)]
struct Symbol {
    y: usize,
    x: usize,
    value: char,
}

fn get_symbols(input: &str) -> Vec<Symbol> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (!c.is_digit(10) && c != '.').then_some(Symbol {
                    y: y.clone(),
                    x: x.clone(),
                    value: c.clone(),
                })
            })
        })
        .collect()
}

fn get_numbers(input: &str) -> Vec<Number> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split(|c: char| !c.is_ascii_digit())
                .filter_map(move |sub| {
                    let start = sub.as_ptr() as usize - line.as_ptr() as usize;
                    let value = sub.parse::<u32>().ok();
                    match value {
                        Some(val) => Some(Number {
                            y: y,
                            x: start..(start + sub.len()),
                            value: val,
                        }),
                        None => None,
                    }
                })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let num_list = get_numbers(&input);
    let symbols = get_symbols(&input);

    Some(
        num_list
            .iter()
            .filter_map(|num| symbols.iter().any(|s| num.is_adj(s)).then_some(num.value))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_list = get_numbers(&input);
    let symbols = get_symbols(&input);

    Some(
        symbols
            .iter()
            .filter(|s| s.value == '*')
            .map(|s| {
                num_list
                    .iter()
                    .filter_map(|n| n.is_adj(&s).then_some(n.value))
                    .collect::<Vec<u32>>()
            })
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums[0] * nums[1])
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4361));
    }

    #[test]
    fn test_solve_part_one() {
        let input = advent_of_code::read_file("inputs", 3);
        assert_eq!(part_one(&input), Some(532331));
    }

    #[test]
    fn test_example_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(467835));
    }

    #[test]
    fn test_solve_part_two() {
        let input = advent_of_code::read_file("inputs", 3);
        assert_eq!(part_two(&input), Some(82301120));
    }
}
