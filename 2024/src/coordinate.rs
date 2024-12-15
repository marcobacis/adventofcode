use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Coordinate {
    pub y: i32,
    pub x: i32,
}

pub const NORTH: Coordinate = Coordinate { y: -1, x: 0 };
pub const SOUTH: Coordinate = Coordinate { y: 1, x: 0 };
pub const EAST: Coordinate = Coordinate { y: 0, x: 1 };
pub const WEST: Coordinate = Coordinate { y: 0, x: -1 };

impl Coordinate {
    pub fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }

    pub fn around(&self) -> Vec<Coordinate> {
        [NORTH, EAST, SOUTH, WEST]
            .iter()
            .map(|d| *self + *d)
            .collect()
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl<T> Mul<T> for Coordinate
where
    T: Into<i32> + Copy, // Ensures T can be converted into i32
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let scalar = rhs.into();
        Self {
            y: self.y * scalar,
            x: self.x * scalar,
        }
    }
}

impl<T> Div<T> for Coordinate
where
    T: Into<i32> + Copy, // Ensures T can be converted into i32
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let scalar = rhs.into();
        Self {
            y: self.y / scalar,
            x: self.x / scalar,
        }
    }
}
