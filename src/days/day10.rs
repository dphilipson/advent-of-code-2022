use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> i32 {
    let mut total_time: i32 = 0;
    let mut time = 20;
    let mut x: i32 = 1;
    let mut sum = 0;
    input.per_line(|line| line.split_whitespace::<String>())
        .for_each(|parts| {
            time += parts.len();
            total_time += parts.len() as i32;
            if time >= 40 {
                sum += x * (total_time / 20) * 20;
                time -= 40;
            }
            if parts.len() == 2 {
                x += parts[1].parse::<i32>().unwrap();
            }
        });
    sum
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut time = 0;
    let mut x: i32 = 1;
    let mut i = 0;
    let mut executing_add = false;
    let mut chars = vec![];
    let instructions: Vec<_> = input.per_line(|line| line.split_whitespace::<String>()).collect();
    while i < instructions.len() {
        chars.push(if (time - x).abs() <= 1 { '#' } else { '.' });
        let instruction = &instructions[i];
        if instruction.len() == 1 {
            i += 1;
        } else {
            if executing_add {
                x += instruction[1].parse::<i32>().unwrap();
                i += 1;
            }
            executing_add = !executing_add;
        }
        if time == 40 {
            chars.push('\n');
            time = 0;
        }
        time += 1;
    }
    println!("{}", chars.into_iter().collect::<String>());
    0
}
