use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Block {
    Free(usize),
    File(u64, usize),
}

impl Block {
    pub fn is_free(&self) -> bool {
        match self {
            Block::Free(_) => true,
            Block::File(_, _) => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Block::Free(_) => false,
            Block::File(_, _) => true,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Block::Free(size) => *size,
            Block::File(_, size) => *size,
        }
    }
}

fn part_one(input: &str) -> Option<u64> {
    let mut disk = expand_part_one(input);
    defrag_part_one(&mut disk);
    Some(checksum(&disk))
}

fn part_two(input: &str) -> Option<u64> {
    let mut disk = expand_part_two(input);
    defrag_part_two(&mut disk);
    Some(checksum(&disk))
}

fn main() {
    let input = fs::read_to_string("inputs/09.txt").unwrap();
    advent_of_code::solve(1, &input, part_one);
    advent_of_code::solve(2, &input, part_two);
}

fn expand_part_one(input: &str) -> Vec<Block> {
    let chars = input.as_bytes();
    let mut out: Vec<Block> = vec![];
    let mut id: u64 = 0;
    let mut is_file = true;
    for c in chars {
        let chunk_size = (c - b'0') as usize;
        for _ in 0..chunk_size {
            out.push(if is_file {
                Block::File(id, 1)
            } else {
                Block::Free(1)
            });
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    out
}

fn expand_part_two(input: &str) -> Vec<Block> {
    let chars = input.as_bytes();
    let mut out: Vec<Block> = vec![];
    let mut id: u64 = 0;
    let mut is_file = true;
    for c in chars {
        let chunk_size = (c - b'0') as usize;
        out.push(if is_file {
            Block::File(id, chunk_size)
        } else {
            Block::Free(chunk_size)
        });
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    out
}

fn checksum(input: &Vec<Block>) -> u64 {
    let mut sum: u64 = 0;
    let mut idx = 0;
    for b in input {
        match b {
            Block::Free(_) => {}
            Block::File(id, size) => (idx..(idx + size)).for_each(|i| sum += i as u64 * id),
        }
        idx += b.size();
    }
    sum
}

fn defrag_part_one(input: &mut Vec<Block>) {
    let mut left = 0;
    let mut right = input.len() - 1;

    while right > left {
        while input[left].is_file() && left < input.len() {
            left += 1;
        }

        while input[right].is_free() {
            right -= 1;
        }

        if input[left].is_free() && left < right {
            // Swap
            input.swap(left, right);
        }
    }
}

fn defrag_part_two(input: &mut Vec<Block>) {
    let mut right = input.len() - 1;

    while right > 1 {
        while input[right].is_free() && right > 0 {
            right -= 1;
        }

        let mut left = 0;
        while left < right && (input[left].is_file() || input[left].size() < input[right].size()) {
            left += 1;
        }

        if left < right && input[left].is_free() && input[left].size() >= input[right].size() {
            let free_block = input[left];
            let file_block = input[right];

            // Swap file with free block of same size
            input[left] = file_block;
            input[right] = Block::Free(file_block.size());

            // Merge free block with free block on the left if possible
            if input[right - 1].is_free() {
                input[right - 1] = Block::Free(input[right - 1].size() + input[right].size());
                input.remove(right);

                // Merge resulting free block with free block on the right if possible
                if right < input.len() && input[right].is_free() {
                    input[right - 1] = Block::Free(input[right - 1].size() + input[right].size());
                    input.remove(right);
                }
            }

            // Put remaining free space on right side of file if required
            if free_block.size() > file_block.size() {
                input.insert(left + 1, Block::Free(free_block.size() - file_block.size()));
            }
        }

        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn str_to_blocks(input: &str) -> Vec<Block> {
        input
            .chars()
            .map(|c| match c {
                '.' => Block::Free(1),
                n => Block::File(n as u64 - '0' as u64, 1),
            })
            .collect()
    }

    fn blocks_to_str(blocks: &Vec<Block>) -> String {
        blocks
            .iter()
            .map(|b| match b {
                Block::Free(size) => ".".repeat(*size),
                Block::File(id, size) => id.to_string().repeat(*size),
            })
            .collect()
    }

    #[rstest]
    #[case("", "")]
    #[case("0", "1")]
    #[case("00", "2")]
    #[case("000", "3")]
    #[case("0.", "11")]
    #[case("0.1", "111")]
    #[case("0..111....22222", "12345")]
    fn test_expansion(#[case] expected: &str, #[case] input: &str) {
        assert_eq!(expected, blocks_to_str(&expand_part_one(input)));
    }

    #[rstest]
    #[case("0", "0")]
    #[case("01.", "0.1")]
    #[case("01..", "0..1")]
    #[case("01..", "0.1.")]
    #[case("001..", "00.1.")]
    #[case("0312..", "0.1.23")]
    #[case(
        "0099811188827773336446555566..............",
        "00...111...2...333.44.5555.6666.777.888899"
    )]
    fn test_defrag_part_one(#[case] expected: &str, #[case] input: &str) {
        let mut input = str_to_blocks(input);
        defrag_part_one(&mut input);
        let actual = blocks_to_str(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_defrag_part_two() {
        let mut input = expand_part_two("2333133121414131402");
        let expected = "00992111777.44.333....5555.6666.....8888..";

        defrag_part_two(&mut input);
        assert_eq!(expected, blocks_to_str(&input));
    }

    #[rstest]
    #[case(0, "")]
    #[case(0, "0")]
    #[case(1, "01.")]
    #[case(2, "0.1.")]
    #[case(5, "012.")]
    #[case(11, "0312..")]
    fn test_checksum(#[case] expected: u64, #[case] input: &str) {
        let input = str_to_blocks(input);
        assert_eq!(expected, checksum(&input));
    }

    #[test]
    fn part_one_test() {
        let input = fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(Some(1928), part_one(&input));
    }

    #[test]
    fn part_two_test() {
        let input = fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(Some(2858), part_two(&input));
    }
}
