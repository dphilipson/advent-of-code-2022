use std::collections::{HashMap, HashSet};
use crate::harness::input::RawInput;
use crate::util::coords::Coord2;

pub fn solve_part1(input: RawInput) -> i32 {
    let mut positions = parse_positions(input);
    for round in 0..10 {
        positions = advance(&positions, round);
    }
    let min_x = positions.iter().map(|c| c.0).min().unwrap();
    let max_x = positions.iter().map(|c| c.0).max().unwrap();
    let min_y = positions.iter().map(|c| c.1).min().unwrap();
    let max_y = positions.iter().map(|c| c.1).max().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - positions.len() as i32
}

pub fn solve_part2(input: RawInput) -> u32 {
    let mut positions = parse_positions(input);
    for round in 0.. {
        let new_positions = advance(&positions, round);
        if new_positions == positions{
            return round + 1;
        }
        positions = new_positions;
    }
    unreachable!()
}

type Coord = Coord2<i32>;


fn parse_positions(input: RawInput) -> HashSet<Coord> {
    let mut out = HashSet::new();
    let lines = input.as_str().trim_end().lines().rev().enumerate();
    for (y, line) in lines {
        for (x, &b) in line.as_bytes().iter().enumerate() {
            if b == b'#' {
                out.insert(Coord2(x as i32, y as i32));
            }
        }
    }
    out
}

const MOVES: [[Coord; 3]; 4] = [
    [Coord2(-1, 1), Coord2(0, 1), Coord2(1, 1)],
    [Coord2(-1, -1), Coord2(0, -1), Coord2(1, -1)],
    [Coord2(-1, -1), Coord2(-1, 0), Coord2(-1, 1)],
    [Coord2(1, -1), Coord2(1, 0), Coord2(1, 1)]
];

fn advance(elves: &HashSet<Coord>, round: u32) -> HashSet<Coord> {
    let mut proposed_moves = HashMap::<Coord, Coord>::new();
    let mut target_counts = HashMap::<Coord, u32>::new();
    for &elf in elves {
        let has_neighbors = elf.neighbors()
            .iter()
            .any(|neighbor| elves.contains(neighbor));
        if !has_neighbors {
            continue;
        }
        for i in round..round + 4 {
            let mooves = MOVES[i as usize % 4];
            if mooves.iter().all(|&moove| !elves.contains(&(elf + moove))) {
                let target = elf + mooves[1];
                proposed_moves.insert(elf, target);
                *target_counts.entry(target).or_default() += 1;
                break;
            }
        }
    }
    let mut out = HashSet::new();
    for elf in elves {
        if let Some(proposed) = proposed_moves.get(elf) {
            if target_counts.get(proposed) == Some(&1) {
                out.insert(*proposed);
                continue
            }
        }
        out.insert(*elf);
    }
    out
}