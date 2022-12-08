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
                .any(|mut view| view.all(|h| h < height))
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
                .map(|view| cmp::min(view.len(), view.take_while(|&h| h < height).count() + 1))
                .product()
        })
        .max()
        .unwrap()
}

fn get_views(
    heights: &Grid<u32>,
    [i, j]: [usize; 2],
) -> [Box<dyn ExactSizeIterator<Item = u32> + '_>; 4] {
    [
        Box::new((0..i).rev().map(move |i2| heights[[i2, j]])),
        Box::new((i + 1..heights.nrows()).map(move |i2| heights[[i2, j]])),
        Box::new((0..j).rev().map(move |j2| heights[[i, j2]])),
        Box::new((j + 1..heights.ncols()).map(move |j2| heights[[i, j2]])),
    ]
}
