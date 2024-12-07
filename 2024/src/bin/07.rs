use std::fs;

fn part_one(input: &str) -> Option<u64> {
    solve(input, is_valid)
}

fn part_two(input: &str) -> Option<u64> {
    solve(input, is_valid_with_concat)
}

fn solve(input: &str, valid_fn: impl Fn(u64, &Vec<u64>) -> bool) -> Option<u64> {
    Some(input.lines().filter_map(|line| {
        let (result, equation) = line.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let equation = equation.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();

        if valid_fn(result, &equation) {
            Some(result)
        } else {
            None
        }
    }).sum::<u64>())
}


fn main() {
    let input = fs::read_to_string("inputs/07.txt").unwrap();

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

fn is_valid_helper(result: u64, equation: &Vec<u64>, operators: &Vec<impl Fn(u64,u64) -> u64>, acc: u64, idx: usize) -> bool {
    if idx == equation.len()  {
        return acc == result;
    }
    if acc > result {
        return false;
    }
    operators.iter().any(|op| is_valid_helper(result, equation, operators, op(acc, equation[idx]), idx + 1))
}

fn is_valid(result: u64, equation: &Vec<u64>) -> bool {

    let operators = vec![
        |a,b| a + b,
        |a,b| a * b,
    ];

    is_valid_helper(result, equation, &operators,equation[0], 1)
}

fn is_valid_with_concat(result: u64, equation: &Vec<u64>) -> bool {
    let operators = vec![
        |a,b| a + b,
        |a,b| a * b,
        |a,b| concat(a,b),
    ];
    is_valid_helper(result, equation, &operators,  equation[0], 1)
}

fn concat(left: u64, right: u64) -> u64 {
    left * 10u64.pow(right.ilog10() + 1) + right
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(190, vec![10, 19], true)]
    #[case(3267, vec![81, 40, 27], true)]
    #[case(83, vec![17, 5], false)]
    #[case(156, vec![15, 6], false)]
    #[case(7290, vec![6, 8, 6, 15], false)]
    #[case(161011, vec![16, 10, 13], false)]
    #[case(192, vec![17, 8, 14], false)]
    #[case(21037, vec![9, 7, 18, 13], false)]
    #[case(292, vec![11, 6, 16, 20], true)]
    fn test_equation_check(#[case] result: u64, #[case] equation: Vec<u64>, #[case] valid: bool) {
        assert_eq!(valid, is_valid(result, &equation));
    }

    #[rstest]
    #[case(190, vec![10, 19], true)]
    #[case(3267, vec![81, 40, 27], true)]
    #[case(83, vec![17, 5], false)]
    #[case(156, vec![15, 6], true)]
    #[case(7290, vec![6, 8, 6, 15], true)]
    #[case(161011, vec![16, 10, 13], false)]
    #[case(192, vec![17, 8, 14], true)]
    #[case(21037, vec![9, 7, 18, 13], false)]
    #[case(292, vec![11, 6, 16, 20], true)]
    fn test_equation_check_with_concat(#[case] result: u64, #[case] equation: Vec<u64>, #[case] valid: bool) {
        assert_eq!(valid, is_valid_with_concat(result, &equation));
    }


    #[test]
    fn test_concat() {
        assert_eq!(1122,concat(11,22));
        assert_eq!(32100,concat(32,100));
        assert_eq!(486,concat(48,6));
    }

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(Some(3749), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(Some(11387), part_two(&input));
    }

}