use crate::harness::input::RawInput;
use arrayvec::ArrayVec;
use ndarray::Array3;

pub fn solve_part1(input: RawInput) -> usize {
    let cubes = parse_cubes_into_3d_grid(input);
    let bounds = cubes.dim();
    let mut surface_area = 0;
    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            for z in 0..bounds.2 {
                let loc = [x, y, z];
                if cubes[loc] {
                    let neighbor_count = get_orthogonal_neighbors(loc, bounds)
                        .into_iter()
                        .filter(|&neighbor| cubes[neighbor])
                        .count();
                    surface_area += 6 - neighbor_count;
                }
            }
        }
    }
    surface_area
}

pub fn solve_part2(input: RawInput) -> usize {
    let cubes = parse_cubes_into_3d_grid(input);
    let mut seen_cubes = cubes.clone();
    let bounds = seen_cubes.dim();
    let mut pending = vec![[0, 0, 0]];
    let mut surface_area = 0;
    while let Some(current) = pending.pop() {
        if seen_cubes[current] {
            continue;
        }
        seen_cubes[current] = true;
        for neighbor in get_orthogonal_neighbors(current, bounds) {
            if cubes[neighbor] {
                surface_area += 1;
            }
            if !seen_cubes[neighbor] {
                pending.push(neighbor);
            }
        }
    }
    surface_area
}

fn parse_cubes_into_3d_grid(input: RawInput) -> Array3<bool> {
    let cubes: Vec<[usize; 3]> = input
        .per_line(|line| {
            let parts = line.split(",");
            [parts[0], parts[1], parts[2]]
        })
        .collect();
    let [x_max, y_max, z_max] = [0, 1, 2].map(|i| cubes.iter().map(|c| c[i]).max().unwrap());
    let mut out = Array3::default((x_max + 2, y_max + 2, z_max + 2));
    for cube in cubes {
        out[cube] = true;
    }
    out
}

fn get_orthogonal_neighbors(
    [x, y, z]: [usize; 3],
    (x_max, y_max, z_max): (usize, usize, usize),
) -> ArrayVec<[usize; 3], 6> {
    [
        [x.wrapping_sub(1), y, z],
        [x + 1, y, z],
        [x, y.wrapping_sub(1), z],
        [x, y + 1, z],
        [x, y, z.wrapping_sub(1)],
        [x, y, z + 1],
    ]
    .into_iter()
    .filter(|&[x, y, z]| x < x_max && y < y_max && z < z_max)
    .collect()
}
