use crate::harness::input::RawInput;
use crate::regex;
use ndarray::Array2;
use Direction::{Down, Left, Right, Up};
use Move::{Forward, TurnLeft, TurnRight};
use Tile::{Blocked, Open, Warp};

static PORTALS: [Portal; 7] = [
    Portal::new(Edge::new([0, 1], Up), Edge::new([3, 0], Left), false),
    Portal::new(Edge::new([0, 2], Up), Edge::new([3, 0], Down), false),
    Portal::new(Edge::new([0, 2], Right), Edge::new([2, 1], Right), true),
    Portal::new(Edge::new([0, 2], Down), Edge::new([1, 1], Right), false),
    Portal::new(Edge::new([2, 1], Down), Edge::new([3, 0], Right), false),
    Portal::new(Edge::new([2, 0], Left), Edge::new([0, 1], Left), true),
    Portal::new(Edge::new([2, 0], Up), Edge::new([1, 1], Left), false),
];

const EDGE_LENGTH: usize = 50;


pub fn solve_part1(input: RawInput) -> usize {
    solve(input, false)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, true)
}

fn solve(input: RawInput, is_part2: bool) -> usize {
    let (board, moves) = parse(input);
    let mut state = State::new(board, is_part2);
    for moove in moves {
        state.apply_move(moove);
    }
    state.position.score()
}

#[derive(Debug)]
struct State {
    board: Array2<Tile>,
    position: Position,
    is_part_2: bool,
}

impl State {
    fn new(board: Array2<Tile>, is_part_2: bool) -> Self {
        Self {
            position: get_start_position(&board),
            board,
            is_part_2,
        }
    }

    fn apply_move(&mut self, moove: Move) {
        match moove {
            Forward(n) => {
                for _ in 0..n {
                    self.move_foward_once();
                }
            }
            TurnLeft => self.position.facing = self.position.facing.left(),
            TurnRight => self.position.facing = self.position.facing.right(),
        }
    }

    fn move_foward_once(&mut self) {
        let mut position = self.position.forward();
        if self.board[position.location] == Warp {
            position = self.warp(position);
        }
        self.position = match self.board[position.location] {
            Open => position,
            Blocked => self.position,
            Warp => panic!("shouldn't be on warp after warping"),
        };
    }

    fn warp(&self, position: Position) -> Position {
        if self.is_part_2 {
            return position.warp_with_portals();
        }
        let [i, j] = position.location;
        let location = match position.facing {
            Right => [i, (0..).position(|j| self.board[[i, j]] != Warp).unwrap()],
            Down => [(0..).position(|i| self.board[[i, j]] != Warp).unwrap(), j],
            Left => [
                i,
                (0..self.board.ncols())
                    .rposition(|j| self.board[[i, j]] != Warp)
                    .unwrap(),
            ],
            Up => [
                (0..self.board.nrows())
                    .rposition(|i| self.board[[i, j]] != Warp)
                    .unwrap(),
                j,
            ],
        };
        Position::new(location, position.facing)
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    location: [usize; 2],
    facing: Direction,
}

impl Position {
    fn new(location: [usize; 2], facing: Direction) -> Self {
        Self { location, facing }
    }

    fn forward(self) -> Self {
        let [i, j] = self.location;
        let location = match self.facing {
            Right => [i, j + 1],
            Down => [i + 1, j],
            Left => [i, j - 1],
            Up => [i - 1, j],
        };
        Self { location, ..self }
    }

    fn warp_with_portals(self) -> Self {
        PORTALS
            .iter()
            .filter_map(|portal| portal.try_warp(self))
            .next()
            .expect("should find matching portal")
    }

    fn score(self) -> usize {
        let [i, j] = self.location;
        1000 * i + 4 * j + self.facing.score()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Open,
    Blocked,
    Warp,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn right(self) -> Self {
        Self::from_ordinal(self as u8 + 1)
    }

    fn reverse(self) -> Self {
        Self::from_ordinal(self as u8 + 2)
    }

    fn left(self) -> Self {
        Self::from_ordinal(self as u8 + 3)
    }

    fn from_ordinal(n: u8) -> Self {
        match n % 4 {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!(),
        }
    }

    fn score(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Copy, Clone)]
struct Portal {
    edge1: Edge,
    edge2: Edge,
    causes_flip: bool,
}

impl Portal {
    const fn new(edge1: Edge, edge2: Edge, causes_flip: bool) -> Self {
        Self {
            edge1,
            edge2,
            causes_flip,
        }
    }

