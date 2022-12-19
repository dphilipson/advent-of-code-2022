use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re;
use std::cmp;

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .per_line(|line| parse_blueprint_costs(line.as_str()))
        .enumerate()
        .map(|(i, costs)| (i as u32 + 1) * get_max_geodes(costs, 24))
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    input
        .per_line(|line| parse_blueprint_costs(line.as_str()))
        .take(3)
        .map(|costs| get_max_geodes(costs, 32))
        .product()
}

fn get_max_geodes(costs: [[u32; 4]; 4], time_limit: u32) -> u32 {
    let max_robots = get_max_robots(costs);
    let mut pending = vec![State {
        time: 0,
        robots: [1, 0, 0, 0],
        resources: [0; 4],
    }];
    let mut best_geodes = 0;
    while let Some(state) = pending.pop() {
        if state.get_best_possible_geodes(time_limit) <= best_geodes {
            continue;
        }
        // Consider building a robot of each type. Try the more advanced robots
        // first so that we get branches with more geodes sooner so they can be
        // used to prune other branches.
        for robot_type in (0..4).rev() {
            if state.robots[robot_type] == max_robots[robot_type] {
                continue;
            }
            if let Some(wait_time) = state.get_wait_time(costs[robot_type]) {
                let time = state.time + wait_time;
                if time > time_limit {
                    continue;
                }
                let mut robots = state.robots;
                robots[robot_type] += 1;
                let mut resources = state.resources;
                for i in 0..4 {
                    resources[i] += wait_time * state.robots[i];
                    resources[i] -= costs[robot_type][i]
                }
                pending.push(State {
                    time,
                    robots,
                    resources,
                });
                best_geodes = cmp::max(best_geodes, resources[3] + robots[3] * (time_limit - time));
            }
        }
    }
    best_geodes
}

fn get_max_robots(costs: [[u32; 4]; 4]) -> [u32; 4] {
    let mut out = [0; 4];
    for i in 0..3 {
        out[i] = (0..4).map(|j| costs[j][i]).max().unwrap();
    }
    out[3] = u32::MAX;
    out
}

#[derive(Copy, Clone, Debug)]
struct State {
    time: u32,
    robots: [u32; 4],
    resources: [u32; 4],
}

impl State {
    fn get_wait_time(self, costs: [u32; 4]) -> Option<u32> {
        let mut wait_time = 0;
        for i in 0..4 {
            let missing = costs[i].saturating_sub(self.resources[i]);
            if missing == 0 {
                continue;
            }
            if self.robots[i] == 0 {
                return None;
            }
            wait_time = cmp::max(wait_time, div_round_up(missing, self.robots[i]));
        }
        Some(wait_time + 1)
    }

    fn get_best_possible_geodes(self, time_limit: u32) -> u32 {
        let remaining_time = time_limit - self.time;
        self.resources[3]
            + self.robots[3] * remaining_time
            + remaining_time * (remaining_time + 1) / 2
    }
}

/// Returns an array of costs, where `costs[i][j]` is the amount of resource `j`
/// required to build a robot of type `i`.
fn parse_blueprint_costs(s: &str) -> [[u32; 4]; 4] {
    let (ore_for_ore, ore_for_clay, ore_for_obsidian, clay_for_obsidian, ore_for_geode, obsidian_for_geode) : (u32, u32, u32, u32, u32, u32) = re::parse_with_regex(
        regex!(
                r"^Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"
            ),
        s
    ).unwrap();
    let mut out: [[u32; 4]; 4] = Default::default();
    out[0][0] = ore_for_ore;
    out[1][0] = ore_for_clay;
    out[2][0] = ore_for_obsidian;
    out[2][1] = clay_for_obsidian;
    out[3][0] = ore_for_geode;
    out[3][2] = obsidian_for_geode;
    out
}

fn div_round_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}
