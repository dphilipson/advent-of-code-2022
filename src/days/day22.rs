use ndarray::Array2;
use crate::harness::input::RawInput;
use crate::regex;

pub fn solve_part1(input: RawInput) -> usize {
    let (board, moves) = parse(input);
    let mut state = State::new(board);
    for moove in moves {
        state.apply_move(moove);
    }
    1000 * state.position[0] + 4 * state.position[1] + state.facing.score()

}

pub fn solve_part2(input: RawInput) -> usize {
    input.as_str();
    todo!()
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
                  self.move_foward_once();
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