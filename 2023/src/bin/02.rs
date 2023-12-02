use std::cmp::max;

use regex::Regex;

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    pub fn new(str: &str) -> Self {
        // Get id
        let split: Vec<&str> = str.split(":").take(2).collect();
        let id = split[0][5..].parse::<u32>().unwrap();

        let re = Regex::new(r"(\d+) (\w+)").unwrap();

        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;

        for c in re.captures_iter(str) {
            let amount = c.get(1).unwrap().as_str().parse::<u32>().unwrap();

            match c.get(2).unwrap().as_str() {
                "red" => red = max(red, amount),
                "blue" => blue = max(blue, amount),
                "green" => green = max(green, amount),
                _ => {}
            };
        }

        Game {
            id,
            red,
            blue,
            green,
        }
    }

    pub fn is_ok(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| Game::new(l))
            .filter(|g| g.is_ok(12, 13, 14))
            .map(|g| g.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| Game::new(l).power()).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(2286));
    }
}
