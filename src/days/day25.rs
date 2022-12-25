use crate::harness::input::RawInput;

const DIGITS: &[u8] = b"=-012";

pub fn solve_part1(input: RawInput) -> String {
    let sum: i64 = input
        .per_line(|line| from_snafu(line.as_str().as_bytes()))
        .sum();
    to_snafu(sum)
}

pub fn solve_part2(_: RawInput) -> usize {
    todo!()
}

fn from_snafu(bs: &[u8]) -> i64 {
    let mut out = 0;
    for b in bs {
        out *= 5;
        out += DIGITS.iter().position(|digit| digit == b).unwrap() as i64 - 2;
    }
    out
}

fn to_snafu(mut n: i64) -> String {
    let mut out = vec![];
    while n > 0 {
        let rem = ((n + 2) % 5) - 2;
        out.push(DIGITS[(rem + 2) as usize]);
        n = (n - rem) / 5;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}
