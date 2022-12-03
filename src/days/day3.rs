use std::collections::HashSet;

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .per_line(|line| line.bytes())
        .into_iter()
        .map(process_line_for_part_1)
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let lines = input.per_line(|line| line.bytes());
    let mut i = 0;
    let mut sum = 0;
    while i < lines.len() {
        let cs1: HashSet<_> = lines[i].iter().copied().collect();
        let cs2: HashSet<_> = lines[i + 1].iter().copied().collect();
        let cs3: HashSet<_> = lines[i + 2].iter().copied().collect();
        let cs12: HashSet<_> = cs1.intersection(&cs2).copied().collect();
        let cs123: Vec<_> = cs12.intersection(&cs3).copied().collect();
        assert_eq!(cs123.len(), 1, "Didn't find just one match.");
        sum += priority(*cs123.first().unwrap());
        i += 3
    }
    sum
}

fn process_line_for_part_1(cs: Vec<u8>) -> u32 {
    let len = cs.len();
    let first_half_set: HashSet<_> = cs.iter().take(len / 2).copied().collect();
    let dupe = cs
        .iter()
        .skip(len / 2)
        .find(|&x| first_half_set.contains(x))
        .unwrap()
        .to_owned();
    priority(dupe)
}

fn priority(c: u8) -> u32 {
    if (b'a'..=b'z').contains(&c) {
        (c - b'a' + 1) as u32
    } else if (b'A'..=b'Z').contains(&c) {
        (c - b'A' + 27) as u32
    } else {
        panic!("Invalid char {c}")
    }
}
