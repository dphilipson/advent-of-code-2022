use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use std::cmp;

pub fn solve_part1(input: RawInput) -> usize {
    let heights = Grid::parse_digits(input.as_str());
    heights
        .indices()
        .filter(|&ij| {
            let height = heights[ij];
            get_views(&heights, ij)
                .into_iter()
                .any(|view| view.into_iter().all(|h| h < height))
        })
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    let heights = Grid::parse_digits(input.as_str());
    heights
        .indices()
        .map(|ij| {
            let height = heights[ij];
            get_views(&heights, ij)
                .into_iter()
                .map(|view| {
                    cmp::min(
                        view.len(),
                        view.into_iter().take_while(|&h| h < height).count() + 1,
                    )
                })
                .product()
        })
        .max()
        .unwrap()
}

fn get_views(heights: &Grid<u32>, [i, j]: [usize; 2]) -> [Vec<u32>; 4] {
    [
        (0..i).rev().map(|i2| heights[[i2, j]]).collect(),
        (i + 1..heights.nrows())
            .map(|i2| heights[[i2, j]])
            .collect(),
        (0..j).rev().map(|j2| heights[[i, j2]]).collect(),
        (j + 1..heights.ncols())
            .map(|j2| heights[[i, j2]])
            .collect(),
    ]
}
