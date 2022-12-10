use crate::harness::input::RawInput;
use std::num::ParseIntError;
use std::str::FromStr;
use Instruction::{Addx, Noop};

pub fn solve_part1(input: RawInput) -> i32 {
    let instructions = input.per_line(|line| line.single()).collect();
    let mut state = State::new(instructions);
    let mut result = 0;
    while !state.is_done() {
        let x = state.x;
        state.advance();
        if (state.cycle - 20) % 40 == 0 {
            result += state.cycle * x;
        }
    }
    result
}

pub fn solve_part2(input: RawInput) -> u8 {
    let instructions = input.per_line(|line| line.single()).collect();
    let mut state = State::new(instructions);
    let mut chars = vec![];
    while !state.is_done() {
        let x = state.x;
        let column = state.cycle % 40;
        let char = if (column - x).abs() <= 1 { '█' } else { ' ' };
        chars.push(char);
        if column == 39 {
            chars.push('\n');
        }
        state.advance();
    }
    println!("{}", chars.into_iter().collect::<String>());
    0
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(' ') {
            Some((_, n)) => Addx(n.parse()?),
            None => Noop,
        })
    }
}

#[derive(Debug)]
struct State {
    instructions: Vec<Instruction>,
    instruction_index: usize,
    cycle: i32,
    x: i32,
    is_adding: bool,
}

impl State {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            instruction_index: 0,
            cycle: 0,
            x: 1,
            is_adding: false,
        }
    }

    fn advance(&mut self) {
        self.cycle += 1;
        match self.instructions[self.instruction_index] {
            Addx(n) => {
                if self.is_adding {
                    self.x += n;
                    self.instruction_index += 1;
                }
                self.is_adding = !self.is_adding
            }
            Noop => self.instruction_index += 1,
        }
    }

    fn is_done(&self) -> bool {
        self.instruction_index == self.instructions.len()
    }
}
