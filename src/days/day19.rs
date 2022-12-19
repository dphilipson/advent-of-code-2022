use std::error;
use std::str::FromStr;
use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re;
use crate::util::search::bfs;

pub fn solve_part1(input: RawInput) -> usize {
    input.per_line(|line| line.single::<Blueprint>())
        .enumerate()
        .map(|(i, blueprint)| {
            let geodes = get_max_geodes(&blueprint, 5);
            println!("Part 1: {i} -> {geodes}");
            (i + 1) * geodes
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    input.per_line(|line| line.single::<Blueprint>())
        .take(3)
        .enumerate()
        .map(|(i, blueprint)| {
            println!("{i}");
            get_max_geodes(&blueprint, 32)
        })
        .product()
}

fn get_max_geodes(blueprint: &Blueprint, time_limit: usize) -> usize {
    let search_result = bfs::search(
        State { time: 0, robot_counts: [1, 0, 0, 0], resource_counts: [0; 4], declined_builds: [false; 4]},
        |state| {
            if state.time == time_limit {
                return vec![];
            }
            let mut max_costs = [0; 4];
            for i in 0..4 {
                max_costs[i] = (0..4).map(|j| blueprint.0[j][i]).max().unwrap();
            }
            let mut out = vec![];
            let mut resource_counts = state.resource_counts;
            for i in 0..4 {
                resource_counts[i] += state.robot_counts[i];
            }
            let mut declined_builds = state.declined_builds;
            for resource in 0..4 {
                if resource < 3 && state.robot_counts[resource] == max_costs[resource] {
                    // Don't build if already as many robots as highest cost.
                    continue;
                }
                // Consider building a robot of type `resource`.
                if state.declined_builds[resource] {
                    // We already skipped building this. Doesn't make sense to build it now.
                    continue;
                }
                let costs = blueprint.0[resource];
                if (0..4).all(|i| state.resource_counts[i] >= costs[i]) {
                    // We can afford to build a resource i.
                    let mut robot_counts = state.robot_counts;
                    robot_counts[resource] += 1;
                    let mut resource_counts = resource_counts;
                    for i in 0..4 {
                        resource_counts[i] -= costs[i];
                    }
                    out.push(State {
                        time: state.time + 1,
                        robot_counts,
                        resource_counts,
                        declined_builds: [false; 4]
                    });
                    declined_builds[resource] = true;
                }
            }
            out.push(State {
                time: state.time + 1,
                robot_counts: state.robot_counts,
                resource_counts,
                declined_builds,
            });
            out
        },
        |_| false
    );
    search_result.seen_states.iter()
        .map(|state| state.state.resource_counts[3])
        .max()
        .unwrap()
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    time: usize,
    robot_counts: [usize; 4],
    resource_counts: [usize; 4],
    declined_builds: [bool; 4],
}


/// Blueprint.0[i] is costs to build i robot.
/// So Blueprint.0[i][j] is how much j needed to build i robot.
#[derive(Debug)]
struct Blueprint([[usize; 4]; 4]);

impl FromStr for Blueprint {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ore_for_ore, ore_for_clay, ore_for_obsidian, clay_for_obsidian, ore_for_geode, obsidian_for_geode) : (usize, usize, usize, usize, usize, usize) = re::parse_with_regex(
            regex!(
                r"^Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"
            ),
            s
        )?;
        let mut out: [[usize; 4]; 4] = Default::default();
        out[0][0] = ore_for_ore;
        out[1][0] = ore_for_clay;
        out[2][0] = ore_for_obsidian;
        out[2][1] = clay_for_obsidian;
        out[3][0] = ore_for_geode;
        out[3][2] = obsidian_for_geode;
        Ok(Self(out))
    }
}