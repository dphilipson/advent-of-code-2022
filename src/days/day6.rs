use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 4)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 14)
}

// Longer solution optimized for speed.
fn solve(input: RawInput, window_size: usize) -> usize {
    let mut counts = [0_u8; 26];
    let mut unique_values = 0;
    let bytes = input.as_str().as_bytes();
    for i in 0..bytes.len() {
        let added = (bytes[i] - b'a') as usize;
        if counts[added] == 0 {
            unique_values += 1;
        }
        counts[added] += 1;
        if i >= window_size {
            let removed = (bytes[i - window_size] - b'a') as usize;
            counts[removed] -= 1;
            if counts[removed] == 0 {
                unique_values -= 1;
            }
        }
        if unique_values == window_size {
            return i + 1;
        }
    }
    panic!("Did not find window of unique values.");
}

// Concise solution that runs 140x slower.
//
// fn solve(input: RawInput, window_size: usize) -> usize {
//     input
//         .as_str()
//         .as_bytes()
//         .windows(window_size)
//         .enumerate()
//         .find(|&(_, bytes)| bytes.iter().collect::<HashSet<_>>().len() == window_size)
//         .unwrap()
//         .0
//         + window_size
// }
