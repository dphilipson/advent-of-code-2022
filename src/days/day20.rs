use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> i64 {
    solve(input, 1, 1)
}

pub fn solve_part2(input: RawInput) -> i64 {
   solve(input, 811589153, 10)
}

fn solve(input: RawInput, multiplier: i64, num_rounds: usize) -> i64 {
    let mut nums: Vec<_> = input.per_line(|line| line.single::<i64>())
        .map(|n| multiplier * n)
        .collect();
    let mut indices: Vec<_> = (0..nums.len()).collect();
    for _ in 0..num_rounds {
        for i in 0..nums.len() {
            let position = indices.iter().position(|&j| i == j).unwrap();
            let amount = nums[position];
            shift(&mut nums, position, amount);
            shift(&mut indices, position, amount);
        }
    }
    let zero_pos = nums.iter().position(|&i| i == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|&i| nums[(zero_pos + i) % nums.len()])
        .sum()
}

fn shift<T: Copy>(nums: &mut [T], i: usize, amount: i64) {
    let hops = pos_mod(amount, nums.len() as i64 - 1) as usize;
    let dest = (i + hops) % (nums.len() - 1);
    let value = nums[i];
    if dest < i {
        nums.copy_within(dest..i, dest + 1);
        nums[dest] = value;
    } else {
        nums.copy_within(i + 1..=dest, i);
        nums[dest] = value;
    }
}

/// Like %, except it always returns a positive number.
fn pos_mod(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}
