use ndarray::Array2;
use std::{
    fmt::Debug,
    fs::{read_to_string, File},
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

fn buf_open(filename: impl AsRef<Path>) -> BufReader<File> {
    let file = File::open(filename).expect("No such file");
    BufReader::new(file)
}

pub fn file_to<T>(filename: impl AsRef<Path>) -> impl Iterator<Item = T>
where
    T::Err: Debug,
    T: FromStr,
{
    file_to_lines(filename).map(|l| {
        l.parse::<T>() // should this be TryFrom?
            .expect("Failed to parse")
    })
}

pub fn file_to_vec<T>(filename: impl AsRef<Path>) -> Vec<T>
where
    T::Err: Debug,
    T: FromStr,
{
    file_to(filename).collect()
}

pub fn file_to_array2(filename: impl AsRef<Path>) -> Array2<u32> {
    let data = read_to_string(filename).expect("Failed to open file");
    let width = data
        .chars()
        .position(|c| c == '\n')
        .unwrap_or_else(|| data.len() - 1);
    let data: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();
    Array2::from_shape_vec((data.len() / width, width), data).expect("Could not build array")
}

pub fn delimited_file_to<T>(filename: impl AsRef<Path>, delim_byte: u8) -> impl Iterator<Item = T>
where
    T: FromStr,
    T::Err: Debug,
{
    buf_open(filename).split(delim_byte).map(|bytes| {
        std::str::from_utf8(&bytes.expect("Unexpected IO interruption"))
            .expect("Failed to read str from utf8")
            .trim()
            .parse::<T>()
            .expect("Failed to parse")
    })
}

// pub fn file_chars_as_digits(filename: impl AsRef<Path>) -> impl Iterator<Item = u32> {
//     buf_open(filename).bytes().map(|byte| {
//         (byte.expect("Unexpected IO interruption") as char)
//             .to_digit(10)
//             .expect("Failed to parse")
//     })
// }

pub fn file_to_lines(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    buf_open(filename)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
}

// pub fn file_to_paragraphs(filename: impl AsRef<Path>) -> impl Iterator<Item = Vec<String>> {
//     // I wish I could figure out how to make this an iterator of iterators, but I'm having trouble
//     // with the lifetimes.
//     // Perhaps the issue lies in https://github.com/rust-lang/rust/issues/61756 ?
//     file_to_lines(filename).peekable().batching(|lines| {
//         lines.peek()?; // Don't keep iterating if the iterator is empty
//         Some(lines.take_while(|line| !line.is_empty()).collect())
//     })
// }
