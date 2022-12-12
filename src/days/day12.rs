use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use crate::util::search::bfs;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, b'S')
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, b'a')
}

fn solve(input: RawInput, goal: u8) -> usize {
    let heights = Grid::parse_bytes(input.as_str());
    let result = bfs::search(
        heights.indices().find(|&ij| heights[ij] == b'E').unwrap(),
        |&ij| {
            heights
                .orthogonal_neighbors(ij)
                .filter(|&ij2| get_height(heights[ij]) <= get_height(heights[ij2]) + 1)
                .collect::<Vec<_>>()
        },
        |&ij| heights[ij] == goal,
    );
    result.goal_state().unwrap().distance
}

fn get_height(h: u8) -> u8 {
    match h {
        b'S' => b'a',
        b'E' => b'z',
        _ => h,
    }
}
