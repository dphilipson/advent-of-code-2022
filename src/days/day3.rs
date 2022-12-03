use std::collections::HashSet;

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .as_str()
        .lines()
        .map(intersect_string_halves)
        .map(priority)
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    input
        .as_str()
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(intersect_strs)
        .map(priority)
        .sum()
}

fn intersect_string_halves(s: &str) -> u8 {
    let split_index = s.len() / 2;
    intersect_strs(&[&s[..split_index], &s[split_index..]])
}

fn intersect_strs(strs: &[&str]) -> u8 {
    let mut iter = strs.iter().map(|s| s.bytes().collect::<HashSet<_>>());
    let first = iter.next().unwrap();
    let intersection = iter.fold(first, |set1, set2| &set1 & &set2);
    intersection.into_iter().next().unwrap()
}

fn priority(c: u8) -> u32 {
    if c.is_ascii_lowercase() {
        (c - b'a' + 1) as u32
    } else {
        (c - b'A' + 27) as u32
    }
}
