use crate::loaders::file_to_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
pub const DATA: &str = "input/aoc12";

pub type Cave = u32;
const START: Cave = 0;
const END: Cave = 1;
type CaveMap = HashMap<Cave, Vec<Cave>>;

/// Encodes a &str representation of a cave to a 32 bit number according to:
/// "start" => START (0)
/// "end" => END (1)
/// u32 of u8 representations packed next to eachother. e.g. `"a"` would be 97, and `"aa"` 97*(1 + 2**8) = 24929
fn str_to_cave(s: &str) -> Cave {
    match s {
        "start" => START,
        "end" => END,
        _ => s.bytes().fold(0, |val, bit| val << 8 | bit as Cave), // This might as well be << 7 since ASCII fits in 7 bits
    }
}

fn is_small_cave(&cave: &Cave) -> bool {
    /*
    We assume all caves are either start, end, or 2 letters. This allows to check the case with a
    simple comparison, making start and end considered large caves (the non-special case)
    */
    const SMALL_CAVE_AA: Cave = 24929; // str_to_cave("aa")
    cave >= SMALL_CAVE_AA
}

pub fn load(filename: &str) -> CaveMap {
    lines_to_cave_map(file_to_lines(filename))
}

fn lines_to_cave_map(lines: impl Iterator<Item = String>) -> CaveMap {
    lines
        .flat_map(|line| {
            let (from, to) = line.split('-').map(str_to_cave).next_tuple().unwrap();
            [(from, to), (to, from)]
        })
        .filter(|(from, to)| *from != END && *to != START)
        .into_group_map()
}

fn traverse(
    current_cave: Cave,
    mut path: HashSet<Cave>,
    map: &CaveMap,
    may_visit_small_cave_twice: bool,
) -> usize {
    path.insert(current_cave);
    map[&current_cave]
        .iter()
        .filter_map(|cave_candidate| {
            if is_small_cave(cave_candidate) {
                if !path.contains(cave_candidate) {
                    Some((cave_candidate, may_visit_small_cave_twice)) // New small cave
                } else if may_visit_small_cave_twice {
                    Some((cave_candidate, false)) // First re-visited small cave
                } else {
                    None // Additionally re-visited small cave
                }
            } else {
                Some((cave_candidate, may_visit_small_cave_twice)) // Big cave
            }
        })
        .map(|(&next_cave, may_visit_small_cave_twice)| {
            if next_cave == END {
                1
            } else {
                traverse(next_cave, path.clone(), map, may_visit_small_cave_twice)
            }
        })
        .sum()
}

pub fn answer1(input: CaveMap) -> usize {
    traverse(START, HashSet::with_capacity(input.len()), &input, false)
}

pub fn answer2(input: CaveMap) -> usize {
    traverse(START, HashSet::with_capacity(input.len()), &input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mockers::str_lines_to_string;
    use itertools::zip_eq;

    const MOCK_DATA: [&str; 2] = [
        r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
    ];

    #[test]
    fn test_answer1_mock_data() {
        for (data, result) in zip_eq(MOCK_DATA, [19, 226]) {
            assert_eq!(
                answer1(lines_to_cave_map(str_lines_to_string(data))),
                result
            )
        }
    }

    #[test]
    fn test_answer2_mock_data() {
        for (data, result) in zip_eq(MOCK_DATA, [103, 3509]) {
            assert_eq!(
                answer2(lines_to_cave_map(str_lines_to_string(data))),
                result
            )
        }
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 4720)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 147848)
    }
}
