use crate::loaders::file_to_lines;
use itertools::Itertools;
use std::collections::HashMap;
pub const DATA: &str = "input/aoc12";

pub type Cave = u32;
const START: u32 = 0;
const END: u32 = 1;
type CaveList = Vec<Cave>;
type CaveMap = HashMap<Cave, CaveList>;

/// Encodes a &str representation of a cave to a 32 bit number according to:
/// "start" => START (0)
/// "end" => END (1)
/// u32 of u8 representations packed next to eachother. e.g. `"a"` would be 97, and `"aa"` 24929
fn str_to_cave(s: &str) -> Cave {
    match s {
        "start" => START,
        "end" => END,
        _ => s.bytes().fold(0, |val, bit| val << 8 | bit as u32), // This might as well be << 7 since ASCII fits in 7 bits
    }
}

fn is_small_cave(&cave: &Cave) -> bool {
    /*
    We assume all caves are either start, end, or 2 letters. This allows to check the case with a
    simple comparison, making start and end considered large caves (the non-special case)
    */
    const SMALL_CAVE_AA: u32 = 24929; // str_to_cave("aa")
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
    mut path: CaveList,
    map: &CaveMap,
    has_used_double_visit: bool,
) -> Vec<CaveList> {
    path.push(current_cave);
    if current_cave == END {
        return vec![path];
    }
    map[&current_cave]
        .iter()
        .filter_map(|cave_candidate| {
            if is_small_cave(cave_candidate) {
                if !path.contains(cave_candidate) {
                    Some((cave_candidate, has_used_double_visit))
                } else if !has_used_double_visit {
                    Some((cave_candidate, true))
                } else {
                    None
                }
            } else {
                Some((cave_candidate, has_used_double_visit))
            }
        })
        .flat_map(|(&next, has_used_double_visit)| {
            traverse(next, path.clone(), map, has_used_double_visit)
        })
        .collect()
}

pub fn answer1(input: CaveMap) -> usize {
    traverse(START, vec![], &input, true).len()
}

pub fn answer2(input: CaveMap) -> usize {
    traverse(START, vec![], &input, false).len()
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
