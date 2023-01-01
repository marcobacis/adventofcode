enum Jet {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Default)]
struct Coord {
    x: usize,
    y: usize,
}

const WIDTH: usize = 7;
const SHAPES: [&[Coord]; 5] = [
    // 0: ####
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 3, y: 0 },
    ],
    // 2: .#.
    // 1: ###
    // 0: .#.
    &[
        Coord { x: 1, y: 2 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
        Coord { x: 2, y: 1 },
        Coord { x: 1, y: 0 },
    ],
    // 2: ..#
    // 1: ..#
    // 0: ###
    &[
        Coord { x: 2, y: 2 },
        Coord { x: 2, y: 1 },
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
    ],
    // 3: #
    // 2: #
    // 1: #
    // 0: #
    &[
        Coord { x: 0, y: 3 },
        Coord { x: 0, y: 2 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: 0 },
    ],
    // 1: ##
    // 0: ##
    &[
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
    ],
];

fn parse_input(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid input char {}", c),
        })
        .collect()
}

#[derive(Default)]
struct Cave<'a> {
    curr_shape: &'a [Coord],
    curr_jet: usize,
    curr_pos: Coord,
    jets: Vec<Jet>,
    top: usize,
    map: Vec<[bool; WIDTH]>,
    rocks_count: usize,
}

impl Cave<'_> {
    fn add_rock_on_top(&mut self) {
        self.curr_pos.x = 2;
        self.curr_pos.y = self.top + 3;
        self.curr_shape = SHAPES[self.rocks_count % SHAPES.len()];
    }

    fn apply_jet(&mut self) {
        let jet = &self.jets[self.curr_jet % self.jets.len()];
        let new_coord = match jet {
            Jet::Left => Coord {
                x: self.curr_pos.x.saturating_sub(1),
                y: self.curr_pos.y,
            },
            Jet::Right => Coord {
                x: self.curr_pos.x + 1,
                y: self.curr_pos.y,
            },
        };
        if self.can_move_to(&new_coord) {
            self.curr_pos = new_coord;
        }
        self.curr_jet += 1;
    }

    fn move_down(&mut self) -> bool {
        let new_coord = Coord {
            x: self.curr_pos.x,
            y: self.curr_pos.y.saturating_sub(1),
        };
        if self.curr_pos.y == 0 || !self.can_move_to(&new_coord) {
            return false;
        }
        self.curr_pos = new_coord;
        true
    }

    fn settle_current_rock(&mut self) {
        for offset in self.curr_shape {
            let x = self.curr_pos.x + offset.x;
            let y = self.curr_pos.y + offset.y;
            while self.map.len() <= y {
                self.map.push([false; WIDTH]);
            }
            self.map[y][x] = true;
            self.top = self.top.max(y + 1);
        }
    }

    fn can_move_to(&self, coord: &Coord) -> bool {
        self.curr_shape.iter().all(|offset| {
            let x = coord.x + offset.x;
            let y = coord.y + offset.y;
            if y >= self.map.len() {
                x < WIDTH
            } else {
                x < WIDTH && !self.map[y][x]
            }
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cave = Cave {
        jets: parse_input(input),
        curr_jet: 0,
        curr_shape: &SHAPES[0],
        curr_pos: Coord { x: 0, y: 0 },
        rocks_count: 0,
        top: 0,
        map: Vec::new(),
    };

    while cave.rocks_count != 2022 {
        cave.add_rock_on_top();
        loop {
            cave.apply_jet();

            if !cave.move_down() {
                break;
            }
        }

        cave.settle_current_rock();

        cave.rocks_count += 1;
    }

    Some(cave.top as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
