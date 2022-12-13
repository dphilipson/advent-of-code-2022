use crate::harness::input::RawInput;
use serde_json::Value;
use std::cmp;
use std::cmp::Ordering;
use Value::{Array, Number};

pub fn solve_part1(input: RawInput) -> usize {
    input
        .grouped_lines(|line| serde_json::from_str::<Value>(line.as_str()).unwrap())
        .enumerate()
        .filter(|(_, pair)| cmp_values(&pair[0], &pair[1]) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut packets: Vec<Value> = input
        .as_str()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let sep1: Value = serde_json::from_str("[[2]]").unwrap();
    let sep2: Value = serde_json::from_str("[[6]]").unwrap();
    packets.extend([sep1.clone(), sep2.clone()]);
    packets.sort_by(cmp_values);
    let i1 = packets.iter().position(|p| p == &sep1).unwrap() + 1;
    let i2 = packets.iter().position(|p| p == &sep2).unwrap() + 1;
    i1 * i2
}

fn cmp_values(v1: &Value, v2: &Value) -> Ordering {
    match (v1, v2) {
        (Number(n1), Number(n2)) => n1.as_u64().cmp(&n2.as_u64()),
        (Number(_), Array(_)) => cmp_values(&Array(vec![v1.clone()]), v2),
        (Array(_), Number(_)) => cmp_values(v2, v1).reverse(),
        (Array(list1), Array(list2)) => {
            for i in 0..cmp::min(list1.len(), list2.len()) {
                let ord = cmp_values(&list1[i], &list2[i]);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            list1.len().cmp(&list2.len())
        }
        _ => panic!(),
    }
}
