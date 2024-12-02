use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/02.txt").unwrap();

    println!("Solutions 🎄");
    let result_part_one = part_one(&input);
    let result_part_two = part_two(&input);

    if let Some(res) = result_part_one {
        println!("Part 1: {}", res);
    }
    if let Some(res) = result_part_two {
        println!("Part 2: {}", res);
    }
}

fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| {
        let report  = extract_numbers(l);    
        is_safe(&report)
    }).filter(|safe| *safe).count() as u32)
}

fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| {
        let report  = extract_numbers(l);    
        is_safe_dampener(&report)
    }).filter(|safe| *safe).count() as u32)
}

fn extract_numbers(report: &str) -> Vec<i32> {
    let numbers : Vec<i32> = report.split(' ').map(|n| n.parse::<i32>().unwrap()).collect();
    numbers
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    let diffs = compute_diffs(numbers);
    let distances_ok = diffs.iter().map(|d| d.abs()).all(|d| (1..=3).contains(&d));
    let increasing = diffs.iter().all(|d| *d > 0);
    let decreasing = diffs.iter().all(|d| *d < 0);

    distances_ok && (increasing || decreasing)
}

fn is_safe_dampener(numbers: &Vec<i32>) -> bool {
    if is_safe(numbers) {
        return true;
    }

    let len = numbers.len();
    (0..len).any(|i| {
        let mut removed : Vec<i32> = numbers.to_vec();
        removed.remove(i);
        is_safe(&removed)
    })
}

fn compute_diffs(numbers: &Vec<i32>) -> Vec<i32> {
    let mut diffs : Vec<i32> = vec![0 ; numbers.len() - 1];
    for i in 0..(numbers.len()-1) {
        diffs[i] = numbers[i] - numbers[i+1];
    }
    diffs
}


#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", false)]
    #[case("8 6 4 4 1", false)]
    #[case("1 3 6 7 9", true)]
    fn safe_report_test(#[case] report: &str, #[case] safe: bool) {
        let report = extract_numbers(report);
        assert_eq!(safe, is_safe(&report));
    }

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", true)]
    #[case("8 6 4 4 1", true)]
    #[case("9 7 6 5 1", true)]
    #[case("9 7 6 -3 1", false)]
    #[case("9 6 -6 3 1", true)]
    #[case("1 3 6 7 9", true)]
    fn safe_report_with_dampener_test(#[case] report: &str, #[case] safe: bool) {
        let report = extract_numbers(report);
        assert_eq!(safe, is_safe_dampener(&report));
    }

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(Some(2), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(Some(4), part_two(&input));
    }

}