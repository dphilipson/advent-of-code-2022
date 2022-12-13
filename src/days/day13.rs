use std::cmp;
use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re::parse_with_regex;
use std::cmp::Ordering;
use std::str::FromStr;
use Packet::{List, Number};

pub fn solve_part1(input: RawInput) -> usize {
   input.grouped_lines(|line| line.single::<Packet>())
       .enumerate()
       .filter(|(_, pair)| pair[0] < pair[1])
       .map(|(i, _)| {
           i + 1
       })
       .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut packets: Vec<_> = input.as_str()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Packet>().unwrap())
        .collect();
    let sep1 = List(vec![List(vec![Number(2)])]);
    let sep2 = List(vec![List(vec![Number(6)])]);
    packets.push(sep1);
    packets.push(sep2);
    packets.sort();
    let sep1 = List(vec![List(vec![Number(2)])]);
    let sep2 = List(vec![List(vec![Number(6)])]);
    let i1 = packets.iter().position(|p| p.clone() == sep1).unwrap();
    let i2 = packets.iter().position(|p| p.clone() == sep2).unwrap();
    (i1 + 1) * (i2 + 1)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(read_item(s).0)
        // println!("Before: {s}");
        // let bytes = s.as_bytes();
        // if bytes[0] == b'[' {
        //     let inner = std::str::from_utf8(&bytes[1..bytes.len() - 1]).unwrap();
        //     if inner.is_empty() {
        //         return Ok(List(vec![]));
        //     }
        //     let mut parts = vec![];
        //     let mut start = 0;
        //     let mut end = 0;
        //     while end < inner.len() {
        //
        //     }
        //     let list = inner.split(',').map(|part| part.parse().unwrap()).collect();
        //     Ok(List(list))
        // } else {
        //     println!("{s}");
        //     Ok(Number(s.parse().unwrap()))
        // }
    }
}

fn read_item(s: &str) -> (Packet, String) {
    if let Some((n, remaining)) = read_number(s) {
        return (Number(n), remaining)
    }
    let mut i = 0;
    let mut bracket_count = 0;
    let bytes = s.as_bytes();
    loop {
        match bytes[i] {
            b'[' => bracket_count += 1,
            b']' => {
                bracket_count -= 1;
                if bracket_count == 0 {
                    break;
                }
            },
            _ => ()
        }
        i += 1;
    }
    let mut inner = std::str::from_utf8(&bytes[1..i]).unwrap().to_owned();
    let mut items = vec![];
    while !inner.is_empty() {
        let (part, remaining) = read_item(&inner);
        items.push(part);
        inner = remaining.trim_start_matches(',').to_owned();
    }
    (List(items), std::str::from_utf8(&bytes[i+1..]).unwrap().to_owned())
}

fn read_number(s: &str) -> Option<(u8, String)> {
    return parse_with_regex::<(u8, String)>(regex!(r"^(\d+)(.*)$"), s).ok()
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Number(n1) => match other {
                Number(n2) => n1.cmp(n2),
                List(_) => List(vec![Number(*n1)]).cmp(other),
            },
            List(list1) => match other {
                Number(_) => other.cmp(self).reverse(),
                List(list2) => {
                    for i in 0..cmp::min(list1.len(), list2.len()) {
                        let ord = list1[i].cmp(&list2[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    list1.len().cmp(&list2.len())
                }
            },
        }
    }
}
