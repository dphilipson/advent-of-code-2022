use crate::harness::input::RawInput;
use arrayvec::{ArrayString, ArrayVec};
use std::collections::HashMap;
use std::error;
use std::str::FromStr;

pub fn solve_part1(input: RawInput) -> i64 {
    let monkeys: Vec<_> = input.per_line(|line| line.single::<Monkey>()).collect();
    compute_monkey_values(&monkeys)["root"]
}

pub fn solve_part2(input: RawInput) -> i64 {
    let monkeys: Vec<_> = input
        .per_line(|line| line.single::<Monkey>())
        .filter(|monkey| &monkey.label != "humn")
        .collect();
    let &root_monkey = monkeys
        .iter()
        .find(|&monkey| &monkey.label == "root")
        .unwrap();
    if let Action::Combine(_, label1, label2) = root_monkey.action {
        let values_by_label = compute_monkey_values(&monkeys);
        let updated_monkeys: Vec<_> = monkeys
            .iter()
            .map(|&monkey| {
                if let Some(&value) = values_by_label.get(&monkey.label) {
                    Monkey {
                        label: monkey.label,
                        action: Action::Yell(value),
                    }
                } else {
                    monkey
                }
            })
            .collect();
        if let Some(&value) = values_by_label.get(&label1) {
            return solve_inverse(&updated_monkeys, &label2, value);
        } else if let Some(&value) = values_by_label.get(&label2) {
            return solve_inverse(&updated_monkeys, &label1, value);
        }
    } else {
        panic!("Couldn't match root.");
    }
    panic!("Couldn't solve either side.");
}

fn compute_monkey_values(monkeys: &[Monkey]) -> HashMap<Label, i64> {
    let mut monkeys_by_child: HashMap<_, _> = monkeys
        .iter()
        .map(|monkey| (monkey.label, ArrayVec::<Monkey, 3>::new()))
        .collect();
    for &monkey in monkeys {
        if let Action::Combine(_, label1, label2) = monkey.action {
            for label in [label1, label2] {
                if let Some(vec) = monkeys_by_child.get_mut(&label) {
                    vec.push(monkey);
                }
            }
        }
    }
    let mut values_by_label = HashMap::<Label, i64>::new();
    let mut pending: Vec<_> = monkeys
        .iter()
        .filter(|&&monkey| match monkey.action {
            Action::Yell(_) => true,
            Action::Combine(_, _, _) => false,
        })
        .collect();
    while let Some(&monkey) = pending.pop() {
        if values_by_label.contains_key(&monkey.label) {
            continue; // May not be necessary.
        }
        if let Some(value) = monkey.action.evaluate(&values_by_label) {
            values_by_label.insert(monkey.label, value);
            pending.extend(&monkeys_by_child[&monkey.label]);
        }
    }
    values_by_label
}

fn solve_inverse(monkeys: &[Monkey], root: &str, root_value: i64) -> i64 {
    let inverse_monkeys: Vec<_> = monkeys
        .iter()
        .flat_map(|monkey| monkey.inverses())
        .map(|monkey| {
            if &monkey.label == root {
                Monkey {
                    label: Label::from(root).unwrap(),
                    action: Action::Yell(root_value),
                }
            } else {
                monkey
            }
        })
        .collect();
    compute_monkey_values(&inverse_monkeys)["humn"]
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
        Ok(Self { label, action })
    }
}

impl Monkey {
    fn inverses(self) -> ArrayVec<Self, 2> {
        match self.action {
            Action::Yell(_) => ArrayVec::from_iter([self]),
            Action::Combine(op, label1, label2) => ArrayVec::from(match op {
                Op::Add => [
                    Self {
                        label: label1,
                        action: Action::Combine(Op::Sub, self.label, label2),
                    },
                    Self {
                        label: label2,
                        action: Action::Combine(Op::Sub, self.label, label1),
                    },
                ],
                Op::Sub => [
                    Self {
                        label: label1,
                        action: Action::Combine(Op::Add, self.label, label2),
                    },
                    Self {
                        label: label2,
                        action: Action::Combine(Op::Sub, label1, self.label),
                    },
                ],
                Op::Mul => [
                    Self {
                        label: label1,
                        action: Action::Combine(Op::Div, self.label, label2),
                    },
                    Self {
                        label: label2,
                        action: Action::Combine(Op::Div, self.label, label1),
                    },
                ],
                Op::Div => [
                    Self {
                        label: label1,
                        action: Action::Combine(Op::Mul, self.label, label2),
                    },
                    Self {
                        label: label2,
                        action: Action::Combine(Op::Div, label1, self.label),
                    },
                ],
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Yell(i64),
    Combine(Op, Label, Label),
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
            Self::Combine(op, label1, label2) => {
                if let Some(&v1) = values_by_label.get(&label1) {
                    if let Some(&v2) = values_by_label.get(&label2) {
                        return Some(op.apply(v1, v2));
                    }
                }
                None
            }
        }
    }
}
