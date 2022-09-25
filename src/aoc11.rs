pub use crate::loaders::file_to_squashed_2d_vec as load;
use std::iter::repeat_with;

use itertools::{iproduct, Itertools};

pub const DATA: &str = "input/aoc11";

type Octopuses = Vec<u32>; // Vector of each octopus count
type Neighbours = Vec<Vec<usize>>; // Vector of vectors, listing each octopus neighbor

/// Lookup table for neighbours, saves time at the cost of memory
fn bake_neighbour_lookup(n_rows: usize, n_cols: usize) -> Neighbours {
    let row_ranges = (0..n_rows).map(|row| row.saturating_sub(1)..n_rows.min(row + 2));
    let column_ranges = (0..n_cols).map(|col| col.saturating_sub(1)..n_cols.min(col + 2));

    iproduct!(row_ranges, column_ranges)
        .enumerate()
        .map(|(idx, (rows, columns))| {
            iproduct![rows, columns]
                .map(|(row, col)| n_cols * row + col)
                .filter(|&i| i != idx) // current cell is not a neighbour
                .collect()
        })
        .collect()
}

fn flash(octopuses: &mut Octopuses, neighbours: &Neighbours) -> u32 {
    // First increment + detect which octopuses are ready to flash
    let mut will_flash = octopuses
        .iter_mut()
        .enumerate()
        .filter_map(|(idx, val)| {
            *val += 1;
            if *val == 10 {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec();

    // Flash all, including ripple effects
    let mut total_flashed = 0;
    while let Some(idx) = will_flash.pop() {
        total_flashed += 1;
        octopuses[idx] = 0;
        for &idx in &neighbours[idx] {
            let octopus = &mut octopuses[idx];
            // 0 means has flashed this session
            if *octopus != 0 {
                *octopus += 1;
                if *octopus == 10 {
                    will_flash.push(idx);
                }
            }
        }
    }
    total_flashed
}

pub fn answer1((mut octopuses, width): (Octopuses, usize)) -> u32 {
    let neighbours = bake_neighbour_lookup(octopuses.len() / width, width);
    repeat_with(|| flash(&mut octopuses, &neighbours))
        .take(100)
        .sum()
}

pub fn answer2((mut octopuses, width): (Octopuses, usize)) -> u32 {
    let neighbours = bake_neighbour_lookup(octopuses.len() / width, width);
    let mut iterations = 0;
    while !octopuses.iter().all_equal() {
        iterations += 1;
        flash(&mut octopuses, &neighbours);
    }
    iterations
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_WIDTH: usize = 10;
    #[rustfmt::skip]
    const MOCK_DATA: [u32; MOCK_WIDTH*MOCK_WIDTH] = [
        5, 4, 8, 3, 1, 4, 3, 2, 2, 3,
        2, 7, 4, 5, 8, 5, 4, 7, 1, 1,
        5, 2, 6, 4, 5, 5, 6, 1, 7, 3,
        6, 1, 4, 1, 3, 3, 6, 1, 4, 6,
        6, 3, 5, 7, 3, 8, 5, 4, 7, 8,
        4, 1, 6, 7, 5, 2, 4, 6, 4, 5,
        2, 1, 7, 6, 8, 4, 1, 7, 2, 1,
        6, 8, 8, 2, 8, 8, 1, 1, 3, 4,
        4, 8, 4, 6, 8, 4, 8, 5, 5, 4,
        5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
    ];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1((MOCK_DATA.to_vec(), MOCK_WIDTH)), 1656)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2((MOCK_DATA.to_vec(), MOCK_WIDTH)), 195)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 1601)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 368)
    }
}
