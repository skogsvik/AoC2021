pub const DATA: &str = "input/placeholder";

pub fn load(data: &str) -> &str {
    data
}

pub fn answer1(input: &str) -> &str {
    input
}

pub fn answer2(input: &str) -> &str {
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: &str = DATA;

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