    fn try_warp(self, position: Position) -> Option<Position> {
        if let Some(mut distance) = self.edge1.distance_along_edge(position) {
            if self.causes_flip {
                distance = EDGE_LENGTH - 1 - distance;
            }
            Some(self.edge2.position_along_edge(distance))
        } else if let Some(mut distance) = self.edge2.distance_along_edge(position) {
            if self.causes_flip {
                distance = EDGE_LENGTH - 1 - distance;
            }
            Some(self.edge1.position_along_edge(distance))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    sector: [usize; 2],
    facing: Direction,
}

impl Edge {
    const fn new(sector: [usize; 2], facing: Direction) -> Self {
        Self { sector, facing }
    }

    fn distance_along_edge(self, position: Position) -> Option<usize> {
        if position.facing != self.facing {
            return None;
        }
        let [i, j] = position.location;
        match self.facing {
            Right => Some(i.wrapping_sub(self.min_i()))
                .filter(|&d| d < EDGE_LENGTH && j == self.max_j() + 1),
            Down => Some(j.wrapping_sub(self.min_j()))
                .filter(|&d| d < EDGE_LENGTH && i == self.max_i() + 1),
            Left => Some(i.wrapping_sub(self.min_i()))
                .filter(|&d| d < EDGE_LENGTH && j == self.min_j() - 1),
            Up => Some(j.wrapping_sub(self.min_j()))
                .filter(|&d| d < EDGE_LENGTH && i == self.min_i() - 1),
        }
    }

    fn position_along_edge(self, distance: usize) -> Position {
        let location = match self.facing {
            Right => [self.min_i() + distance, self.max_j()],
            Down => [self.max_i(), self.min_j() + distance],
            Left => [self.min_i() + distance, self.min_j()],
            Up => [self.min_i(), self.min_j() + distance],
        };
        Position::new(location, self.facing.reverse())
    }

    fn min_i(self) -> usize {
        self.sector[0] * EDGE_LENGTH + 1
    }

    fn min_j(self) -> usize {
        self.sector[1] * EDGE_LENGTH + 1
    }

    fn max_i(self) -> usize {
        self.min_i() + EDGE_LENGTH - 1
    }

    fn max_j(self) -> usize {
        self.min_j() + EDGE_LENGTH - 1
    }
}

fn parse_board(s: &str) -> Array2<Tile> {
    let lines: Vec<_> = s.lines().map(|s| s.as_bytes()).collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let height = lines.len();
    let mut out = Array2::from_elem((height + 2, width + 2), Warp);
    for i in 0..height {
        for j in 0..width {
            if let Some(b) = lines[i].get(j) {
                out[[i + 1, j + 1]] = match b {
                    b'.' => Open,
                    b'#' => Blocked,
                    b' ' => Warp,
                    _ => panic!(),
                };
            }
        }
    }
    out
}

fn parse_moves(s: &str) -> Vec<Move> {
    let numbers: Vec<_> = regex!(r"[RL]")
        .split(s)
        .map(|s| Forward(s.parse().unwrap()))
        .collect();
    let turns: Vec<_> = regex!(r"\d+")
        .split(s)
        .filter_map(|s| match s {
            "L" => Some(TurnLeft),
            "R" => Some(TurnRight),
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
    let (board, moves) = input.as_str().trim_end().split_once("\n\n").unwrap();
    let board = parse_board(board);
    let moves = parse_moves(moves);
    (board, moves)
}

fn get_start_position(board: &Array2<Tile>) -> Position {
    for j in 0..board.ncols() {
        if board[[1, j]] == Open {
            return Position::new([1, j], Right);
        }
    }
    panic!("couldn't find start")
}