use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re;
use crate::util::re::Str;
use arrayvec::ArrayString;
use ndarray::{Array2, Array3};
use std::str::FromStr;
use std::{cmp, error};

pub fn solve_part1(input: RawInput) -> u32 {
    let layout = parse_layout(input);
    get_best_scores_by_bitset(&layout, 30)
        .into_iter()
        .max()
        .unwrap()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let layout = parse_layout(input);
    let scores_by_bitset = get_best_scores_by_bitset(&layout, 26);
    let bitset_max = 1 << layout.n_valves();
    let mut best = 0;
    for i in 0..bitset_max {
        for j in i + 1..bitset_max {
            if i & j == 0 {
                let score = scores_by_bitset[i] + scores_by_bitset[j];
                best = cmp::max(best, score);
            }
        }
    }
    best
}

fn parse_layout(input: RawInput) -> Layout {
    let valves: Vec<_> = input.per_line(|line| line.single::<Valve>()).collect();
    Layout::from_valves(&valves)
}

fn get_best_scores_by_bitset(layout: &Layout, time_limit: usize) -> Vec<u32> {
    // bests[[time, location, bitset]] represents the best possible score that
    // can be obtained by being at `location` at `time` with the valves in
    // `bitset` opened, and then waiting there until time runs out.
    let mut bests =
        Array3::<u32>::default((time_limit + 1, layout.n_valves(), 1 << layout.n_valves()));
    let mut bests_by_bitset = vec![0_u32; 1 << layout.n_valves()];
    let mut pending_by_time: Vec<Vec<[usize; 2]>> = vec![vec![]; time_limit];
    for (i, valve) in layout.valves.iter().enumerate() {
        let time = valve.distance_from_start + 1;
        let bitset = 1 << i;
        let score = valve.flow_rate * (time_limit - time) as u32;
        bests[[time, i, bitset]] = score;
        bests_by_bitset[bitset] = score;
        pending_by_time[time].push([i, bitset]);
    }
    for time in 0..time_limit - 1 {
        for pending_index in 0..pending_by_time[time].len() {
            // Iterate over index rather than elements to avoid double-borrow.
            let [i, bitset] = pending_by_time[time][pending_index];
            for j in 0..layout.n_valves() {
                if bitset & 1 << j != 0 {
                    continue;
                }
                let next_time = time + layout.distance(i, j) + 1;
                if next_time >= time_limit {
                    continue;
                }
                let next_bitset = bitset | 1 << j;
                let score = bests[[time, i, bitset]]
                    + layout.flow_rate(j) * (time_limit - next_time) as u32;
                let next = [next_time, j, next_bitset];
                bests[next] = cmp::max(bests[next], score);
                bests_by_bitset[next_bitset] = cmp::max(bests_by_bitset[next_bitset], score);
                pending_by_time[next_time].push([j, next_bitset]);
            }
        }
    }
    bests_by_bitset
}

type Label = ArrayString<2>;

#[derive(Debug)]
struct Valve {
    label: Label,
    flow_rate: u32,
    connections: Vec<Label>,
}

impl FromStr for Valve {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, flow_rate, Str(tunnels)): (Label, u32, Str) = re::parse_with_regex(
            regex!(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)$"),
            s,
        )?;
        let connections = tunnels
            .split(", ")
            .map(|s| Label::from(s).unwrap())
            .collect();
        Ok(Self {
            label,
            flow_rate,
            connections,
        })
    }
}

/// `UsefulValves` are valves with nonzero flow. Not useful outside of a `Layout` struct.
#[derive(Debug)]
struct UsefulValve {
    flow_rate: u32,
    /// Index of this valve when reading from a table of distances.
    distance_index: usize,
    distance_from_start: usize,
}

#[derive(Debug)]
struct Layout {
    valves: Vec<UsefulValve>,
    distances: Array2<usize>,
}

impl Layout {
    fn from_valves(valves: &[Valve]) -> Self {
        let distances = compute_distances(valves);
        let start_label = Label::from("AA").unwrap();
        let original_index_of_start = valves.iter().position(|v| v.label == start_label).unwrap();
        let useful_valves_with_original_index: Vec<_> = valves
            .iter()
            .enumerate()
            .filter(|(_, v)| v.flow_rate > 0)
            .collect();
        let valves: Vec<_> = useful_valves_with_original_index
            .iter()
            .map(|&(i, valve)| UsefulValve {
                flow_rate: valve.flow_rate,
                distance_index: i,
                distance_from_start: distances[[i, original_index_of_start]],
            })
            .collect();
        Self { valves, distances }
    }

    fn n_valves(&self) -> usize {
        self.valves.len()
    }

    fn flow_rate(&self, i: usize) -> u32 {
        self.valves[i].flow_rate
    }

    fn distance(&self, i: usize, j: usize) -> usize {
        self.distances[[self.valves[i].distance_index, self.valves[j].distance_index]]
    }
}

fn compute_distances(valves: &[Valve]) -> Array2<usize> {
    // Floyd-Warshall algorithm for all-pairs shortest paths
    let index_for_label = |l: Label| valves.iter().position(|v| v.label == l).unwrap();
    let mut distances = Array2::<usize>::from_elem((valves.len(), valves.len()), usize::MAX / 2);
    for i in 0..valves.len() {
        distances[[i, i]] = 0;
    }
    for (i, valve) in valves.iter().enumerate() {
        for &connection in &valve.connections {
            let j = index_for_label(connection);
            distances[[i, j]] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distances[[i, j]] =
                    cmp::min(distances[[i, j]], distances[[i, k]] + distances[[k, j]])
            }
        }
    }
    distances
}
