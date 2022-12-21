use crate::harness::input::RawInput;
use arrayvec::{ArrayString, ArrayVec};
use std::collections::HashMap;
use std::error;
use std::str::FromStr;

pub fn solve_part1(input: RawInput) -> i64 {
    let monkeys: Vec<_> = input.per_line(|line| line.single::<Monkey>()).collect();
    solve(&monkeys, "root")
}

pub fn solve_part2(input: RawInput) -> i64 {
    let monkeys = get_part2_monkeys(input);
    solve(&monkeys, "humn")
}

fn get_part2_monkeys(input: RawInput) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for monkey in input.per_line(|line| line.single::<Monkey>()) {
        match monkey.label.as_str() {
            "root" => {
                if let Action::Combine(_, label1, label2) = monkey.action {
                    monkeys.extend([
                        Monkey::new(label1, Action::Watch(label2)),
                        Monkey::new(label2, Action::Watch(label1)),
                    ]);
                } else {
                    panic!("Root monkey was not watching other monkeys.");
                }
            }
            "humn" => (),
            _ => {
                monkeys.push(monkey);
                monkeys.extend(monkey.inverses());
            }
        }
    }
    monkeys
}

fn solve(monkeys: &[Monkey], root: &str) -> i64 {
    let mut monkeys_by_child = HashMap::<Label, ArrayVec<Monkey, 4>>::new();
    for &monkey in monkeys {
        match monkey.action {
            Action::Combine(_, label1, label2) => {
                for label in [label1, label2] {
                    monkeys_by_child.entry(label).or_default().push(monkey);
                }
            }
            Action::Watch(label) => {
                monkeys_by_child.entry(label).or_default().push(monkey);
            }
            _ => (),
        }
    }
    let mut values_by_label = HashMap::<Label, i64>::new();
    let mut pending: Vec<_> = monkeys
        .iter()
        .filter(|&&monkey| matches!(monkey.action, Action::Yell(_)))
        .collect();
    while let Some(&monkey) = pending.pop() {
        if values_by_label.contains_key(&monkey.label) {
            continue;
        }
        if let Some(value) = monkey.action.evaluate(&values_by_label) {
            if &monkey.label == root {
                return value;
            }
            values_by_label.insert(monkey.label, value);
            pending.extend(&monkeys_by_child[&monkey.label]);
        }
    }
    panic!("Could not evaluate root.");
}

type Label = ArrayString<4>;

#[derive(Copy, Clone, Debug)]
struct Monkey {
    label: Label,
    action: Action,
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, action) = s.split_once(": ").ok_or("No colon")?;
        let label: Label = label.parse()?;
        let action: Action = action.parse()?;
        Ok(Self::new(label, action))
    }
}

impl Monkey {
    fn new(label: Label, action: Action) -> Self {
        Self { label, action }
    }

    fn inverses(self) -> ArrayVec<Monkey, 2> {
        match self.action {
            Action::Yell(_) => ArrayVec::new(),
            Action::Combine(op, label1, label2) => ArrayVec::from(match op {
                Op::Add => [
                    Self::new(label1, Action::Combine(Op::Sub, self.label, label2)),
                    Self::new(label2, Action::Combine(Op::Sub, self.label, label1)),
                ],
                Op::Sub => [
                    Self::new(label1, Action::Combine(Op::Add, self.label, label2)),
                    Self::new(label2, Action::Combine(Op::Sub, label1, self.label)),
                ],
                Op::Mul => [
                    Self::new(label1, Action::Combine(Op::Div, self.label, label2)),
                    Self::new(label2, Action::Combine(Op::Div, self.label, label1)),
                ],
                Op::Div => [
                    Self::new(label1, Action::Combine(Op::Mul, self.label, label2)),
                    Self::new(label2, Action::Combine(Op::Div, label1, self.label)),
                ],
            }),
            Action::Watch(_) => ArrayVec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Yell(i64),
    Combine(Op, Label, Label),
    Watch(Label),
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

impl FromStr for Action {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let out = if parts.len() == 1 {
            Self::Yell(parts[0].parse()?)
        } else {
            let op = match parts[1] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => Err("Unrecognized operation")?,
            };
            let label1: Label = parts[0].parse()?;
            let label2: Label = parts[2].parse()?;
            Self::Combine(op, label1, label2)
        };
        Ok(out)
    }
}

impl Action {
    fn evaluate(self, values_by_label: &HashMap<Label, i64>) -> Option<i64> {
        match self {
            Self::Yell(value) => Some(value),
            Self::Combine(op, label1, label2) => values_by_label.get(&label1).and_then(|&value1| {
                values_by_label
                    .get(&label2)
                    .map(|&value2| op.apply(value1, value2))
            }),
            Self::Watch(label) => values_by_label.get(&label).copied(),
        }
    }
}
