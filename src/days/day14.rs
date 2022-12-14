use crate::harness::input::RawInput;
use std::cmp;
use std::collections::HashSet;

type Coord = (u32, u32);

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, |pos, max_y| pos.1 < max_y)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, |pos, _| pos != (500, 0)) + 1
}

fn solve(input: RawInput, should_continue: impl Fn(Coord, u32) -> bool) -> usize {
    let mut filled_spots = HashSet::new();
    input
        .as_str()
        .lines()
        .map(parse_veins)
        .for_each(|vein| add_rocks(&mut filled_spots, &vein));
    let max_y = filled_spots.iter().map(|pos| pos.1).max().unwrap();
    let mut count = 0;
    while should_continue(drop_sand(&mut filled_spots, max_y), max_y) {
        count += 1;
    }
    count
}

fn drop_sand(filled_spots: &mut HashSet<Coord>, max_y: u32) -> Coord {
    let mut sand = (500, 0);
    while let Some(new_sand) = move_sand(filled_spots, max_y, sand) {
        sand = new_sand;
    }
    filled_spots.insert(sand);
    sand
}

fn move_sand(filled_spots: &HashSet<Coord>, max_y: u32, sand_pos: Coord) -> Option<Coord> {
    let (x, y) = sand_pos;
    let candidates = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
    candidates
        .into_iter()
        .find(|candidate| candidate.1 != max_y + 2 && !filled_spots.contains(candidate))
}

fn add_rocks(filled_spots: &mut HashSet<Coord>, veins: &[Coord]) {
    for i in 0..veins.len() - 1 {
        let (start_x, start_y) = veins[i];
        let (end_x, end_y) = veins[i + 1];
        if start_x == end_x {
            for y in cmp::min(start_y, end_y)..=cmp::max(start_y, end_y) {
                filled_spots.insert((start_x, y));
            }
        } else {
            for x in cmp::min(start_x, end_x)..=cmp::max(start_x, end_x) {
                filled_spots.insert((x, start_y));
            }
        }
    }
}

fn parse_veins(line: &str) -> Vec<Coord> {
    line.split(" -> ")
        .map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}
