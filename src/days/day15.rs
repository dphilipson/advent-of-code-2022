use crate::harness::input::RawInput;
use crate::regex;
use crate::util::coords::Coord2;
use crate::util::re;
use std::collections::HashSet;
use std::str::FromStr;
use std::{cmp, error};

pub fn solve_part1(input: RawInput) -> usize {
    let y = 2000000;
    let readings: Vec<_> = input.per_line(|line| line.single::<Reading>()).collect();
    let num_beacons_on_row = readings
        .iter()
        .filter(|reading| reading.beacon.1 == y)
        .map(|reading| reading.beacon.0)
        .collect::<HashSet<_>>()
        .len();
    let mut intervals: Vec<_> = readings
        .into_iter()
        .filter_map(|reading| reading.get_banned_interval(y))
        .collect();
    let interval_sum: i32 = merge_intervals(&mut intervals)
        .into_iter()
        .map(|(min, max)| max - min)
        .sum();
    (interval_sum as usize) - num_beacons_on_row
}

pub fn solve_part2(input: RawInput) -> u64 {
    let circles: Vec<_> = input
        .per_line(|line| line.single::<Reading>())
        .map(Reading::to_manhattan_circle)
        .collect();
    for i in 0..circles.len() {
        for j in i..circles.len() {
            let intersection = circles[i].intersect_edge(circles[j]);
            for point in intersection {
                if circles.iter().all(|circle| !circle.contains(point))
                    && (0..=4000000).contains(&point.0)
                    && (0..=4000000).contains(&point.1)
                {
                    return (4000000 * point.0 as u64) + point.1 as u64;
                }
            }
        }
    }
    panic!()
}

type Coord = Coord2<i32>;

#[derive(Debug, Copy, Clone)]
struct Reading {
    sensor: Coord,
    beacon: Coord,
}

impl FromStr for Reading {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_x, s_y, b_x, b_y): (i32, i32, i32, i32) = re::parse_with_regex(
            regex!(r"^Sensor at x=(.?\d+), y=(.?\d+): closest beacon is at x=(.?\d+), y=(.?\d+)$"),
            s,
        )?;
        Ok(Self {
            sensor: Coord2(s_x, s_y),
            beacon: Coord2(b_x, b_y),
        })
    }
}

impl Reading {
    fn get_banned_interval(self, y: i32) -> Option<(i32, i32)> {
        let beacon_distance = (self.sensor - self.beacon).manhattan_norm();
        let Coord2(sx, sy) = self.sensor;
        let dy = (sy - y).abs();
        if beacon_distance < dy {
            None
        } else {
            let span = beacon_distance - dy;
            Some((sx - span, sx + span + 1))
        }
    }

    fn to_manhattan_circle(self) -> ManhattanCircle {
        ManhattanCircle {
            center: self.sensor,
            radius: (self.sensor - self.beacon).manhattan_norm(),
        }
    }
}

fn merge_intervals(intervals: &mut [(i32, i32)]) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return vec![];
    }
    intervals.sort();
    let mut result = vec![];
    let mut current_interval = intervals[0];
    for &next_interval in &intervals[1..] {
        if next_interval.0 <= current_interval.1 {
            current_interval.1 = cmp::max(current_interval.1, next_interval.1);
        } else {
            result.push(current_interval);
            current_interval = next_interval;
        }
    }
    result.push(current_interval);
    result
}

#[derive(Debug, Copy, Clone)]
struct ManhattanCircle {
    center: Coord,
    radius: i32,
}

impl ManhattanCircle {
    fn contains(self, point: Coord) -> bool {
        (self.center - point).manhattan_norm() <= self.radius
    }

    fn barely_misses(self, point: Coord) -> bool {
        (self.center - point).manhattan_norm() == self.radius + 1
    }

    fn intersect_edge(self, other: ManhattanCircle) -> Vec<Coord> {
        let ManhattanCircle {
            center: Coord2(x1, y1),
            radius: r1,
        } = self;
        let ManhattanCircle {
            center: Coord2(x2, y2),
            radius: r2,
        } = other;
        let r1 = r1 + 1;
        let r2 = r2 + 1;
        if (x1 + x2 + y1 + y2 + r1 + r2) % 2 != 0 {
            return vec![];
        }
        let mut result = vec![];
        for r1_sign in [1, -1] {
            for r2_sign in [1, -1] {
                for y1_sign in [1, -1] {
                    // We get these values for x and y by solving the system of equations:
                    //   |x - x1| + |y - y1| = d1  AND
                    //   |x - x2| + |y - y2| = d2
                    let x = (x1 + x2 + r1_sign * r1 + r2_sign * r2 + y1_sign * (y1 - y2)) / 2;
                    let y = y1 + y1_sign * (r1_sign * r1 + x1 - x);
                    let candidate = Coord2(x, y);
                    if self.barely_misses(candidate) && other.barely_misses(candidate) {
                        result.push(candidate);
                    }
                }
            }
        }
        result
    }
}
