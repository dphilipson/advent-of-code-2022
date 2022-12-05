use crate::harness::input::{LineInput, RawInput};

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .per_line(parse_to_values)
        .map(|(their_move, our_move)| score_moves(their_move, our_move))
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    input
        .per_line(parse_to_values)
        .map(|(their_move, desired_result)| {
            score_moves(their_move, required_move(their_move, desired_result))
        })
        .sum()
}

fn parse_to_values(line: LineInput) -> (u8, u8) {
    let bytes = line.bytes();
    (bytes[0] - b'A' + 1, bytes[2] - b'X' + 1)
}

fn score_moves(their_move: u8, our_move: u8) -> u32 {
    // A bit cheeky. The expression `(4 + our_move - their_move) % 3` works out
    // to 0, 1, or 2 if we lose, draw, or win respectively.
    (our_move + (((4 + our_move - their_move) % 3) * 3)) as u32
}

fn required_move(their_move: u8, desired_result: u8) -> u8 {
    // Also cheeky. This is what you get if you set `desired_result - 1` equal
    // to the expression from the previous comment and solve for `our_move`.
    ((their_move + desired_result) % 3) + 1
}
