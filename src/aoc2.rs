pub use crate::loaders::file_to_lines as load;
pub const DATA: &str = "input/aoc2";

#[derive(Default)]
struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

fn parse_direction(input: String) -> (String, u32) {
    let (direction, value) = input.split_once(' ').expect("No space");
    let value = value.parse().expect("Int Parse Failed");
    (direction.to_string(), value)
}

impl Position {
    fn add_direction(&mut self, input: String) {
        let (direction, value) = parse_direction(input);
        match direction.as_ref() {
            "forward" => self.horizontal += value,
            "down" => self.depth += value,
            "up" => self.depth -= value,
            _ => panic!("{}", direction),
        }
    }
    fn add_direction_with_aim(&mut self, input: String) {
        let (direction, value) = parse_direction(input);
        match direction.as_ref() {
            "forward" => {
                self.horizontal += value;
                self.depth += self.aim * value;
            }
            "down" => self.aim += value,
            "up" => self.aim -= value,
            _ => panic!("{}", direction),
        }
    }
}

fn solve<F>(input: impl Iterator<Item = String>, method: F) -> u32
where
    F: Fn(&mut Position, String),
{
    let mut pos = Position::default();
    input.for_each(|dir| method(&mut pos, dir));
    pos.horizontal * pos.depth
}

pub fn answer1(input: impl Iterator<Item = String>) -> u32 {
    solve(input, Position::add_direction)
}

pub fn answer2(input: impl Iterator<Item = String>) -> u32 {
    solve(input, Position::add_direction_with_aim)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mockers::*;

    const MOCK_DATA: &str = concat!(
        "forward 5\n",
        "down 5\n",
        "forward 8\n",
        "up 3\n",
        "down 8\n",
        "forward 2\n",
    );

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(str_lines_to_string(MOCK_DATA)), 150)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(str_lines_to_string(MOCK_DATA)), 900)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 1524750)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 1592426537)
    }
}
