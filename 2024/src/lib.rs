use std::{fmt::Display, time::Instant};

pub mod grid;
pub mod coordinate;

pub fn solve<T: Display>(part: usize, input: &str, solve_fn: impl Fn(&str) -> Option<T>) {
    let start = Instant::now();
    let result =  solve_fn(input);
    let duration = start.elapsed();

    if let Some(res) = result {
        println!("🎄 Part {} solution: {} (took {:?})", part, res, duration);
    }
}