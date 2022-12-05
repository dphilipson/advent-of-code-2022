#![allow(dead_code)]
extern crate core;

mod days;
mod harness;
mod util;

use crate::days::{day, DAY};

fn main() {
    harness::solve(DAY, day::solve_part1, day::solve_part2);
}
