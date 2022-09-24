use std::collections::HashSet;

use itertools::Itertools;

use crate::loaders::file_to_lines;
pub const DATA: &str = "input/aoc13";

pub type Point = [u32; 2];
#[cfg_attr(test, derive(Clone))]
pub struct Instruction {
    axis: usize,
    index: u32,
}
type Points = HashSet<Point>;
type Instructions = Vec<Instruction>;

pub fn load(filename: impl AsRef<std::path::Path>) -> (Points, Instructions) {
    let mut lines = file_to_lines(filename);
    let points = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect();

    let instructions = lines
        .map(|line| match line.split_once('=') {
            Some(("fold along x", i)) => Instruction {
                axis: 0,
                index: i.parse().unwrap(),
            },
            Some(("fold along y", i)) => Instruction {
                axis: 1,
                index: i.parse().unwrap(),
            },
            _ => panic!("unexpected line!\n{}", line),
        })
        .collect();
    (points, instructions)
}

fn fold(points: &mut Points, instruction: &Instruction) {
    // Drain all values, mirroring those beyond the fold and put back into the set
    // TODO: This could be a really good use of drain_filtered once released
    // https://github.com/rust-lang/rust/issues/59618
    let folded = points
        .drain()
        .update(|point| {
            if instruction.index < point[instruction.axis] {
                // Mirror point
                point[instruction.axis] = 2 * instruction.index - point[instruction.axis];
            }
        })
        .collect_vec();
    points.extend(folded);
}

fn points_to_string(points: &Points) -> String {
    let mut points = points.iter().collect_vec();
    points.sort_unstable_by(|[x1, y1], [x2, y2]| y1.cmp(y2).then_with(|| x1.cmp(x2)));

    let mut output = String::with_capacity(points.len().pow(2));
    output.push('\n');
    let mut previuos = [0; 2];
    for current in points.into_iter() {
        for _ in 0..current[1] - previuos[1] {
            output.push('\n');
        }
        for _ in 0..current[0]
            .checked_sub(previuos[0] + 1)
            .unwrap_or(current[0])
        {
            output.push(' ');
        }
        output.push('#');
        previuos = *current;
    }
    output
}

pub fn answer1((mut points, instructions): (Points, Instructions)) -> usize {
    fold(&mut points, &instructions[0]);
    points.len()
}

pub fn answer2((mut points, instructions): (Points, Instructions)) -> String {
    for instruction in instructions {
        fold(&mut points, &instruction);
    }
    points_to_string(&points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_POINTS: [Point; 18] = [
        [6, 10],
        [0, 14],
        [9, 10],
        [0, 3],
        [10, 4],
        [4, 11],
        [6, 0],
        [6, 12],
        [4, 1],
        [0, 13],
        [10, 12],
        [3, 4],
        [3, 0],
        [8, 4],
        [1, 10],
        [2, 14],
        [8, 10],
        [9, 0],
    ];
    const MOCK_INSTRUCTIONS: [Instruction; 2] = [
        Instruction { axis: 1, index: 7 },
        Instruction { axis: 0, index: 5 },
    ];
    const MOCK_SOLUTION: &str = r"
#####
#   #
#   #
#   #
#####";
    const SOLUTION: &str = r"
#  #  ##  #    #### ###   ##  #### #  #
#  # #  # #       # #  # #  #    # #  #
#  # #    #      #  #  # #  #   #  #  #
#  # #    #     #   ###  ####  #   #  #
#  # #  # #    #    # #  #  # #    #  #
 ##   ##  #### #### #  # #  # ####  ##";

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1((HashSet::from_iter(MOCK_POINTS), MOCK_INSTRUCTIONS.to_vec())), 17)
    }
    
    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2((HashSet::from_iter(MOCK_POINTS), MOCK_INSTRUCTIONS.to_vec())), MOCK_SOLUTION)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 693)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), SOLUTION)
    }
}
