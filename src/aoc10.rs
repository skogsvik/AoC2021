pub use crate::loaders::file_to_lines as load;
use itertools::Itertools;
pub const DATA: &str = "input/aoc10";

fn byte_to_points(byte: &u8) -> u32 {
    match byte {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        bad => panic!("Unexpected char {}", bad),
    }
}

pub fn answer1(input: impl Iterator<Item = String>) -> u32 {
    input
        .map(|line| {
            let mut stack = Vec::with_capacity(line.len());
            for byte in line.as_bytes() {
                match byte {
                    b'{' | b'(' | b'[' | b'<' => stack.push(byte),
                    _ => {
                        if stack
                            .pop()
                            .map(|opener| byte.wrapping_sub(*opener) <= 2)
                            .unwrap()
                        {
                            continue; // Line is good so far, check next byte
                        } else {
                        }
                        return byte_to_points(byte); // Incorrect closer, yield points
                    }
                }
            }
            0 // Line is fine or corrupt
        })
        .sum()
}

pub fn answer2(input: impl Iterator<Item = String>) -> u64 {
    let mut scores = input
        .filter_map(|line| {
            let mut stack = Vec::with_capacity(line.len());
            for byte in line.as_bytes() {
                match byte {
                    b'{' | b'(' | b'[' | b'<' => stack.push(byte),
                    _ => {
                        if stack
                            .pop()
                            .map(|opener| byte.wrapping_sub(*opener) <= 2)
                            .unwrap()
                        {
                            continue; // Line is good so far, check next byte
                        } else {
                        }
                        return None; // Incorrect closer, ignore
                    }
                }
            }
            if stack.is_empty() {
                return None;
            }
            stack.reverse();
            Some(
                stack
                    .iter()
                    .map(|opener| match opener {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => panic!(),
                    })
                    .fold(0, |total, point| 5 * total + point),
            )
        })
        .collect_vec();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mockers::str_lines_to_string;

    const MOCK_DATA: &str = concat!(
        "[({(<(())[]>[[{[]{<()<>>\n",
        "[(()[<>])]({[<{<<[]>>(\n",
        "{([(<{}[<>[]}>{[]{[(<()>\n",
        "(((({<>}<{<{<>}{[]{[]{}\n",
        "[[<[([]))<([[{}[[()]]]\n",
        "[{[{({}]{}}([{[{{{}}([]\n",
        "{<[[]]>}<{[{[{[]{()[[[]\n",
        "[<(<(<(<{}))><([]([]()\n",
        "<{([([[(<>()){}]>(<<{{\n",
        "<{([{{}}[<[[[<>{}]]]>[]]\n",
    );

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(str_lines_to_string(MOCK_DATA)), 26397)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(str_lines_to_string(MOCK_DATA)), 288957)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 193275)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 2429644557)
    }
}
