use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re::parse_with_regex;

pub fn solve_part1(input: RawInput) -> String {
    let (mut crates, moves) = parse_input(input);
    for muve in moves {
        apply_move(&mut crates, muve)
    }
    crates.into_iter()
        .map(|crait| *crait.last().unwrap() as char)
        .collect()
}

pub fn solve_part2(input: RawInput) -> String {
    let (mut crates, moves) = parse_input(input);
    for muve in moves {
        apply_big_move(&mut crates, muve)
    }
    crates.into_iter()
        .map(|crait| *crait.last().unwrap() as char)
        .collect()
}

fn parse_input(input: RawInput) -> (Vec<Vec<u8>>, Vec<(usize, usize, usize)>) {
    let groups: Vec<_> = input.grouped_lines(|line| line.single::<String>()).collect();
    let crates = parse_crates(&groups[0]);
    let moves = parse_moves(&groups[1]);
    (crates, moves)
}

fn parse_crates(input:&[String]) -> Vec<Vec<u8>> {
    let crate_rows: Vec<_> = input[..input.len() - 1]
        .into_iter()
        .map(|row| parse_crate_row(&row))
        .collect();
    let ncols = crate_rows.last().unwrap().len();
    let mut result: Vec<Vec<u8>> = (0..ncols).map(|_| Vec::new()).collect();
    for row in crate_rows.into_iter().rev() {
        for (i, char) in row {
            result[i].push(char)
        }
    }
    result
}

fn parse_moves(input: &[String]) -> Vec<(usize, usize, usize)> {
    input.iter()
        .map(|s|
            parse_with_regex::<(usize, usize, usize)>(
                regex!(r"move (\d+) from (\d+) to (\d+)"),
                s
            ).unwrap())
        .collect()
}

fn parse_crate_row(s: &str) -> Vec<(usize, u8)> {
    let bytes = s.as_bytes();
    let mut i = 1;
    let mut result = vec![];
    while i <= bytes.len() {
        if bytes[i] != b' ' {
            result.push((i / 4, bytes[i]))
        }
        i += 4;
    }
    result
}

fn apply_move(crates: &mut Vec<Vec<u8>>, (count, start, end): (usize, usize, usize)) {
    for _ in 0..count {
        let crait = crates[start - 1].pop().unwrap();
        crates[end - 1].push(crait);
    }
}

fn apply_big_move(crates: &mut Vec<Vec<u8>>, (count, start, end): (usize, usize, usize)) {
    let start_col = &crates[start - 1];
    let aagh = &start_col[start_col.len() - count..].to_owned();
    for _ in 0..count {
        crates[start - 1].pop();
    }
    crates[end - 1].extend(aagh);
}
