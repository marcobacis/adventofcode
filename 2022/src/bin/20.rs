fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

struct MixingList {
    // Original values
    values: Vec<i64>,
    // indices[i] => index of the value in the mixed array at position i (to mixed[i] = values[indices[i]])
    indices: Vec<i64>,
    key: i64,
    rounds: usize,
}

impl MixingList {
    fn mix(&mut self) {
        for _ in 0..self.rounds {
            for i in 0..self.values.len() {
                let val = self.values[i] * self.key;

                // Find position of the value at position in the mixed array
                let idx = self.indices.iter().position(|&x| x == i as i64).unwrap();
                let max_idx = self.indices.len() as i64 - 1;

                // New position in which to put the value
                let new_idx = (((idx as i64) + val).rem_euclid(max_idx)) as usize;

                self.indices.remove(idx);
                self.indices.insert(new_idx, i as i64);
            }
        }
    }

    fn solve(&mut self) -> i64 {
        self.mix();

        let zero_pos = self.values.iter().position(|&n| n == 0).unwrap();
        let idx = self
            .indices
            .iter()
            .position(|&n| n == zero_pos as i64)
            .unwrap();
        return [1000, 2000, 3000]
            .iter()
            .map(|x| {
                let index = self.indices[(idx + x) as usize % self.indices.len()] as usize;
                self.values[index] * self.key
            })
            .sum();
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let values = parse(input);
    let indices = (0..(values.len() as i64)).collect();
    let mut list = MixingList {
        values,
        indices,
        key: 1,
        rounds: 1,
    };

    Some(list.solve())
}

pub fn part_two(input: &str) -> Option<i64> {
    let values = parse(input);
    let indices = (0..(values.len() as i64)).collect();
    let mut list = MixingList {
        values,
        indices,
        key: 811589153,
        rounds: 10,
    };

    Some(list.solve())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
