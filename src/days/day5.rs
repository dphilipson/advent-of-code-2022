use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re;
use std::error;
use std::str::FromStr;

pub fn solve_part1(input: RawInput) -> String {
    let Input { mut crates, moves } = input.as_str().parse().unwrap();
    for moove in moves {
        crates.apply_part1_move(moove);
    }
    crates.read_top_crates()
}

pub fn solve_part2(input: RawInput) -> String {
    let Input { mut crates, moves } = input.as_str().parse().unwrap();
    for moove in moves {
        crates.apply_part2_move(moove);
    }
    crates.read_top_crates()
}

#[derive(Debug)]
struct Input {
    crates: Crates,
    moves: Vec<Move>,
}

impl FromStr for Input {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (crates_str, moves_str) = s.split_once("\n\n").ok_or("Could not split.")?;
        let crates = crates_str.parse()?;
        let mut moves = vec![];
        for line in moves_str.lines() {
            moves.push(line.parse()?);
        }
        Ok(Self { crates, moves })
    }
}

#[derive(Debug)]
struct Crates(Vec<Vec<u8>>);

impl FromStr for Crates {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.lines().rev().skip(1).map(parse_crate_row).collect();
        let ncols = rows.first().unwrap().len();
        let mut result = vec![Vec::new(); ncols];
        for row in rows {
            for (i, char) in row {
                result[i].push(char)
            }
        }
        Ok(Self(result))
    }
}

impl Crates {
    fn apply_part1_move(&mut self, Move { count, start, end }: Move) {
        for _ in 0..count {
            let crait = self.0[start].pop().unwrap();
            self.0[end].push(crait);
        }
    }

    fn apply_part2_move(&mut self, Move { count, start, end }: Move) {
        let start_col = &mut self.0[start];
        let crates = start_col.split_off(start_col.len() - count);
        self.0[end].extend(crates);
    }

    fn read_top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| *stack.last().unwrap() as char)
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    count: usize,
    start: usize,
    end: usize,
}

impl FromStr for Move {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, start, end): (usize, usize, usize) =
            re::parse_with_regex(regex!(r"^move (\d+) from (\d+) to (\d+)$"), s)?;
        Ok(Self {
            count,
            start: start - 1,
            end: end - 1,
        })
    }
}

fn parse_crate_row(s: &str) -> Vec<(usize, u8)> {
    s.bytes()
        .skip(1)
        .step_by(4)
        .enumerate()
        .filter(|&(_, b)| b != b' ')
        .collect()
}
