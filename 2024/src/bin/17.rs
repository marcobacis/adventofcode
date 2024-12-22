use std::fs;

use itertools::Itertools;

fn part_one(input: &str) -> Option<String> {
    let mut device = Device::from(input);
    Some(device.run())
}

fn part_two(input: &str) -> Option<u64> {
    let initial = Device::from(input);

    let digits = vec![];
    for d in 0..8 {
        let res = solve_recursive(&initial.memory, &digits, d);
        if res.is_some() {
            return res;
        }
    }
    None
}

fn solve_recursive(program: &Vec<u8>, digits: &Vec<u8>, current: u8) -> Option<u64> {
    // (Based on reverse engineering of the input programs)
    // Each octal digit in the input gives an additional output value in opposite order
    // We can search the space of solutions ("a" values) using BFS, checking every time
    // that the partial output matches

    let mut current_digits: Vec<u8> = digits.clone();
    current_digits.push(current);

    let a: u64 = current_digits
        .iter()
        .fold(0u64, |acc, &digit| acc * 8 + digit as u64);
    let mut device = Device {
        a,
        b: 0,
        c: 0,
        memory: program.clone(),
        ip: 0,
        out: vec![],
    };
    device.run();

    let cursor = program.len() - (digits.len() + 1);
    let expected = &program[cursor..];

    if expected != device.out {
        return None;
    } else if cursor == 0 {
        return Some(a);
    }

    for d in 0..8 {
        let res = solve_recursive(program, &current_digits, d);
        if res.is_some() {
            return res;
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("inputs/17.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

#[derive(Clone, Copy)]
enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Device {
    a: u64,
    b: u64,
    c: u64,
    memory: Vec<u8>,
    ip: u32,
    out: Vec<u8>,
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let a: u64 = lines[0].split_once(": ").unwrap().1.parse().unwrap();
        let b: u64 = lines[1].split_once(": ").unwrap().1.parse().unwrap();
        let c: u64 = lines[2].split_once(": ").unwrap().1.parse().unwrap();
        let memory = lines[4]
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            a,
            b,
            c,
            memory,
            ip: 0,
            out: vec![],
        }
    }
}

impl Device {
    pub fn run(&mut self) -> String {
        while !self.ended() {
            self.step();
        }

        self.output()
    }

    pub fn step(&mut self) {
        if self.ended() {
            return;
        }

        let op = OpCode::from(self.memory[self.ip as usize]);
        let operand = self.memory[(self.ip + 1) as usize];

        match op {
            OpCode::Adv => {
                let denominator = u64::pow(2, self.combo_operand(operand) as u32);
                self.a /= denominator;
            }
            OpCode::Bxl => {
                self.b ^= operand as u64;
            }
            OpCode::Bst => {
                self.b = self.combo_operand(operand) % 8;
            }
            OpCode::Jnz => {
                if self.a != 0 {
                    self.ip = operand as u32;
                    return;
                }
            }
            OpCode::Bxc => {
                self.b ^= self.c;
            }
            OpCode::Out => {
                let output = self.combo_operand(operand) % 8;
                self.out.push(output as u8);
            }
            OpCode::Bdv => {
                let denominator = u64::pow(2, self.combo_operand(operand) as u32);
                self.b = self.a / denominator;
            }
            OpCode::Cdv => {
                let denominator = u64::pow(2, self.combo_operand(operand) as u32);
                self.c = self.a / denominator;
            }
        }

        self.ip += 2;
    }

    pub fn ended(&self) -> bool {
        self.ip > (self.memory.len() - 1) as u32
    }

    pub fn output(&self) -> String {
        self.out.iter().map(|n| n.to_string()).join(",")
    }

    pub fn memory_str(&self) -> String {
        self.memory.iter().map(|n| n.to_string()).join(",")
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Reserved value"),
        }
    }
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[test]
    fn can_parse_device() {
        let input = "Register A: 729
Register B: 82341
Register C: 432

Program: 0,1,5,4,3,0";

        let expected = Device {
            a: 729,
            b: 82341,
            c: 432,
            memory: vec![0, 1, 5, 4, 3, 0],
            ip: 0,
            out: vec![],
        };

        assert_eq!(expected, Device::from(input));
    }

    #[rstest]
    #[case(10, 0, 10)]
    #[case(10, 1, 5)]
    #[case(40, 2, 10)]
    #[case(40, 3, 5)]
    #[case(4, 4, 0)]
    #[case(32, 5, 4)]
    #[case(32, 6, 1)]
    fn test_division(#[case] numerator: u64, #[case] operand: u8, #[case] result: u64) {
        let mut device = Device {
            a: numerator,
            b: 3,
            c: 5,
            ip: 0,
            memory: vec![OpCode::Adv as u8, operand],
            out: vec![],
        };

        device.step();

        assert_eq!(result, device.a);
        assert_eq!(2, device.ip);
    }

    #[test]
    fn test_bxl() {
        let mut device = Device {
            a: 0,
            b: 7435,
            c: 5,
            ip: 0,
            memory: vec![OpCode::Bxl as u8, 6],
            out: vec![],
        };

        device.step();

        assert_eq!(device.b, 7437);
        assert_eq!(device.ip, 2);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 3)]
    #[case(4, 7)]
    #[case(5, 3)]
    #[case(6, 2)]
    fn test_bst(#[case] operand: u8, #[case] expected: u8) {
        let mut device = Device {
            a: 31,
            b: 67,
            c: 10,
            ip: 0,
            memory: vec![OpCode::Bst as u8, operand],
            out: vec![],
        };

        device.step();

        assert_eq!(device.b, expected as u64);
        assert_eq!(device.ip, 2);
    }

    #[rstest]
    #[case(0, 2, 2)]
    #[case(1, 2, 2)]
    #[case(2, 3, 3)]
    #[case(6, 5, 5)]
    fn test_jnz(#[case] a: u64, #[case] operand: u8, #[case] expected: u32) {
        let mut device = Device {
            a,
            b: 0,
            c: 0,
            ip: 0,
            memory: vec![OpCode::Jnz as u8, operand],
            out: vec![],
        };

        device.step();

        assert_eq!(device.ip, expected);
    }

    #[rstest]
    #[case(10, 5, 15)]
    #[case(34, 12, 46)]
    #[case(68, 543, 603)]
    fn test_bxc(#[case] b: u64, #[case] c: u64, #[case] expected: u64) {
        let mut device = Device {
            a: 0,
            b,
            c,
            ip: 0,
            memory: vec![OpCode::Bxc as u8, 0],
            out: vec![],
        };

        device.step();

        assert_eq!(expected, device.b);
        assert_eq!(device.ip, 2);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 3)]
    #[case(4, 4)]
    #[case(5, 0)]
    #[case(6, 2)]
    fn test_out(#[case] operand: u8, #[case] expected: u8) {
        let mut device = Device {
            a: 292,
            b: 184,
            c: 8234,
            ip: 0,
            memory: vec![OpCode::Out as u8, operand],
            out: vec![],
        };

        device.step();

        assert_eq!(expected, device.out[0]);
        assert_eq!(device.ip, 2);
    }

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/17_1.txt").unwrap();
        assert_eq!(Some(String::from("4,6,3,5,6,3,5,2,1,0")), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/17_2.txt").unwrap();
        assert_eq!(Some(117440), part_two(&input));
    }
}
