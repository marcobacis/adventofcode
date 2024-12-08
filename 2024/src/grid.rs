use std::ops;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinate {
    pub y: i32,
    pub x: i32,
}

impl Coordinate {
    pub fn new(y: i32, x: i32) -> Self {
        Self {y,x}
    }
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            y: self.y + rhs.y,
            x: self.x + rhs.x
        }
    }
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            y: self.y - rhs.y,
            x: self.x - rhs.x
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    grid: Vec<char>,
    pub height: usize,
    pub width: usize,
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

    pub fn get(&self, coord: &Coordinate) -> Option<char> {
        if !self.is_inside(coord) {
            return None;
        }
        Some(self.grid[coord.y as usize * self.width + coord.x as usize])
    }

    pub fn set(&mut self, coordinate: Coordinate, c: char) {
        if !self.is_inside(&coordinate) {
            return;
        }

        self.grid[coordinate.y as usize * self.width + coordinate.x as usize] = c;
    }

    pub fn is_inside(&self, coordinate: &Coordinate) -> bool {
        coordinate.y >= 0 && coordinate.y < self.height as i32 && coordinate.x >= 0 && coordinate.x < self.width as i32
    }
    
    pub fn find_first(&self, arg: char) -> Option<Coordinate> {
        for y in 0..self.height {
            for x in 0..self.width {
                let coordinate =Coordinate{y: y as i32,x: x as i32};
                if self.get(&coordinate).unwrap() == arg {
                    return Some(coordinate)
                }
            }
        }
        None
    }
}