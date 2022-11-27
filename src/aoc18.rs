pub const DATA: &str = "input/aoc18";
pub use crate::loaders::file_to as load;
use itertools::Itertools;
use std::{fmt::Debug, iter::Sum, mem, ops::Add, str::FromStr};
use Number::*;

#[derive(Clone, PartialEq)]
pub enum Number {
    Regular(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Default for Number {
    fn default() -> Self {
        Regular(0)
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut out = Pair(self.into(), rhs.into());
        out.reduce();
        out
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

fn extract_pair(s: &str) -> (&str, &str) {
    let mut level = 1;
    let delim = s[1..]
        .find(|c| match c {
            '[' => {
                level += 1;
                false
            }
            ']' => {
                level -= 1;
                false
            }
            ',' => level == 1, // Outermost pair's delimiter
            _ => false,
        })
        .unwrap()
        + 1;

    let end = s[delim + 1..]
        .find(|c| match c {
            '[' => {
                level += 1;
                false
            }
            ']' => {
                level -= 1;
                level == 0
            }
            _ => false,
        })
        .unwrap()
        + delim
        + 1;

    (&s[1..delim], &s[delim + 1..end])
}

impl FromStr for Number {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('[') => {
                let (left, right) = extract_pair(s);
                Ok(Pair(
                    left.parse::<Self>()?.into(),
                    right.parse::<Self>()?.into(),
                ))
            }
            Some(n) => Ok(Regular(n.to_digit(10).unwrap())), // n is < 10 due to reduction
            None => panic!("Cannot parse empty string"),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular(arg0) => write!(f, "{:?}", arg0),
            Self::Pair(arg0, arg1) => write!(f, "[{:?}, {:?}]", arg0, arg1),
        }
    }
}

impl Number {
    pub fn unwrap_regular(self) -> u32 {
        match self {
            Regular(n) => n,
            _ => panic!("Value is not regular"),
        }
    }

    pub fn unwrap_pair(self) -> (Number, Number) {
        match self {
            Pair(a, b) => (*a, *b),
            _ => panic!("Value is not a pair"),
        }
    }

    fn leftmost_regular(&mut self) -> Option<&mut Self> {
        match self {
            Regular(_) => Some(self),
            Pair(left, right) => left.leftmost_regular().or_else(|| right.leftmost_regular()),
        }
    }

    fn rightmost_regular(&mut self) -> Option<&mut Self> {
        match self {
            Regular(_) => Some(self),
            Pair(left, right) => right
                .rightmost_regular()
                .or_else(|| left.rightmost_regular()),
        }
    }

    pub fn explode_pair(&mut self) -> bool {
        self.explode_pair_inner(0, None, None)
    }

    fn explode_pair_inner(
        &mut self,
        level: u32,
        left_of_pair: Option<&mut Self>,
        right_of_pair: Option<&mut Self>,
    ) -> bool {
        if matches!(self, Regular(_)) {
            return false;
        }
        if level < 4 {
            if let Pair(left, right) = self {
                return left.explode_pair_inner(level + 1, left_of_pair, Some(right))
                    || right.explode_pair_inner(level + 1, Some(left), right_of_pair);
            }
            unreachable!()
        }

        let pair = mem::take(self);

        let (left, right) = pair.unwrap_pair();
        let closest_regular_to_the_right = left_of_pair.and_then(Self::rightmost_regular);
        if let Some(Regular(regular_to_the_right)) = closest_regular_to_the_right {
            *regular_to_the_right += left.unwrap_regular();
        }

        let closest_regular_to_the_left = right_of_pair.and_then(Self::leftmost_regular);
        if let Some(Regular(regular_to_the_left)) = closest_regular_to_the_left {
            *regular_to_the_left += right.unwrap_regular();
        }

        true
    }

    pub fn split_num(&mut self) -> bool {
        match self {
            Pair(a, b) => a.split_num() || b.split_num(),
            Regular(n) if *n < 10 => false,
            Regular(n) => {
                *self = Pair(Regular(*n / 2).into(), Regular((*n + 1) / 2).into()); // (floor, ceil)
                true
            }
        }
    }

    pub fn reduce(&mut self) {
        while self.explode_pair() || self.split_num() {}
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Regular(n) => *n,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

pub fn answer1(numbers: impl Iterator<Item = Number>) -> u32 {
    numbers.sum::<Number>().magnitude()
}

pub fn answer2(numbers: impl Iterator<Item = Number>) -> u32 {
    numbers
        .permutations(2) // All ordered pairs
        .map(|nums| answer1(nums.into_iter()))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA: &str = concat!(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]\n",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n",
        "[[[[5,4],[7,7]],8],[[8,3],8]]\n",
        "[[9,3],[[9,9],[6,[4,9]]]]\n",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n"
    );

    fn parse_mock_data(data: &str) -> impl Iterator<Item = Number> + '_ {
        data.lines().map(|line| line.parse().unwrap())
    }

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(parse_mock_data(MOCK_DATA)), 4140)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(parse_mock_data(MOCK_DATA)), 3993)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 3725)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 4832)
    }

    #[test]
    fn test_explode_once() {
        const EXAMPLES: [[&str; 2]; 5] = [
            // (the 9 has no regular number to its left, so it is not added to any regular number)
            ["[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"],
            // (the 2 has no regular number to its right, and so it is not added to any regular number)
            ["[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"],
            ["[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"],
            // (the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action)
            [
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ],
            [
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ],
        ];

        for [data, ans] in EXAMPLES {
            let mut data = data.parse::<Number>().unwrap();
            assert!(data.explode_pair()); // An explosion should give a "true"
            assert_eq!(data, ans.parse().unwrap());
        }
    }

    #[test]
    fn test_sum() {
        const EXAMPLES: [[&str; 2]; 5] = [
            [
                "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]",
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ],
            [
                "[1,1]\n[2,2]\n[3,3]\n[4,4]",
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ],
            [
                "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]",
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ],
            [
                "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ],
            [
                concat!(
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n",
                    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n",
                    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n",
                    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n",
                    "[7,[5,[[3,8],[1,4]]]]\n",
                    "[[2,[2,2]],[8,[8,1]]]\n",
                    "[2,9]\n",
                    "[1,[[[9,3],9],[[9,0],[0,7]]]]\n",
                    "[[[5,[7,4]],7],1]\n",
                    "[[[[4,2],2],6],[8,7]]\n"
                ),
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ],
        ];

        for [data, ans] in EXAMPLES {
            assert_eq!(parse_mock_data(data).sum::<Number>(), ans.parse().unwrap());
        }
    }

    #[test]
    fn test_magnitude() {
        const EXAMPLES: [(&str, u32); 6] = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (data, ans) in EXAMPLES {
            assert_eq!(data.parse::<Number>().unwrap().magnitude(), ans);
        }
    }
}
