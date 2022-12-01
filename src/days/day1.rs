use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u32 {
    let groups = input.grouped_lines(|line| line.single::<u32>());
    groups
        .into_iter()
        .map(|group| group.into_iter().sum())
        .max()
        .unwrap()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let groups = input.grouped_lines(|line| line.single::<u32>());
    let mut sums: Vec<u32> = groups
        .into_iter()
        .map(|group| group.into_iter().sum())
        .collect();
    sums.sort();
    sums.into_iter().rev().take(3).sum()
}
