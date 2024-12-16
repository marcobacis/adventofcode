use std::{fmt::Debug, str::FromStr};

use crate::coordinate::Coordinate;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    grid: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn new(input: &str, parser: impl Fn(&str) -> T, separator: char) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines
            .first()
            .map(|line| line.split(separator).count())
            .unwrap_or(0);

        let grid = lines
            .iter()
            .flat_map(|line| line.split(separator).map(&parser))
            .collect();

        Self {
            height,
            width,
            grid,
        }
    }

    pub fn get(&self, coord: &Coordinate) -> Option<&T> {
        if !self.is_inside(coord) {
            return None;
        }
        Some(&self.grid[coord.y as usize * self.width + coord.x as usize])
    }

    pub fn set(&mut self, coordinate: Coordinate, value: T) {
        if !self.is_inside(&coordinate) {
            return;
        }

        self.grid[coordinate.y as usize * self.width + coordinate.x as usize] = value;
    }

    pub fn is_inside(&self, coordinate: &Coordinate) -> bool {
        coordinate.y >= 0
            && coordinate.y < self.height as i32
            && coordinate.x >= 0
            && coordinate.x < self.width as i32
    }
}

impl Grid<char> {
    pub fn new_chars(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().unwrap_or(&"").len();

        let grid = input.chars().filter(|c| !c.is_whitespace()).collect();

        Grid {
            height,
            width,
            grid,
        }
    }

    pub fn find_first(&self, arg: char) -> Option<Coordinate> {
        for y in 0..self.height {
            for x in 0..self.width {
                let coordinate = Coordinate {
                    y: y as i32,
                    x: x as i32,
                };
                if *self.get(&coordinate).unwrap() == arg {
                    return Some(coordinate);
                }
            }
        }
        None
    }
}

impl<T> Grid<T>
where
    T: FromStr + Copy,
    <T as FromStr>::Err: Debug,
{
    pub fn new_numeric(input: &str, separator: char) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines
            .first()
            .map(|line| line.split(separator).count())
            .unwrap_or(0);

        let grid = lines
            .iter()
            .flat_map(|line| line.split(separator).map(|item| item.parse::<T>().unwrap()))
            .collect();

        Grid {
            height,
            width,
            grid,
        }
    }

    pub fn new_numeric_chars(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|line| line.chars().count()).unwrap_or(0);

        let grid = lines
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|item| item.to_string().parse::<T>().unwrap())
            })
            .collect();

        Grid {
            height,
            width,
            grid,
        }
    }
}
