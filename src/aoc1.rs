pub use crate::loaders::file_to_vec as load;

pub const DATA: &str = "input/aoc1";

pub fn answer1(input: &[u32]) -> usize {
    input.windows(2).filter(|&x| x[0] < x[1]).count()
}

pub fn answer2(input: &[u32]) -> usize {
    answer1(&input.windows(3).map(|x| x.iter().sum()).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: &[u32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(MOCK_DATA), 7)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(MOCK_DATA), 5)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load(DATA)), 1624)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load(DATA)), 1653)
    }
}
