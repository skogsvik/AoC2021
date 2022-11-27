pub use crate::loaders::file_to_lines as load;
use std::{cmp, str};

pub const DATA: &str = "input/aoc3";

pub fn answer1(input: impl Iterator<Item = String>) -> i32 {
    let mut input = input.peekable();
    let mut len = 0;
    // Count occurences of '1' for each position
    let mut counts = vec![0; input.peek().unwrap().len()];
    for line in input {
        line.chars()
            .zip(counts.iter_mut())
            .for_each(|(bit, count)| match bit {
                '1' => *count += 1,
                '0' => (),
                _ => panic!("Unexpected bit"),
            });
        len += 1;
    }

    // Build binary number
    let majority_count = len / 2;
    let gamma = counts.iter().fold(0, |num, bit| {
        if *bit > majority_count {
            (num << 1) | 1 // Add a 1
        } else {
            num << 1 // Add a 0
        }
    });

    // Calculate epsilon as compliment of gamma
    let mask = (1 << counts.len()) - 1;
    let epsilon = gamma ^ mask;

    gamma * epsilon
}

fn filter_value(mut all_nums: Vec<i32>, cmp: cmp::Ordering, start_bit: u32) -> i32 {
    for bit_pos in (0..=start_bit).rev() {
        if all_nums.len() <= 1 {
            // Early exit
            break;
        }
        let n_ones = all_nums
            .iter()
            .filter(|&&s| (s >> bit_pos) & 0b1 == 0b1)
            .count();
        let target_bit = match (2 * n_ones).cmp(&all_nums.len()) {
            cmp::Ordering::Equal => match cmp {
                // Tiebreaker
                cmp::Ordering::Less => 0b0,
                cmp::Ordering::Greater => 0b1,
                cmp::Ordering::Equal => panic!("Invalid choice of comparison"),
            },
            x if x == cmp => 0b1, // ones fit filter
            _ => 0b0,             // zeros fit filter
        };
        all_nums.retain(|&s| (s >> bit_pos) & 0b1 != target_bit);
    }
    if all_nums.len() != 1 {
        panic!("Not exactly one number left after filter");
    }
    all_nums.pop().unwrap() // Only one item left
}

pub fn answer2(input: impl Iterator<Item = String>) -> i32 {
    let input: Vec<i32> = input
        .map(|line| i32::from_str_radix(&line, 2).unwrap())
        .collect();
    let bits_to_ignore = input.iter().map(|i| i.leading_zeros()).min().unwrap();
    let start_bit = i32::BITS - 1 - bits_to_ignore;
    filter_value(input.to_vec(), cmp::Ordering::Greater, start_bit)
        * filter_value(input.to_vec(), cmp::Ordering::Less, start_bit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mockers::*;

    const MOCK_DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(str_lines_to_string(MOCK_DATA)), 198)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(str_lines_to_string(MOCK_DATA)), 230)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 4174964)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 4474944)
    }
}
