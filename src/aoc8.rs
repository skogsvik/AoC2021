pub use crate::loaders::file_to_lines as load;
use itertools::Itertools;
use std::collections::HashSet;

pub const DATA: &str = "input/aoc8";

pub fn answer1(input: impl Iterator<Item = String>) -> usize {
    input
        .map(|line| {
            line.split_once('|')
                .unwrap()
                .1 // only consider output
                .split_whitespace()
                .map(str::len)
                .filter(|len| matches!(len, 2..=4 | 7)) // Count words with 2,3,4, or 7 characters
                .count()
        })
        .sum()
}

pub fn answer2(input: impl Iterator<Item = String>) -> usize {
    input
        .map(|line| {
            // Separate input and output
            let (input, output) = line.split_once('|').unwrap();

            // Create map of number of segments to vector of sets of segements
            let mut input = input
                .split_whitespace()
                .map(|digit| (digit.len(), digit.chars().collect::<HashSet<_>>()))
                .into_group_map();

            // Get the unique digits directly from length
            let one = input.remove(&2).unwrap().swap_remove(0);
            let four = input.remove(&4).unwrap().swap_remove(0);
            let seven = input.remove(&3).unwrap().swap_remove(0);
            let eight = input.remove(&7).unwrap().swap_remove(0);

            // Calculate six segment digits
            let six_len_digits = input.get_mut(&6).unwrap();
            let nine = remove_by(six_len_digits, |digit| four.is_subset(digit)).unwrap(); // 4 is a subset of 9, but not 0 and 6
            let zero = remove_by(six_len_digits, |digit| one.is_subset(digit)).unwrap(); // 1 is a subset 0, but not 6
            let six = six_len_digits.swap_remove(0); // Only 6 left

            // Calculate five segment digits
            let five_len_digits = input.get_mut(&5).unwrap();
            let three = remove_by(five_len_digits, |digit| one.is_subset(digit)).unwrap(); // 1 is a subset of 3, but not 2 and 5
            let five = remove_by(five_len_digits, |digit| nine.is_superset(digit)).unwrap(); // 9 is superset of 5, but not 2
            let two = five_len_digits.swap_remove(0); // Only 2 left

            let numbers = [zero, one, two, three, four, five, six, seven, eight, nine];

            // Calculate output number
            output
                .split_whitespace()
                .map(|digit| digit.chars().collect::<HashSet<_>>())
                .rev()
                .enumerate()
                .map(|(exp, digit)| {
                    10usize.pow(exp as u32) * numbers.iter().position(|x| x == &digit).unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

fn remove_by<P, I>(array: &mut Vec<I>, predicate: P) -> Result<I, &str>
where
    P: FnMut(&I) -> bool,
{
    let idx = array.iter().position(predicate).ok_or("No item found")?;
    Ok(array.swap_remove(idx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mockers::*;

    const MOCK_DATA: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |        fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |        fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |        cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |        efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |        gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |        gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |        cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |        ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |        gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |        fgae cfgab fg bagce";

    #[test]
    fn test_answer1_mock_data() {
        assert_eq!(answer1(str_lines_to_string(MOCK_DATA)), 26)
    }

    #[test]
    fn test_answer2_mock_data() {
        assert_eq!(answer2(str_lines_to_string(MOCK_DATA)), 61229)
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(load(DATA)), 488)
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(load(DATA)), 1040429)
    }
}
