use crate::harness::input::RawInput;
use crate::regex;

pub fn solve_part1(input: RawInput) -> usize {
    input
        .per_line(|line| {
            line.parse_with_regex::<(u32, u32, u32, u32)>(regex!(r"(\d+)-(\d+),(\d+)-(\d+)"))
        })
        .filter(|&(start1, end1, start2, end2)| {
            (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
        })
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    input
        .per_line(|line| {
            line.parse_with_regex::<(u32, u32, u32, u32)>(regex!(r"(\d+)-(\d+),(\d+)-(\d+)"))
        })
        .filter(|&(start1, end1, start2, end2)| {
            (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1)
        })
        .count()
}
