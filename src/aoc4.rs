pub use crate::loaders::file_to_lines as load;
use itertools::Itertools;

pub const DATA: &str = "input/aoc4";

type Grid<T, const LENGTH: usize> = [[T; LENGTH]; LENGTH];

#[derive(Default, Debug, Clone)]
pub struct Board {
    board: Grid<u8, 5>,
    mask: Grid<bool, 5>,
}

fn check_row<const L: usize>(mask: Grid<bool, L>, i_row: usize) -> bool {
    mask[i_row].iter().all(|&m| m)
}

fn check_column<const L: usize>(mask: Grid<bool, L>, i_column: usize) -> bool {
    mask.iter().all(|row| row[i_column])
}

impl Board {
    fn check_number(&mut self, number: u8) -> bool {
        // Assume only one occurence of each number per board
        self.board.iter().enumerate().any(|(ir, row)| {
            row.iter().enumerate().any(|(ic, col)| {
                if *col == number {
                    self.mask[ir][ic] = true;
                    check_row(self.mask, ir) || check_column(self.mask, ic)
                } else {
                    false
                }
            })
        })
    }

    fn iter_unmasked(&self) -> impl Iterator<Item = u8> + '_ {
        self.board
            .iter()
            .zip(self.mask)
            .flat_map(|(row, mask_row)| row.iter().zip(mask_row))
            .filter_map(|(board, mask)| if !mask { Some(*board) } else { None })
    }
}

pub fn parse_lines(mut lines: impl Iterator<Item = String>) -> (Vec<u8>, Vec<Board>) {
    let order = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            let mut board = Board::default();
            for (input_row, row) in chunk.skip(1).zip(board.board.iter_mut()) {
                for (input_col, col) in input_row.split_whitespace().zip(row.iter_mut()) {
                    *col = input_col.parse().unwrap();
                }
            }
            board
        })
        .collect();
    (order, boards)
}

pub fn answer1(input: impl Iterator<Item = String>) -> u32 {
    find_first_winner(parse_lines(input))
}

fn find_first_winner((order, mut boards): (Vec<u8>, Vec<Board>)) -> u32 {
    // TODO: change order type to a trait bound instead to avoid a collect in answer2
    for number in order {
        for board in boards.iter_mut() {
            if board.check_number(number) {
                return board.iter_unmasked().map_into::<u32>().sum::<u32>() * number as u32;
            }
        }
    }
    panic!("Game doesn't end, needs more numbers")
}

pub fn answer2(input: impl Iterator<Item = String>) -> u32 {
    let (order, mut boards) = parse_lines(input);
    let mut order = order.into_iter(); // Make a consumable iterator so that we can re-use it later
    for number in &mut order {
        boards = boards // could be replaced by retain_mut if using nightly
            .into_iter()
            .filter_map(|mut board| {
                if board.check_number(number) {
                    None
                } else {
                    Some(board)
                }
            })
            .collect();
        if boards.len() == 1 {
            return find_first_winner((order.collect(), boards));
        }
    }
    panic!("Not enough bingo numbers to declare a slowest board")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mockers::*;

    const MOCK_DATA: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(str_lines_to_string(MOCK_DATA)), 4512)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(str_lines_to_string(MOCK_DATA)), 1924)
    }
    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 31424)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 23042)
    }
}
