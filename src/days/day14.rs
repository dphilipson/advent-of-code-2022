use crate::harness::input::RawInput;
use ndarray::Array2;
use std::cmp;

// Optimized for speed. Gets 300x speedup over the straightforward solution by
// using a grid of booleans instead of a hashset and by tracking the most recent
// path taken by sand, since the path only differs in the last element from one
// sand drop to the next.

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, |pos, max_y| pos[1] < max_y)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, |pos, _| pos != [500, 0]) + 1
}

fn solve(input: RawInput, should_continue: impl Fn([usize; 2], usize) -> bool) -> usize {
    let veins: Vec<_> = input.as_str().lines().map(parse_veins).collect();
    let mut world = SandWorld::from_veins(&veins);
    let mut count = 0;
    while should_continue(world.drop_sand(), world.max_y) {
        count += 1;
    }
    count
}

#[derive(Debug)]
struct SandWorld {
    filled_spots: Array2<bool>,
    latest_path: Vec<[usize; 2]>,
    max_y: usize,
}

impl SandWorld {
    fn from_veins(veins: &[Vec<[usize; 2]>]) -> Self {
        let max_y = veins.iter().flatten().map(|pos| pos[1]).max().unwrap();
        let filled_spots: Array2<bool> = Array2::default((500 + max_y + 2, max_y + 2));
        let mut result = Self {
            filled_spots,
            max_y,
            latest_path: vec![[500, 0]],
        };
        for vein in veins {
            result.add_vein(vein);
        }
        result
    }

    fn drop_sand(&mut self) -> [usize; 2] {
        loop {
            let sand = *self.latest_path.last().unwrap();
            let [x, y] = sand;
            let candidates = [[x, y + 1], [x - 1, y + 1], [x + 1, y + 1]];
            let next_spot = candidates
                .into_iter()
                .find(|&candidate| candidate[1] != self.max_y + 2 && !self.filled_spots[candidate]);
            match next_spot {
                Some(spot) => self.latest_path.push(spot),
                None => {
                    self.latest_path.pop();
                    self.filled_spots[sand] = true;
                    return sand;
                }
            }
        }
    }

    fn add_vein(&mut self, vein: &[[usize; 2]]) {
        for i in 0..vein.len() - 1 {
            let [start_x, start_y] = vein[i];
            let [end_x, end_y] = vein[i + 1];
            if start_x == end_x {
                for y in cmp::min(start_y, end_y)..=cmp::max(start_y, end_y) {
                    self.filled_spots[[start_x, y]] = true;
                }
            } else {
                for x in cmp::min(start_x, end_x)..=cmp::max(start_x, end_x) {
                    self.filled_spots[[x, start_y]] = true;
                }
            }
        }
    }
}

fn parse_veins(line: &str) -> Vec<[usize; 2]> {
    line.split(" -> ")
        .map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            [a.parse().unwrap(), b.parse().unwrap()]
        })
        .collect()
}
