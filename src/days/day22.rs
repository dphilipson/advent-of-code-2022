use ndarray::Array2;
use crate::harness::input::RawInput;
use crate::regex;

// Extreme mess ahead. Don't expect quality.

pub fn solve_part1(input: RawInput) -> usize {
    todo!();
    let (board, moves) = parse(input);
    let mut state = State::new(board);
    for moove in moves {
        state.apply_move(moove);
    }
    1000 * state.position[0] + 4 * state.position[1] + state.facing.score()

}

pub fn solve_part2(input: RawInput) -> usize {
    let (board, moves) = parse(input);
    let mut state = State::new(board);
    for moove in moves {
        state.apply_move(moove);
    }
    println!("position: {:?}, facing: {:?}", state.position, state.facing);
    1000 * state.position[0] + 4 * state.position[1] + state.facing.score()
}

#[derive(Debug)]
struct State {
    board: Array2<Tile>,
    position: [usize; 2],
    facing: Direction,
}

impl State {
    fn new(board: Array2<Tile>) -> Self {
        Self {
            position: get_start_position(&board),
            board,
            facing: Direction::Right
        }
    }

    fn apply_move(&mut self, moove: Move) {
        match moove {
            Move::Forward(n) => {
              for _ in 0..n {
                  self.move_foward_once_part2();
              }
            },
            Move::TurnLeft => self.facing = self.facing.left(),
            Move::TurnRight => self.facing = self.facing.right(),
        }
    }

    fn move_foward_once(&mut self) {
        let mut position = self.facing.move_from(self.position);
        position = match self.board[position] {
            Tile::Open => position,
            Tile::Blocked => self.position,
            Tile::Warp => {
                let [i, j] = self.position;
                position = match self.facing {
                    Direction::Right => [i, 0],
                    Direction::Up => [self.board.nrows() - 1, j],
                    Direction::Left => [i, self.board.ncols() - 1],
                    Direction::Down => [0, j],
                };
                while self.board[position] == Tile::Warp {
                    position = self.facing.move_from(position);
                }
                if self.board[position] == Tile::Open { position } else { self.position }
            }
        };
        self.position = position
    }

