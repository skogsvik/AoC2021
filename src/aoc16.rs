use crate::loaders::file_to_string;
use bits::Bits;

pub const DATA: &str = "input/aoc16";

pub fn load(data: impl AsRef<std::path::Path>) -> Bits {
    file_to_string(data).trim().parse().unwrap()
}
mod bits {
    use bitvec::prelude::*;
    use funty::Integral;
    use itertools::{Either, Itertools};
    use std::iter;

    pub struct Bits {
        data: BitVec<u8, Msb0>, // Use bytes since input data is in full bytes, use Msb0 since it makes loading the BitVec easier
        idx: usize,
    }

    impl std::str::FromStr for Bits {
        type Err = std::num::ParseIntError;

        fn from_str(data: &str) -> Result<Self, Self::Err> {
            let n = data.len();
            let data = (0..n)
                .step_by(2) // 2 hex make a byte
                .map(|i| u8::from_str_radix(&data[i..i + 2], 16))
                .try_collect()?;
            Ok(Self { data, idx: 0 })
        }
    }

    impl Bits {
        /// Move the index forward by `n` places
        pub fn skip(&mut self, n: usize) {
            self.idx += n;
        }

        /// Get the next bit as a `bool`
        pub fn pop(&mut self) -> bool {
            let b = self.data[self.idx];
            self.skip(1);
            b
        }

        /// Get the next len bits as an integer
        pub fn pop_integral<I: Integral>(&mut self, len: usize) -> I {
            let int = self.data[self.idx..self.idx + len].load_be();
            self.skip(len);
            int
        }

        /// Get the length of the literal packet which is currently at the pointer. No checks are made.
        pub fn get_literal_len(&self) -> usize {
            (self.idx..)
                .step_by(5) // Each block in the literal packet is 5 bits long
                .take_while(|&idx| self.data[idx])
                .count()
                * 5
                + 5
        }

        /// Pop of the next literal from memory starting from the current index
        pub fn pop_literal<I>(&mut self) -> I
        where
            I: Integral,
        {
            let len = self.get_literal_len();
            let int = self.data[self.idx..self.idx + len]
                .chunks(5)
                .fold(I::default(), |val, new| {
                    (val << 4) | new[1..].load_be::<I>()
                });
            self.skip(len);
            int
        }

        /// Iterate over the content of the operator by repeatably applying the closure to `self` a
        /// limited number of times. The limit is decided by the contents of the operator packet
        pub fn iter_operator<'a, F, I>(
            &'a mut self,
            fun: F,
        ) -> Either<impl Iterator<Item = I> + 'a, impl Iterator<Item = I> + 'a>
        where
            F: Fn(&mut Bits) -> I + 'a,
            I: Integral,
        {
            if self.pop() {
                // len id is 1
                let n = self.pop_integral(11);
                Either::Left(iter::repeat_with(move || fun(self)).take(n))
            } else {
                // len id is 0
                let bits_to_consume: usize = self.pop_integral(15);
                let target_idx = self.idx + bits_to_consume;
                Either::Right((0..).map_while(move |_| {
                    if self.idx < target_idx {
                        Some(fun(self))
                    } else {
                        None
                    }
                }))
            }
        }
    }
}

fn operate(op_code: u32, mut iter: impl Iterator<Item = u64>) -> u64 {
    match op_code {
        0 => iter.sum(),
        1 => iter.product(),
        2 => iter.min().unwrap(),
        3 => iter.max().unwrap(),
        5 => (iter.next().unwrap() > iter.next().unwrap()) as u64,
        6 => (iter.next().unwrap() < iter.next().unwrap()) as u64,
        7 => (iter.next().unwrap() == iter.next().unwrap()) as u64,
        _ => unreachable!(),
    }
}

fn extract_versions(bits: &mut Bits) -> u32 {
    let version = bits.pop_integral(3);
    let packet_type: u32 = bits.pop_integral(3);
    if packet_type == 4 {
        // Literal, skip past it since we don't care about the content yet
        bits.skip(bits.get_literal_len());
        version
    } else {
        // Operator, apply function recursively
        version + bits.iter_operator(extract_versions).sum::<u32>()
    }
}

fn calculate_operation(bits: &mut Bits) -> u64 {
    bits.skip(3); // Skip version
    let packet_type: u32 = bits.pop_integral(3);
    if packet_type == 4 {
        // Literal
        bits.pop_literal()
    } else {
        // Operator
        operate(packet_type, bits.iter_operator(calculate_operation))
    }
}

pub fn answer1(mut input: Bits) -> u32 {
    extract_versions(&mut input)
}

pub fn answer2(mut input: Bits) -> u64 {
    calculate_operation(&mut input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_DATA_AND_ANSWERS_1: [(&str, u32); 4] = [
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ];

    const MOCK_DATA_AND_ANSWERS_2: [(&str, u64); 8] = [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];

    #[test]
    fn test_answer1_mock_data() {
        for (mock_data, ans) in MOCK_DATA_AND_ANSWERS_1 {
            assert_eq!(answer1(mock_data.parse().unwrap()), ans);
        }
    }

    #[test]
    fn test_answer2_mock_data() {
        for (mock_data, ans) in MOCK_DATA_AND_ANSWERS_2 {
            assert_eq!(answer2(mock_data.parse().unwrap()), ans);
        }
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 1007)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 834151779165)
    }
}
