use std::fs;

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("Not a valid direction".to_string()),
        }
    }
}

struct Command {
    direction: Direction,
    distance: u32,
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Command {
            direction: value.chars().next().ok_or("String not long enough".to_string())?.try_into()?,
            distance: value.chars().skip(1).collect::<String>().parse().or(Err("Distance is not a number"))?,
        })
    }
}

struct Lock {
    position: u32,
}

impl Default for Lock {
    fn default() -> Self {
        Self { position: 50 }
    }
}

impl Lock {

    pub fn at_0(&self) -> bool {
        self.position == 0
    }

    pub fn click(&mut self, command: &Command) -> u32 {
        let mut count = 0;
        for _ in 0..command.distance {
            match command.direction {
                Direction::Left => {
                    self.position = ((self.position as i32 - 1) % 100) as u32;
                }
                Direction::Right => {
                    self.position = ((self.position as i32 + 1) % 100) as u32;
                }
            };
            if self.at_0() {
                count += 1;
            }
        }
        count
    }
}

fn part_one(input: &str) -> Option<u32> {
    let mut lock = Lock::default();
    Some(
        input
            .lines()
            .filter_map(|l| Command::try_from(l).ok())
            .filter(|c| {
                lock.click(&c);
                lock.at_0()
            })
            .count() as u32
    )
}

fn part_two(input: &str) -> Option<u32> {
    let mut lock = Lock::default();
    Some(
        input
            .lines()
            .filter_map(|l| Command::try_from(l).ok())
            .map(|c| lock.click(&c))
            .sum()
    )
}

fn main() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/01.txt").unwrap();
        assert_eq!(Some(3), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/01.txt").unwrap();
        assert_eq!(Some(6), part_two(&input));
    }
}
