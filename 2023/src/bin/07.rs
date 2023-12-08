use std::{cmp::Ordering, collections::HashMap, fs::create_dir};

use itertools::enumerate;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
enum HandKind {
    High,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<char>,
    kind: HandKind,
    pub variant: bool,
}

impl Hand {
    pub fn new(str: &str, variant: bool) -> Self {
        let cards = str.chars();
        let mut hand = Hand {
            cards: cards.collect(),
            kind: Self::kind(str),
            variant,
        };

        if variant && hand.cards.contains(&'J') {
            hand.maximise_kind();
        }

        hand
    }

    fn maximise_kind(&mut self) {
        let cards: String = self.cards.iter().filter(|c| **c != 'J').collect();
        let num_js = self.cards.iter().filter(|c| **c == 'J').count();

        let mut occ = Self::get_card_occurrences(&cards);
        if occ.len() == 0 {
            // All Js
            occ.push(num_js);
        } else {
            occ[0] = occ[0] + num_js;
        }

        self.kind = Self::kind_from_occurrences(&occ);
    }

    fn get_card_occurrences(cards: &str) -> Vec<usize> {
        let mut map: HashMap<char, usize> = HashMap::new();
        for card in cards.chars() {
            match map.get(&card) {
                Some(count) => map.insert(card, count + 1),
                None => map.insert(card, 1),
            };
        }

        let mut cards: Vec<usize> = map.values().map(|v| *v).collect();
        cards.sort();
        cards.reverse();

        cards
    }

    fn kind_from_occurrences(cards: &Vec<usize>) -> HandKind {
        match cards[0] {
            5 => HandKind::Five,
            4 => HandKind::Four,
            3 => {
                if cards[1] == 2 {
                    HandKind::FullHouse
                } else {
                    HandKind::Three
                }
            }
            2 => {
                if cards[1] == 2 {
                    HandKind::Two
                } else {
                    HandKind::One
                }
            }
            _ => HandKind::High,
        }
    }

    fn kind(hand: &str) -> HandKind {
        let mut cards = Self::get_card_occurrences(hand);
        Self::kind_from_occurrences(&cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let variant = self.variant || other.variant;

        if self.kind > other.kind {
            return Ordering::Greater;
        }
        if self.kind < other.kind {
            return Ordering::Less;
        }

        for i in 0..self.cards.len() {
            let selfval = card_value(self.cards[i], variant);
            let otherval = card_value(other.cards[i], variant);
            if selfval > otherval {
                return Ordering::Greater;
            }
            if selfval < otherval {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

fn card_value(card: char, variant: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if variant {
                1
            } else {
                11
            }
        }
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input: Vec<(Hand, usize)> = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(h, b)| (Hand::new(h, false), b.parse::<usize>().unwrap()))
        .collect();
    input.sort_by(|a, b| a.0.cmp(&b.0));

    Some(
        enumerate(input)
            .map(|(i, (_, b))| (i + 1) * b)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input: Vec<(Hand, usize)> = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(h, b)| (Hand::new(h, true), b.parse::<usize>().unwrap()))
        .collect();
    input.sort_by(|a, b| a.0.cmp(&b.0));

    Some(
        enumerate(input)
            .map(|(i, (_, b))| (i + 1) * b)
            .sum::<usize>() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_kind() {
        assert_eq!(Hand::new("AAAAA", false).kind, HandKind::Five);
        assert_eq!(Hand::new("AA8AA", false).kind, HandKind::Four);
        assert_eq!(Hand::new("23332", false).kind, HandKind::FullHouse);
        assert_eq!(Hand::new("T55J5", false).kind, HandKind::Three);
        assert_eq!(Hand::new("QQQJA", false).kind, HandKind::Three);
        assert_eq!(Hand::new("KK677", false).kind, HandKind::Two);
        assert_eq!(Hand::new("KTJJT", false).kind, HandKind::Two);
        assert_eq!(Hand::new("32T3K", false).kind, HandKind::One);
        assert_eq!(Hand::new("23456", false).kind, HandKind::High);
    }

    #[test]
    fn test_part_one_example() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(6440));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("inputs", 7);
        assert_eq!(part_one(&input), Some(256448566));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(5905));
    }
}
