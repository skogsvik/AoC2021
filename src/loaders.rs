use std::{
    fmt::Debug,
    fs::File,
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
    buf_open(filename).lines().map(|l| {
        l.expect("Could not parse line")
            .parse::<T>()
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

pub fn file_to_lines(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    buf_open(filename)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
}
