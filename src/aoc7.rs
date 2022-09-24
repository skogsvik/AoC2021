use crate::loaders::delimited_file_to;
use std::{fmt::Debug, path::Path, str::FromStr};

pub const DATA: &str = "input/aoc7";

pub fn load<T>(filename: impl AsRef<Path>) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    delimited_file_to(filename, b',').collect::<Vec<T>>()
}

/// Since the problem is to find `min(sum(|pos-crab| over crab) over pos)`, which is the same as to
/// minize the mean absolute error of the position `pos`. It is known that the median over `crab` is
/// the optimal solution to this problem. We simply need to calculate the median and then the mean
/// absolute error. For even length results, the lower of the 2 center results is the correct one.
/// See also https://en.wikipedia.org/wiki/Mean_absolute_error#Optimality_property (2022-05-07)
pub fn answer1(input: &mut [i32]) -> i32 {
    let best_pos = *input.select_nth_unstable((input.len() - 1) / 2).1; // [lower] median
    input.iter().map(|crab| (best_pos - crab).abs()).sum()
}

/// Since the problem is to find `min(sum(sum(1..|pos-crab|) over crab) over pos)` or
/// `min(sum(|pos-crab|^2+|pos-crab| over crab) over pos)/2`.
/// The mean over `crab` minimizes `sum(|pos-crab|^2 over crab)` which gives a very good
/// approximation to the answer. So good that the true answer lays inside the open range of ± 0.5.
///
pub fn answer2(input: &[i32]) -> i32 {
    let mean = input.iter().sum::<i32>() as f32 / (input.len() as f32);
    [mean.floor() as i32, mean.ceil() as i32] // Check both due to offset from |pos-crab| term
        .into_iter()
        .map(|pos| {
            input
                .iter()
                .map(|crab| {
                    let diff = (pos - crab).abs();
                    (diff * (diff + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(&mut MOCK_DATA.clone()), 37)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(&MOCK_DATA), 168)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&mut load(DATA)), 336040)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load(DATA)), 94813675)
    }
}
