pub use crate::loaders::file_to_squashed_2d_vec as load;
use itertools::{iproduct, Itertools};
use std::{cmp::Reverse, collections::BinaryHeap, iter::repeat};

pub const DATA: &str = "input/aoc15";

type Node = u32;
type Cave = Vec<Node>;
type Neighbours = Vec<Vec<usize>>;

/// Lookup table for neighbours, saves time at the cost of memory
fn bake_neighbour_lookup(n_rows: usize, n_cols: usize) -> Neighbours {
    let max_row = n_rows - 1;
    let max_col = n_cols - 1;
    iproduct!(0..n_rows, 0..n_cols)
        .map(|(row, col)| {
            let idx = n_cols * row + col;
            let mut out = Vec::with_capacity(4);
            if row > 0 {
                out.push(idx - n_cols);
            }
            if row < max_row {
                out.push(idx + n_cols);
            }
            if col > 0 {
                out.push(idx - 1);
            }
            if col < max_col {
                out.push(idx + 1);
            }
            out
        })
        .collect()
}

fn a_star(nodes: &Cave, width: usize, start: usize, goal: usize) -> u32 {
    let n_nodes = nodes.len();
    let height = n_nodes / width;
    let m_distance = iproduct!(0..height, 0..width)
        .map(|(r, c)| (height - r + width - c) as u32)
        .collect_vec();
    let neighbours = bake_neighbour_lookup(height, width);

    let mut g_score = repeat(u32::MAX).take(n_nodes).collect_vec();
    let mut closed = repeat(false).take(n_nodes).collect_vec();

    g_score[start] = 0;
    let mut open = BinaryHeap::from([(Reverse(m_distance[start]), start)]);
    loop {
        let (_, current) = open.pop().unwrap();

        if current == goal {
            return g_score[goal];
        }
        if closed[current] {
            continue;
        }

        for &nbor in &neighbours[current] {
            if closed[nbor] {
                continue;
            }
            let candidate_g_score = g_score[current] + nodes[nbor];
            if candidate_g_score < g_score[nbor] {
                g_score[nbor] = candidate_g_score;
                open.push((Reverse(candidate_g_score + m_distance[nbor]), nbor));
            }
        }
        closed[current] = true;
    }
}

fn extend(cave: Cave, width: usize, n: usize) -> Cave {
    let mut new_cave = Vec::with_capacity(cave.len() * n * n);
    // Copy to left
    for row in cave.chunks(width) {
        for _ in 0..n {
            new_cave.extend_from_slice(row);
        }
    }
    // Copy down
    for _ in 0..n - 1 {
        new_cave.extend_from_within(..n * cave.len());
    }
    // Increase
    for (i_row, row) in new_cave.chunks_mut(n * cave.len()).enumerate() {
        let i_row = i_row as u32;
        for (i_col, seg) in row.chunks_mut(width).enumerate() {
            let i_col = (i_col % n) as u32;
            for num in seg {
                *num = (*num + i_row + i_col - 1) % 9 + 1;
            }
        }
    }

    new_cave
}

pub fn answer1((cave, width): (Cave, usize)) -> u32 {
    a_star(&cave, width, 0, cave.len() - 1)
}

pub fn answer2((cave, mut width): (Cave, usize)) -> u32 {
    let cave = extend(cave, width, 5);
    width *= 5;
    a_star(&cave, width, 0, cave.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_WIDTH: usize = 10;
    #[rustfmt::skip]
    const MOCK_DATA: [Node; MOCK_WIDTH * MOCK_WIDTH] = [
        1,1,6,3,7,5,1,7,4,2,
        1,3,8,1,3,7,3,6,7,2,
        2,1,3,6,5,1,1,3,2,8,
        3,6,9,4,9,3,1,5,6,9,
        7,4,6,3,4,1,7,1,1,1,
        1,3,1,9,1,2,8,1,3,7,
        1,3,5,9,9,1,2,4,2,1,
        3,1,2,5,4,2,1,6,3,9,
        1,2,9,3,1,3,8,5,2,1,
        2,3,1,1,9,4,4,5,8,1,
    ];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1((MOCK_DATA.to_vec(), MOCK_WIDTH)), 40)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2((MOCK_DATA.to_vec(), MOCK_WIDTH)), 315)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 619)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 2922)
    }
}
