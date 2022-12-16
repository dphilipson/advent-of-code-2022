use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re;
use ndarray::{Array2, Array3};
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use std::{cmp, error};

pub fn solve_part1(input: RawInput) -> u32 {
    todo!();
    let valves_by_label = parse_valves(input);
    let mut best_by_state = HashMap::<State, (u32, u32)>::new();
    let mut pending = vec![State {
        location: [b'A', b'A'],
        open_valves: vec![],
        pressure: 0,
        pressure_rate: 0,
        time: 0,
    }];
    while let Some(state) = pending.pop() {
        if let Some(&(best_time, best_pressure)) = best_by_state.get(&state) {
            if state.time >= best_time
                && state.pressure <= best_pressure + (state.time - best_time) * state.pressure_rate
            {
                continue;
            }
            if state.pressure > best_pressure {
                best_by_state.insert(state.clone(), (state.time, state.pressure));
            }
        } else {
            best_by_state.insert(state.clone(), (state.time, state.pressure));
        }
        if state.time == 30 {
            continue;
        }
        let current_location = valves_by_label.get(&state.location).unwrap();
        let next_pressure = state.pressure + state.pressure_rate;
        if current_location.flow_rate > 0 && !state.open_valves.contains(&state.location) {
            let mut open_valves = state.open_valves.clone();
            open_valves.push(state.location);
            open_valves.sort();
            pending.push(State {
                location: state.location,
                open_valves,
                pressure: next_pressure,
                pressure_rate: state.pressure_rate + current_location.flow_rate,
                time: state.time + 1,
            });
        }
        for &next_location in &current_location.connections {
            pending.push(State {
                location: next_location,
                open_valves: state.open_valves.clone(),
                pressure: next_pressure,
                pressure_rate: state.pressure_rate,
                time: state.time + 1,
            })
        }
    }
    best_by_state
        .values()
        .map(|&(_, pressure)| pressure)
        .max()
        .unwrap()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let max_time = 26;
    let valves_by_label = parse_valves(input);
    let mut labels: Vec<_> = valves_by_label.keys().copied().collect();
    labels.sort();
    let index_for_label = |label: Label| labels.iter().position(|&l| l == label).unwrap();
    let mut dists = Array2::from_elem((labels.len(), labels.len()), 10000_u32);
    for valve in valves_by_label.values() {
        let i = index_for_label(valve.label);
        for &connection in &valve.connections {
            let j = index_for_label(connection);
            dists[[i, j]] = 1;
            dists[[j, i]] = 1;
        }
    }
    for i in 0..labels.len() {
        dists[[i, i]] = 0;
    }
    for k in 0..labels.len() {
        for i in 0..labels.len() {
            for j in 0..labels.len() {
                dists[[i, j]] = cmp::min(dists[[i, j]], dists[[i, k]] + dists[[k, j]]);
            }
        }
    }
    let mut useful_labels: Vec<_> = valves_by_label
        .values()
        .filter(|&valve| valve.flow_rate > 0 || valve.label == [b'A', b'A'])
        .map(|valve| valve.label)
        .collect();
    useful_labels.sort();
    let get_dist = |label_idx1: usize, label_idx2: usize| {
        dists[[
            index_for_label(useful_labels[label_idx1]),
            index_for_label(useful_labels[label_idx2]),
        ]]
    };
    let useful_flow_rates = useful_labels
        .iter()
        .map(|label| valves_by_label.get(label).unwrap().flow_rate)
        .collect::<Vec<_>>();
    let pressure_gains_by_subset: Vec<_> = (0..1 << useful_labels.len())
        .map(|bitset| {
            (0..useful_labels.len())
                .map(|i| {
                    if bitset & (1 << i) > 0 {
                        useful_flow_rates[i]
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .collect();
    let mut bests: Array3<Option<u32>> = Array3::from_elem(
        (max_time + 1, useful_labels.len(), 1 << useful_labels.len()),
        None,
    );
    bests[[0, 0, 1]] = Some(0);
    for time in 1..=max_time {
        for label in 0..useful_labels.len() {
            for bitset in 0..(1 << useful_labels.len()) {
                if bitset & (1 << label) == 0 {
                    continue;
                }
                let mut best = None;
                if time > 0 {
                    if let Some(idle_prev) = bests[[time - 1, label, bitset]] {
                        best = Some(idle_prev + pressure_gains_by_subset[bitset]);
                    }
                }
                for label2 in 0..useful_labels.len() {
                    if label == label2 || bitset & (1 << label2) == 0 {
                        continue;
                    }
                    let time_diff = get_dist(label, label2) as usize + 1;
                    if time_diff > time {
                        continue;
                    }
                    if time_diff == time && label2 != 0 {
                        continue;
                    }
                    let prev_bitset = bitset & !(1 << label);
                    if let Some(prev_best) = bests[[time - time_diff, label2, prev_bitset]] {
                        best = cmp::max(
                            best,
                            Some(
                                prev_best
                                    + pressure_gains_by_subset[prev_bitset] * time_diff as u32,
                            ),
                        );
                    }
                }
                bests[[time, label, bitset]] = best;
            }
        }
    }
    let best_by_subset: Vec<_> = (0..1 << useful_labels.len())
        .map(|bitset| {
            (0..useful_labels.len())
                .map(|label| bests[[max_time, label, bitset]])
                .max()
                .unwrap()
        })
        .collect();
    let mut best = None;
    for subset in 0..1 << useful_labels.len() {
        for subset2 in subset + 1..1 << useful_labels.len() {
            if subset & subset2 != 1 {
                continue;
            }
            if let Some(a) = best_by_subset[subset] {
                if let Some(b) = best_by_subset[subset2] {
                    best = cmp::max(best, Some(a + b));
                }
            }
        }
    }
    best.unwrap()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    time: u32,
    location: Label,
    open_valves: Vec<Label>,
    pressure: u32,
    pressure_rate: u32,
}

type Label = [u8; 2];

#[derive(Debug)]
struct Valve {
    label: Label,
    flow_rate: u32,
    connections: Vec<Label>,
}

fn parse_valves(input: RawInput) -> HashMap<Label, Valve> {
    input
        .per_line(|line| line.single::<Valve>())
        .map(|valve| (valve.label, valve))
        .collect()
}

impl FromStr for Valve {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, flow_rate, tunnels): (String, u32, String) = re::parse_with_regex(
            regex!(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$"),
            s,
        )?;
        let connections = tunnels
            .split(", ")
            .map(|s| {
                let bytes = s.as_bytes();
                [bytes[0], bytes[1]]
            })
            .collect();
        let name = name.as_bytes();
        let name = [name[0], name[1]];
        Ok(Self {
            label: name,
            flow_rate,
            connections,
        })
    }
}
