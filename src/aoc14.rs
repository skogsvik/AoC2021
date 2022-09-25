use crate::loaders::file_to_lines;
use std::collections::HashMap;

use itertools::Itertools;

pub const DATA: &str = "input/aoc14";

type Polymer = Vec<u8>;
type PolymerPairsCounts = HashMap<u32, u64>; // character pair -> count
type RuleMap = HashMap<u32, [u32; 2]>; // character pair -> resulting character pairs

/// Push the 2 u8s into the lower half of a u32
const fn pack_bytes(b1: u8, b2: u8) -> u32 {
    (b1 as u32) << 8 | b2 as u32
}

pub fn load(filename: impl AsRef<std::path::Path>) -> (Polymer, RuleMap) {
    let mut lines = file_to_lines(filename);
    let polymer = lines.next().unwrap().into_bytes(); // Polymer is first line ...

    let rules = lines
        .skip(1) // ... and then a newline ...
        // ... and finally the rules
        .map(|line| {
            let (pair, new_byte) = line.split_once(" -> ").unwrap();
            let [p1, p2]: [u8; 2] = pair.as_bytes().try_into().unwrap();
            let new_byte = new_byte.bytes().next().unwrap();
            (
                pack_bytes(p1, p2),
                [pack_bytes(p1, new_byte), pack_bytes(new_byte, p2)],
            )
        })
        .collect();

    (polymer, rules)
}

/// Single iteration of growing the polymer into a new one
fn grow(pairs: PolymerPairsCounts, rules: &RuleMap) -> PolymerPairsCounts {
    pairs
        .into_iter()
        // Each polymer pair becomes 2 new pairs and occur as many times as the original pair
        .flat_map(|(pair, count)| rules[&pair].iter().map(move |new_pair| (*new_pair, count)))
        // Don't use counts since it yields a usize, we need larger numbers than that
        .into_grouping_map()
        .sum()
}

fn answer((polymer, rules): (Polymer, RuleMap), n: u32) -> u64 {
    // Get polymer pairs and counts from original polymer
    let mut polymer_pairs = HashMap::with_capacity(polymer.len());
    for pair in polymer.windows(2) {
        *polymer_pairs
            .entry(pack_bytes(pair[0], pair[1]))
            .or_default() += 1;
    }

    // Grow polymer for n cycles
    for _ in 0..n {
        polymer_pairs = grow(polymer_pairs, &rules);
    }

    // Count the second half of each pair...
    let mut counts = polymer_pairs
        .into_iter()
        .map(|(pair, count)| (pair as u8, count))
        .into_grouping_map()
        .sum();
    // ... and make sure to count first symbol also which is otherwise missed
    *counts.entry(polymer[0]).or_default() += 1;

    use itertools::MinMaxResult::*;
    match counts.into_values().minmax() {
        MinMax(min, max) => max - min,
        NoElements => panic!("Polymer is empty"),
        OneElement(n) => panic!("All characters exist exactly {} time(s)", n),
    }
}

pub fn answer1(input: (Polymer, RuleMap)) -> u64 {
    answer(input, 10)
}

pub fn answer2(input: (Polymer, RuleMap)) -> u64 {
    answer(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_POLYMER: [u8; 4] = [b'N', b'N', b'C', b'B'];
    const MOCK_RULES: [(u32, [u32; 2]); 16] = [
        (
            pack_bytes(b'C', b'H'),
            [pack_bytes(b'C', b'B'), pack_bytes(b'B', b'H')],
        ),
        (
            pack_bytes(b'H', b'H'),
            [pack_bytes(b'H', b'N'), pack_bytes(b'N', b'H')],
        ),
        (
            pack_bytes(b'C', b'B'),
            [pack_bytes(b'C', b'H'), pack_bytes(b'H', b'B')],
        ),
        (
            pack_bytes(b'N', b'H'),
            [pack_bytes(b'N', b'C'), pack_bytes(b'C', b'H')],
        ),
        (
            pack_bytes(b'H', b'B'),
            [pack_bytes(b'H', b'C'), pack_bytes(b'C', b'B')],
        ),
        (
            pack_bytes(b'H', b'C'),
            [pack_bytes(b'H', b'B'), pack_bytes(b'B', b'C')],
        ),
        (
            pack_bytes(b'H', b'N'),
            [pack_bytes(b'H', b'C'), pack_bytes(b'C', b'N')],
        ),
        (
            pack_bytes(b'N', b'N'),
            [pack_bytes(b'N', b'C'), pack_bytes(b'C', b'N')],
        ),
        (
            pack_bytes(b'B', b'H'),
            [pack_bytes(b'B', b'H'), pack_bytes(b'H', b'H')],
        ),
        (
            pack_bytes(b'N', b'C'),
            [pack_bytes(b'N', b'B'), pack_bytes(b'B', b'C')],
        ),
        (
            pack_bytes(b'N', b'B'),
            [pack_bytes(b'N', b'B'), pack_bytes(b'B', b'B')],
        ),
        (
            pack_bytes(b'B', b'N'),
            [pack_bytes(b'B', b'B'), pack_bytes(b'B', b'N')],
        ),
        (
            pack_bytes(b'B', b'B'),
            [pack_bytes(b'B', b'N'), pack_bytes(b'N', b'B')],
        ),
        (
            pack_bytes(b'B', b'C'),
            [pack_bytes(b'B', b'B'), pack_bytes(b'B', b'C')],
        ),
        (
            pack_bytes(b'C', b'C'),
            [pack_bytes(b'C', b'N'), pack_bytes(b'N', b'C')],
        ),
        (
            pack_bytes(b'C', b'N'),
            [pack_bytes(b'C', b'C'), pack_bytes(b'C', b'N')],
        ),
    ];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1((MOCK_POLYMER.to_vec(), MOCK_RULES.into())), 1588)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(
            answer2((MOCK_POLYMER.to_vec(), MOCK_RULES.into())),
            2188189693529
        )
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 2194)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 2360298895777)
    }
}
