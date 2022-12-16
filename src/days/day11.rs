use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re::{parse_with_regex, Str};
use std::collections::VecDeque;
use std::error;
use std::str::FromStr;
use Op::{Add, Mul, Square};

pub fn solve_part1(input: RawInput) -> u64 {
    solve(input, 20, 3, false)
}

pub fn solve_part2(input: RawInput) -> u64 {
    solve(input, 10000, 1, true)
}

fn solve(input: RawInput, num_rounds: u32, divisor: u64, use_modulus: bool) -> u64 {
    let mut monkeys: Monkeys = input.as_str().parse().unwrap();
    let modulus = if use_modulus {
        monkeys.0.iter().map(|monkey| monkey.test_factor).product()
    } else {
        u64::MAX
    };
    for _ in 0..num_rounds {
        monkeys.execute_round(divisor, modulus);
    }
    monkeys.get_monkey_business()
}

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);

impl FromStr for Monkeys {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys: Result<Vec<Monkey>, Self::Err> =
            s.trim().split("\n\n").map(str::parse).collect();
        Ok(Self(monkeys?))
    }
}

impl Monkeys {
    fn execute_round(&mut self, divisor: u64, modulus: u64) {
        for i in 0..self.0.len() {
            while let Some((worry_level, target)) = self.0[i].toss_next_item(divisor, modulus) {
                self.0[target].items.push_back(worry_level);
            }
        }
    }

    fn get_monkey_business(&self) -> u64 {
        let mut toss_counts: Vec<_> = self.0.iter().map(|monkey| monkey.toss_count).collect();
        toss_counts.sort();
        toss_counts.reverse();
        toss_counts[0] * toss_counts[1]
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test_factor: u64,
    true_target: usize,
    false_target: usize,
    toss_count: u64,
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(
            r"Monkey \d+:
  Starting items: (.+?)
  Operation: new = old (.) (.+?)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"
        );
        let (Str(items), op_char, Str(op_value), test_factor, true_target, false_target): (
            Str,
            char,
            Str,
            u64,
            usize,
            usize,
        ) = parse_with_regex(re, s)?;
        let items: VecDeque<u64> = items.split(", ").map(|s| s.parse().unwrap()).collect();
        let op = if op_value == "old" {
            Square
        } else {
            let op_value = op_value.parse().unwrap();
            if op_char == '+' {
                Add(op_value)
            } else {
                Mul(op_value)
            }
        };
        Ok(Self {
            items,
            op,
            test_factor,
            true_target,
            false_target,
            toss_count: 0,
        })
    }
}

impl Monkey {
    fn toss_next_item(&mut self, divisor: u64, modulus: u64) -> Option<(u64, usize)> {
        self.items.pop_front().map(|item| {
            let worry_level = (self.op.apply(item) / divisor) % modulus;
            let target = if worry_level % self.test_factor == 0 {
                self.true_target
            } else {
                self.false_target
            };
            self.toss_count += 1;
            (worry_level, target)
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    fn apply(self, x: u64) -> u64 {
        match self {
            Add(y) => x + y,
            Mul(y) => x * y,
            Square => x * x,
        }
    }
}
