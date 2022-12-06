use crate::harness::input::RawInput;
use std::collections::HashSet;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 4)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 14)
}

fn solve(input: RawInput, window_size: usize) -> usize {
    input
        .as_str()
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|&(_, bytes)| bytes.iter().collect::<HashSet<_>>().len() == window_size)
        .unwrap()
        .0
        + window_size
}
