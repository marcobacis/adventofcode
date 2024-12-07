use std::fs;

fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(y as i32, x as i32) == 'X' {
                let directions = vec![(-1,-1),(-1,0),(-1, 1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
                for d in directions {
                    if grid.get_str(y, x, d, 4) == "XMAS" {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
}

fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(y as i32,x as i32) == 'A' && grid.check_x_mas(y, x){
                count += 1;
            }
        }
    }

    Some(count)
}


struct Grid {
    grid: Vec<char>,
    height: usize,
    width: usize,
}


impl Grid {

    pub fn new(input: &str) -> Self {
        let lines : Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().unwrap().len();

        Grid {
            height,
            width,
            grid: input.chars().filter(|c| !c.is_whitespace()).collect()
        }
    }

    fn get(&self, y: i32, x: i32) -> char {
        self.grid[y as usize * self.width + x as usize]
    }

    fn is_inside(&self, y: i32, x: i32) -> bool {
        y >= 0 && y < self.height as i32 && x >= 0 && x < self.width as i32
    }

    fn get_str(&self, y: usize, x: usize, dir: (i32, i32), length: usize) -> String {
        let coords = (0..length).map(|i| (i as i32 * dir.0 + y as i32, i as i32 * dir.1 + x as i32));
        self.get_coords(coords)
    }

    fn get_coords(&self, coords: impl Iterator<Item =(i32, i32)>) -> String {
        coords.filter(|(ry,rx)| self.is_inside(*ry, *rx))
        .map(|(ry,rx)| self.get(ry,rx))
        .collect()
    }
    
    fn check_x_mas(&self, y: usize, x: usize) -> bool {
        let directions = vec![(-1,-1),(-1, 1),(1,-1),(1,1)];
        let coords = directions.iter().map(|(dy,dx)| (y as i32 + dy, x as i32 + dx));

        let s : String = self.get_coords(coords);

        match s.as_str() {
            "MSMS" |"MMSS" | "SMSM" |"SSMM" => true,
            _ => false,
        }
    }

}


fn main() {
    let input = fs::read_to_string("inputs/04.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[test]
    fn can_create_grid() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM";

        let grid = Grid::new(&input);

        assert_eq!(3, grid.height);
        assert_eq!(10, grid.width);
    }

    
    #[rstest]
    #[case(0,0,(1,1),3, "MSX")]
    #[case(2,2,(-1,-1),3, "XSM")]
    #[case(2,2,(0,1),4, "XSXM")]
    #[case(2,2,(1,0),4, "XAAA")]
    fn test_get_str(#[case] y: usize, #[case] x: usize, #[case] dir: (i32, i32), #[case] l: usize, #[case] expected: String) {
        let input = fs::read_to_string("examples/04.txt").unwrap();
        let grid = Grid::new(&input);

        let s = grid.get_str(y,x, dir, l);
        assert_eq!(expected, s);
    }


    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(Some(18), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(Some(9), part_two(&input));
    }

}