    fn move_foward_once_part2(&mut self) {
        let mut position = self.facing.move_from(self.position);
        position = match self.board[position] {
            Tile::Open => position,
            Tile::Blocked => self.position,
            Tile::Warp => {
                let (facing, position) = warp(self.facing, position);
                if self.board[position] == Tile::Open { self.facing = facing; position } else { self.position }
            }
        };
        self.position = position
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Open,
    Blocked,
    Warp
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    fn move_from(self, [x, y]: [usize; 2]) -> [usize; 2] {
        match self {
            Direction::Right => [x, y + 1],
            Direction::Up => [x - 1, y],
            Direction::Left => [x, y - 1],
            Direction::Down => [x + 1, y],
        }
    }

    fn score(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Left => 2,
            Direction::Down => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

fn parse_board(s: &str) -> Array2<Tile> {
    let lines: Vec<_> = s.lines().map(|s| s.as_bytes()).collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let height = lines.len();
    let mut out = Array2::from_elem((height + 2, width + 2), Tile::Warp);
    for i in 0..height {
        for j in 0..width {
            if let Some(b) = lines[i].get(j) {
                out[[i + 1, j + 1]] = match b {
                    b'.' => Tile::Open,
                    b'#' => Tile::Blocked,
                    b' ' => Tile::Warp,
                    _ => panic!(),
                };
            }
        }
    }
    out
}

fn parse_moves(s: &str) -> Vec<Move> {
    let numbers: Vec<_> = regex!(r"[RL]").split(s)
        .map(|s| Move::Forward(s.parse().unwrap()))
        .collect();
    let turns: Vec<_> = regex!(r"\d+").split(s)
        .filter_map(|s| match s {
            "L" => Some(Move::TurnLeft),
            "R" => Some(Move::TurnRight),
            _ => None,
        })
        .collect();
    let mut out = vec![];
    for i in 0..numbers.len() {
        out.push(numbers[i]);
        out.extend(turns.get(i));
    }
    out
}

fn parse(input: RawInput) -> (Array2<Tile>, Vec<Move>) {
    let (board, moves) = input.as_str().trim_end()
        .split_once("\n\n").unwrap();
    let board = parse_board(board);
    let moves = parse_moves(moves);
    (board, moves)
}

fn get_start_position(board: &Array2<Tile>) -> [usize; 2] {
    for j in 0..board.ncols() {
        if board[[1, j]] == Tile::Open {
            return [1, j];
        }
    }
    panic!("couldn't find start")
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    start: [usize; 2],
    is_horizontal: bool,
    destination_index: usize,
    flip_while_warping: bool,
}

// fn get_edges() -> Vec<Edge> {
//     vec![
//         Edge { // D
//             start: [0, 51],
//             is_horizontal: true,
//             destination_index: todo!(),
//             flip_while_warping: false,
//         },
//         Edge { // C
//             start: [0, 101],
//             is_horizontal: true,
//             destination_index: todo!(),
//             flip_while_warping: false,
//         },
//         Edge { // F
//             start: [100, 1],
//             is_horizontal: true,
//             destination_index: todo!(),
//             flip_while_warping: false,
//         },
//         Edge { // B
//             start: [51, 101],
//             is_horizontal: true,
//             destination_index: todo!(),
//             flip_while_warping: false,
//         },
//         Edge { // G
//             start: [151, 51],
//             is_horizontal: true,
//             destination_index: todo!(),
//             flip_while_warping: false,
//         },
//         Edge { // C
//             start: [201, 1],
//             is_horizontal: true,
//             destination_index: todo!(),
//             flip_while_warping: false,
//         },
//     ]
// }

fn warp(facing: Direction, [i, j]: [usize; 2]) ->(Direction, [usize; 2]) {
    match facing {
        Direction::Right => {
            if j == 151 {
                if i < 51 {
                    (Direction::Left, [(51 - i) + 100, 100]) // A
                } else {
                    panic!("Why here??");
                }
            } else if j == 101 {
                if (51..=100).contains(&i) {
                    (Direction::Up, [50, i + 50]) // B
                } else if (101..=150).contains(&i) {
                    (Direction::Left, [51 - (i - 100), 150]) // A
                } else {
                    panic!("Shouldn't be here");
                }
            } else if j == 51 {
                if i > 150 {
                    (Direction::Up, [150, i - 100]) // G
                } else {
                    panic!("whyyyy");
                }
            } else {
                panic!("This is bad too")
            }
        }
        Direction::Up => {
            if i == 0 {
                if j < 51 {
                    panic!("How here?");
                } else if j < 101 {
                    (Direction::Right, [j + 100, 1]) // D
                } else {
                    (Direction::Up, [200, j - 100]) // C
                }
            } else if i == 100 {
                if j < 51 {
                    (Direction::Right, [j + 50, 51]) // F
                } else {
                    panic!("How here? 2");
                }
            } else {
                panic!("Up in weird spot");
            }
        }
        Direction::Left => {
            if j == 0 {
                if i < 101 {
                    panic!("noooo");
                } else if i < 151 {
                    (Direction::Right, [51 - (i - 100), 51]) // E
                } else {
                    (Direction::Down, [1, i - 100]) // D
                }
            } else if j == 50 {
                if i < 51 {
                    (Direction::Right, [(51 - i) + 100, 1]) // E
                } else if i < 101 {
                    (Direction::Down, [101, i - 50]) // F
                } else {
                    panic!("Yet another please no");
                }
            } else {
                panic!("Not here either")
            }
        }
        Direction::Down => {
            if i == 201 {
                if (1..=50).contains(&j) {
                    (Direction::Down, [1, j + 100]) // C
                } else {
                    panic!("AAAA");
                }
            } else if i == 151 {
                if (51..=100).contains(&j) {
                    (Direction::Left, [j + 100, 50]) // G
                } else {
                    panic!("Halp");
                }
            } else if (101..=150).contains(&j) {
                (Direction::Left, [j - 50, 100]) // B
            } else {
                panic!("Last one!")
            }
        }
    }

}