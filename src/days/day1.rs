use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u32 {
    parse_sums(input).max().unwrap()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let mut sums: Vec<_> = parse_sums(input).collect();
    sums.sort();
    sums.into_iter().rev().take(3).sum()
}

fn parse_sums(input: RawInput) -> impl Iterator<Item = u32> {
    input
        .grouped_lines(|line| line.single::<u32>())
        .into_iter()
        .map(|group| group.into_iter().sum())
}
