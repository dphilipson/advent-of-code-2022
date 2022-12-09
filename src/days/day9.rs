use crate::harness::input::RawInput;
use crate::util::coords::Coord2;
use std::collections::HashSet;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 2)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 10)
}

fn solve(input: RawInput, length: usize) -> usize {
    let mut tail_positions = HashSet::new();
    let mut snake = Snake::new(length);
    input
        .as_str()
        .lines()
        .map(parse_movement)
        .for_each(|(movement, distance)| {
            for _ in 0..distance {
                snake.move_head(movement);
                tail_positions.insert(snake.tail());
            }
        });
    tail_positions.len()
}

type Coord = Coord2<i32>;

fn parse_movement(line: &str) -> (Coord, u8) {
    let (direction, distance) = line.split_once(' ').unwrap();
    let distance = distance.parse().unwrap();
    let movement = match direction {
        "R" => Coord2(1, 0),
        "U" => Coord2(0, 1),
        "L" => Coord2(-1, 0),
        "D" => Coord2(0, -1),
        _ => panic!(),
    };
    (movement, distance)
}

#[derive(Debug)]
struct Snake(Vec<Coord>);

impl Snake {
    fn new(length: usize) -> Self {
        Snake(vec![Coord::default(); length])
    }

    fn move_head(&mut self, movement: Coord) {
        self.0[0] += movement;
        for i in 1..self.0.len() {
            let delta = self.0[i - 1] - self.0[i];
            if delta.0.abs() > 1 || delta.1.abs() > 1 {
                self.0[i].0 += delta.0.signum();
                self.0[i].1 += delta.1.signum();
            }
        }
    }

    fn tail(&self) -> Coord {
        *self.0.last().unwrap()
    }
}
