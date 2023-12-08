pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = advent_of_code::helpers::get_numbers(lines.next().unwrap());
    let distances = advent_of_code::helpers::get_numbers(lines.next().unwrap());

    let matches = times.iter().zip(distances.iter());

    Some(matches.map(|(tmax, dmax)| find_times(&tmax, &dmax)).product::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let tmax = lines.next().unwrap().chars().skip(5).filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap();
    let dmax = lines.next().unwrap().chars().skip(9).filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap();
    
    Some(find_times(&tmax, &dmax))
}

fn find_times(tmax: &usize, dmax: &usize) -> u32 {
    (0..*tmax).map(|h| (tmax - h) * h).filter(|d| d > dmax).count() as u32
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(288));
    }

    #[test]
    fn test_part_two_example() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(71503));
    }
}
