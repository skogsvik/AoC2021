pub use crate::loaders::file_to_array2 as load;
use ndarray::Array2;
pub const DATA: &str = "input/aoc9";

type Map = Array2<u32>;

fn iter_low_points(floor: &Map) -> impl Iterator<Item = ((usize, usize), &u32)> {
    let (n_row, n_col) = floor.dim();
    floor.indexed_iter().filter(move |&((i_row, i_col), &pos)| {
        iter_neighbours(i_row, i_col, n_row - 1, n_col - 1).all(|idx| floor[idx] > pos)
    })
}

fn iter_neighbours(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut v = Vec::with_capacity(4);
    if row > 0 {
        v.push((row - 1, col));
    }
    if row < max_row {
        v.push((row + 1, col));
    }
    if col > 0 {
        v.push((row, col - 1));
    }
    if col < max_col {
        v.push((row, col + 1));
    }
    v.into_iter()
}

fn find_unclaimed(claim_mask: &Array2<bool>) -> Option<(usize, usize)> {
    claim_mask
        .indexed_iter()
        .find_map(|(idx, &val)| if !val { Some(idx) } else { None })
}

// TODO: use sobel gradient descent instead
pub fn answer1(input: &Map) -> u32 {
    iter_low_points(input).map(|(_, i)| i + 1).sum()
}

pub fn answer2(floor: &Map) -> usize {
    // Map to keep track of cells to not check, starts as all the edges
    let mut claimed = floor.map(|&height| height == 9);
    let (n_row, n_col) = floor.dim();
    let (max_row, max_col) = (n_row - 1, n_col - 1);
    let mut basins = Vec::new();

    while let Some(idx) = find_unclaimed(&claimed) {
        claimed[idx] = true;
        let mut basin = 1;
        let mut new_cells = vec![idx];
        while let Some((row, col)) = new_cells.pop() {
            new_cells.extend(iter_neighbours(row, col, max_row, max_col).filter(|&idx| {
                if claimed[idx] {
                    return false; // Only care for unclaimed cells
                }
                claimed[idx] = true;
                basin += 1;
                true
            }))
        }
        basins.push(basin);
    }
    basins.sort();
    basins[basins.len() - 3..].iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    const MOCK_DATA: [[u32; 10]; 5] = [
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ];

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(&arr2(&MOCK_DATA)), 15)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(&arr2(&MOCK_DATA)), 1134)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(&load(DATA)), 554)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(&load(DATA)), 1017792)
    }
}
