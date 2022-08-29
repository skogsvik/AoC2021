use crate::loaders::delimited_file_to;

pub const DATA: &str = "input/aoc6";
const REPRODUCTIVE_PERIOD: usize = 7; // Days between offspring
const PUBERTY_LENGTH: usize = 2; // Extra days before starting reproductive cycle

#[inline]
pub fn load(filename: impl AsRef<std::path::Path>) -> impl Iterator<Item = usize> {
    delimited_file_to(filename, b',')
}

fn answer(input: impl Iterator<Item = usize>, days: usize) -> i64 {
    const N: usize = REPRODUCTIVE_PERIOD + PUBERTY_LENGTH;
    let mut counter = [0; N];
    for i in input {
        counter[i] += 1;
    }
    for d in 0..days {
        counter[(d + REPRODUCTIVE_PERIOD) % N] += counter[d % N];
    }
    counter.iter().sum()
}

pub fn answer1(input: impl Iterator<Item = usize>) -> i64 {
    answer(input, 80)
}

pub fn answer2(input: impl Iterator<Item = usize>) -> i64 {
    answer(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: &[usize] = &[3, 4, 3, 1, 2];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(MOCK_DATA.iter().cloned()), 5934)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(MOCK_DATA.iter().cloned()), 26984457539)
    }
    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 361169)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 1634946868992)
    }
}
