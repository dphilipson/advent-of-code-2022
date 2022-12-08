use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use std::cmp;

pub fn solve_part1(input: RawInput) -> usize {
    let heights = Grid::parse_digits(input.as_str());
    let mut views = initialize_views(&heights);
    heights
        .indices()
        .filter(|&ij| {
            let height = heights[ij];
            fill_views(&mut views, &heights, ij);
            views.iter().any(|view| view.iter().all(|&h| h < height))
        })
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    let heights = Grid::parse_digits(input.as_str());
    let mut views = initialize_views(&heights);
    heights
        .indices()
        .map(|ij| {
            let height = heights[ij];
            fill_views(&mut views, &heights, ij);
            views
                .iter()
                .map(|view| {
                    cmp::min(
                        view.len(),
                        view.iter().take_while(|&&h| h < height).count() + 1,
                    )
                })
                .product()
        })
        .max()
        .unwrap()
}

fn initialize_views(heights: &Grid<u32>) -> [Vec<u32>; 4] {
    [
        Vec::with_capacity(heights.nrows() - 1),
        Vec::with_capacity(heights.nrows() - 1),
        Vec::with_capacity(heights.ncols() - 1),
        Vec::with_capacity(heights.ncols() - 1),
    ]
}

fn fill_views(views: &mut [Vec<u32>; 4], heights: &Grid<u32>, [i, j]: [usize; 2]) {
    views.iter_mut().for_each(|v| v.clear());
    views[0].extend((0..i).rev().map(move |i2| heights[[i2, j]]));
    views[1].extend((i + 1..heights.nrows()).map(move |i2| heights[[i2, j]]));
    views[2].extend((0..j).rev().map(move |j2| heights[[i, j2]]));
    views[3].extend((j + 1..heights.ncols()).map(move |j2| heights[[i, j2]]));
}
