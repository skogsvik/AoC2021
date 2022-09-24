pub use crate::loaders::file_to as load;
use itertools::{EitherOrBoth, Itertools};
use std::{collections::HashMap, error, str};

pub const DATA: &str = "input/aoc5";

#[derive(Debug, Clone)]
pub struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl str::FromStr for Vent {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<i32> = s
            .split(" -> ")
            .flat_map(|point| point.split(','))
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()?;
        if dims.len() == 4 {
            Ok(Vent {
                x1: dims[0],
                y1: dims[1],
                x2: dims[2],
                y2: dims[3],
            })
        } else {
            Err("Unexpected number of dimensions found".into())
        }
    }
}

pub fn answer1(input: impl Iterator<Item = Vent>) -> usize {
    answer(input, true)
}
pub fn answer2(input: impl Iterator<Item = Vent>) -> usize {
    answer(input, false)
}

fn answer(input: impl Iterator<Item = Vent>, ignore_diagonal: bool) -> usize {
    let mut grid = HashMap::<(i32, i32), i32>::new();

    for vent in input {
        if ignore_diagonal && vent.x1 != vent.x2 && vent.y1 != vent.y2 {
            continue;
        }
        let x_range: Vec<_> = if vent.x1 < vent.x2 {
            (vent.x1..=vent.x2).collect()
        } else {
            (vent.x2..=vent.x1).rev().collect()
        };
        let y_range: Vec<_> = if vent.y1 < vent.y2 {
            (vent.y1..=vent.y2).collect()
        } else {
            (vent.y2..=vent.y1).rev().collect()
        };
        for coord in x_range.into_iter().zip_longest(y_range) {
            let coord = match coord {
                EitherOrBoth::Both(x, y) => (x, y),
                EitherOrBoth::Left(x) => (x, vent.y1),
                EitherOrBoth::Right(y) => (vent.x1, y),
            };
            *grid.entry(coord).or_default() += 1;
        }
    }

    grid.values().filter(|&&n| n >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: &[Vent] = &[
        Vent {
            x1: 0,
            y1: 9,
            x2: 5,
            y2: 9,
        },
        Vent {
            x1: 8,
            y1: 0,
            x2: 0,
            y2: 8,
        },
        Vent {
            x1: 9,
            y1: 4,
            x2: 3,
            y2: 4,
        },
        Vent {
            x1: 2,
            y1: 2,
            x2: 2,
            y2: 1,
        },
        Vent {
            x1: 7,
            y1: 0,
            x2: 7,
            y2: 4,
        },
        Vent {
            x1: 6,
            y1: 4,
            x2: 2,
            y2: 0,
        },
        Vent {
            x1: 0,
            y1: 9,
            x2: 2,
            y2: 9,
        },
        Vent {
            x1: 3,
            y1: 4,
            x2: 1,
            y2: 4,
        },
        Vent {
            x1: 0,
            y1: 0,
            x2: 8,
            y2: 8,
        },
        Vent {
            x1: 5,
            y1: 5,
            x2: 8,
            y2: 2,
        },
    ];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(MOCK_DATA.iter().cloned()), 5)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(MOCK_DATA.iter().cloned()), 12)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 8622)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 22037)
    }
}
