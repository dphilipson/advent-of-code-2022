use crate::harness::input::RawInput;
use crate::util::coords::Coord2;
use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: RawInput) -> i32 {
    let mut state = State::new(parse_movements(input));
    for _ in 0..2022 {
        state.drop_block();
    }
    state.max_height
}

pub fn solve_part2(input: RawInput) -> u64 {
    let mut state = State::new(parse_movements(input));
    let limit: u64 = 1000000000000;
    let CycleParams {
        block_count,
        height,
        cycle_length,
        height_gain_per_cycle,
    } = find_cycle_params(&mut state);
    state.reset();
    let remaining_step_count = limit - block_count;
    let complete_cycles = remaining_step_count / cycle_length;
    let height_after_cycles = height + complete_cycles * height_gain_per_cycle;
    let leftover_steps = remaining_step_count % cycle_length;
    for _ in 0..(block_count + leftover_steps) {
        state.drop_block();
    }
    let leftover_height_gain = state.max_height as u64 - height;
    height_after_cycles + leftover_height_gain
}

#[derive(Debug, Copy, Clone)]
struct CycleParams {
    block_count: u64,
    height: u64,
    cycle_length: u64,
    height_gain_per_cycle: u64,
}

fn find_cycle_params(state: &mut State) -> CycleParams {
    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
    struct Key {
        movement_index: usize,
        grid_snapshot: u128,
    }

    #[derive(Debug, Copy, Clone)]
    struct Value {
        block_count: i32,
        height: i32,
    }

    let mut snapshot_map = HashMap::<Key, Value>::new();
    loop {
        state.drop_block();
        if state.block_count % 5 == 0 {
            if let Some(grid_snapshot) = state.take_grid_snapshot() {
                let key = Key {
                    movement_index: state.movement_index,
                    grid_snapshot,
                };
                if let Some(&old_snapshot) = snapshot_map.get(&key) {
                    return CycleParams {
                        block_count: old_snapshot.block_count as u64,
                        height: old_snapshot.height as u64,
                        cycle_length: (state.block_count - old_snapshot.block_count) as u64,
                        height_gain_per_cycle: (state.max_height - old_snapshot.height) as u64,
                    };
                } else {
                    let value = Value {
                        block_count: state.block_count,
                        height: state.max_height,
                    };
                    snapshot_map.insert(key, value);
                }
            }
        }
    }
}

type Coord = Coord2<i32>;

#[derive(Debug)]
struct State {
    grid: HashSet<Coord>,
    block_count: i32,
    movement_index: usize,
    max_height: i32,
    movements: Vec<Coord>,
}

impl State {
    fn new(movements: Vec<Coord>) -> Self {
        Self {
            grid: HashSet::new(),
            block_count: 0,
            movement_index: 0,
            max_height: 0,
            movements,
        }
    }

    fn drop_block(&mut self) {
        let mut block = self.next_block();
        self.block_count += 1;
        block = self
            .try_move_block(&block, Coord2(2, self.max_height + 3))
            .unwrap();
        loop {
            let movement = self.movements[self.movement_index];
            self.movement_index = (self.movement_index + 1) % self.movements.len();
            if let Some(new_block) = self.try_move_block(&block, movement) {
                block = new_block;
            }
            if let Some(new_block) = self.try_move_block(&block, Coord2(0, -1)) {
                block = new_block;
            } else {
                break;
            }
        }
        for &part in &block {
            self.grid.insert(part);
            self.max_height = cmp::max(self.max_height, part.1 + 1);
        }
    }

    fn next_block(&self) -> Vec<Coord> {
        match self.block_count % 5 {
            0 => vec![Coord2(0, 0), Coord2(1, 0), Coord2(2, 0), Coord2(3, 0)],
            1 => vec![
                Coord2(0, 1),
                Coord2(1, 0),
                Coord2(1, 1),
                Coord2(1, 2),
                Coord2(2, 1),
            ],
            2 => vec![
                Coord2(0, 0),
                Coord2(1, 0),
                Coord2(2, 0),
                Coord2(2, 1),
                Coord2(2, 2),
            ],
            3 => vec![Coord2(0, 0), Coord2(0, 1), Coord2(0, 2), Coord2(0, 3)],
            4 => vec![Coord2(0, 0), Coord2(0, 1), Coord2(1, 0), Coord2(1, 1)],
            _ => panic!(),
        }
    }

    fn try_move_block(&self, block: &[Coord], displacement: Coord) -> Option<Vec<Coord>> {
        let new_block: Vec<_> = block.iter().map(|&part| part + displacement).collect();
        if self.block_overlaps(&new_block) {
            None
        } else {
            Some(new_block)
        }
    }

    fn block_overlaps(&self, block: &[Coord]) -> bool {
        block
            .iter()
            .any(|part| !(0..7).contains(&part.0) || part.1 < 0 || self.grid.contains(part))
    }

    /// Returns a bitset of the last eighteen rows
    fn take_grid_snapshot(&self) -> Option<u128> {
        if self.max_height < 18 {
            return None;
        }
        let mut out = 0;
        for dy in 0..18 {
            let y = self.max_height - 1 - dy;
            for x in 0..7 {
                if self.grid.contains(&Coord2(x, y)) {
                    let i = 18 * x + dy;
                    out |= 1 << i;
                }
            }
        }
        Some(out)
    }

    fn reset(&mut self) {
        self.grid.clear();
        self.block_count = 0;
        self.movement_index = 0;
        self.max_height = 0;
    }
}

fn parse_movements(input: RawInput) -> Vec<Coord> {
    input
        .as_str()
        .trim()
        .as_bytes()
        .iter()
        .map(|&b| match b {
            b'<' => Coord2(-1, 0),
            b'>' => Coord2(1, 0),
            _ => panic!(),
        })
        .collect()
}
