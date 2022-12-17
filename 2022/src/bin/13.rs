use serde_json::{json, Value};
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Value> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l.trim()).unwrap())
        .collect();

    let result = lines
        .chunks(2)
        .map(|v| v.to_vec())
        .enumerate()
        .filter(|(_, v)| compare(&v[0], &v[1]).is_le())
        .map(|(i, _)| i + 1)
        .fold(0, |a, c| a + c);

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: Vec<Value> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l.trim()).unwrap())
        .collect();

    lines.push(serde_json::from_str("[[2]]").unwrap());
    lines.push(serde_json::from_str("[[6]]").unwrap());

    lines.sort_by(|a, b| compare(a, b));

    let first = lines.iter().position(|v| v.to_string() == "[[2]]").unwrap() + 1;
    let second = lines.iter().position(|v| v.to_string() == "[[6]]").unwrap() + 1;

    Some((first * second) as u32)
}

fn compare(a: &Value, b: &Value) -> std::cmp::Ordering {
    let res = match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_u64().cmp(&b.as_u64()),
        (Value::Array(a), Value::Array(b)) => {
            if a.is_empty() || b.is_empty() {
                return a.len().cmp(&b.len());
            } else {
                let res = compare(&json!(a[0]), &json!(b[0]));
                if res.is_eq() {
                    compare(&json!(a[1..]), &json!(b[1..]))
                } else {
                    res
                }
            }
        }
        (Value::Number(a), Value::Array(b)) => compare(&json!(vec![a]), &json!(b)),
        (Value::Array(a), Value::Number(b)) => compare(&json!(a), &json!(vec![b])),
        _ => Ordering::Greater,
    };

    res
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
