pub fn str_lines_to_string(input: &str) -> impl Iterator<Item = String> + '_{
    input.lines().map(ToOwned::to_owned)
}
