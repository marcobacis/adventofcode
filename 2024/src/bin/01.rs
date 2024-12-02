use std::str::FromStr;
use std::iter::zip;
use std::fs;

fn part_one(input: &str) -> u32 {
    
    let (mut left, mut right) : (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("   ").map(|(n1, n2)| (i32::from_str(n1).unwrap(), i32::from_str(n2).unwrap())).unwrap())
        .unzip();

    left.sort();
    right.sort();


    let sum = zip(left, right).map(|(n1,n2)| (n1-n2).abs()).sum::<i32>() as u32;

    sum
}

fn part_two(input: &str) -> u32 {
    
    let (left, right) : (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("   ").map(|(n1, n2)| (i32::from_str(n1).unwrap(), i32::from_str(n2).unwrap())).unwrap())
        .unzip();

    left.iter().map(|l| (*l as usize) * right.iter().filter(|r| l == *r).count()).sum::<usize>() as u32
}


fn main() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    
    println!("Solution for part 1: {}", part_one(&input));
    println!("Solution for part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_one_test() {

        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(11, part_one(input));
    }

    #[test]
    fn part_two_test() {

        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(31, part_two(input));
    }

}