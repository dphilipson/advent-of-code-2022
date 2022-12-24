use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use crate::util::search::bfs;
use std::error;
use std::str::FromStr;

const START_LOCATION: Index = [0, 1];

pub fn solve_part1(input: RawInput) -> usize {
    let blizzards_at_times = load_blizzards_at_times(input);
    get_shortest_time(
        &blizzards_at_times,
        START_LOCATION,
        blizzards_at_times[0].end_location(),
        0,
    )
}

pub fn solve_part2(input: RawInput) -> usize {
    let blizzards_at_times = load_blizzards_at_times(input);
    let end_location = blizzards_at_times[0].end_location();
    let time = get_shortest_time(&blizzards_at_times, START_LOCATION, end_location, 0);
    let time = get_shortest_time(&blizzards_at_times, end_location, START_LOCATION, time);
    get_shortest_time(&blizzards_at_times, START_LOCATION, end_location, time)
}

fn get_shortest_time(
    blizzards_at_times: &[Blizzards],
    start_loc: Index,
    end_loc: Index,
    start_time: usize,
) -> usize {
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    struct State {
        time: usize,
        location: Index,
    }

    let grid = &blizzards_at_times[0].right_blizzards;
    let search_result = bfs::search(
        State {
            time: start_time,
            location: start_loc,
        },
        |s| {
            let time = s.time + 1;
            let mut locs: Vec<_> = grid.orthogonal_neighbors(s.location).collect();
            locs.push(s.location);
            locs.into_iter()
                .filter(|&ij| blizzards_at_times[time].is_clear(ij))
                .map(|ij| State { time, location: ij })
                .collect::<Vec<State>>()
        },
        |s| s.location == end_loc,
    );
    search_result.goal_state().unwrap().state.time
}

type Index = [usize; 2];

#[derive(Debug)]
struct Blizzards {
    right_blizzards: Grid<bool>,
    up_blizzards: Grid<bool>,
    left_blizzards: Grid<bool>,
    down_blizzards: Grid<bool>,
}

impl FromStr for Blizzards {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::parse_bytes(s);
        let right_blizzards = grid.map(|&b| b == b'>');
        let up_blizzards = grid.map(|&b| b == b'^');
        let left_blizzards = grid.map(|&b| b == b'<');
        let down_blizzards = grid.map(|&b| b == b'v');
        Ok(Self {
            right_blizzards,
            up_blizzards,
            left_blizzards,
            down_blizzards,
        })
    }
}

impl Blizzards {
    fn is_clear(&self, ij: Index) -> bool {
        let [i, j] = ij;
        (ij == START_LOCATION
            || ij == self.end_location()
            || ((1..self.nrows() - 1).contains(&i) && (1..self.ncols() - 1).contains(&j)))
            && !(self.right_blizzards[ij]
                || self.up_blizzards[ij]
                || self.left_blizzards[ij]
                || self.down_blizzards[ij])
    }

    fn next(&self) -> Self {
        let mut right_blizzards = self.right_blizzards.map(|_| false);
        let mut up_blizzards = right_blizzards.clone();
        let mut left_blizzards = right_blizzards.clone();
        let mut down_blizzards = right_blizzards.clone();

        for [i, j] in self.right_blizzards.indices() {
            if self.right_blizzards[[i, j]] {
                let next_j = (j % (self.ncols() - 2)) + 1;
                right_blizzards[[i, next_j]] = true;
            }
        }
        for [i, j] in self.up_blizzards.indices() {
            if self.up_blizzards[[i, j]] {
                let next_i = ((i + self.nrows() - 4) % (self.nrows() - 2)) + 1;
                up_blizzards[[next_i, j]] = true;
            }
        }
        for [i, j] in self.left_blizzards.indices() {
            if self.left_blizzards[[i, j]] {
                let next_j = ((j + self.ncols() - 4) % (self.ncols() - 2)) + 1;
                left_blizzards[[i, next_j]] = true;
            }
        }
        for [i, j] in self.down_blizzards.indices() {
            if self.down_blizzards[[i, j]] {
                let next_i = (i % (self.nrows() - 2)) + 1;
                down_blizzards[[next_i, j]] = true;
            }
        }
        Self {
            right_blizzards,
            up_blizzards,
            left_blizzards,
            down_blizzards,
        }
    }

    fn nrows(&self) -> usize {
        self.right_blizzards.nrows()
    }

    fn ncols(&self) -> usize {
        self.right_blizzards.ncols()
    }

    fn end_location(&self) -> Index {
        [self.nrows() - 1, self.ncols() - 2]
    }
}

fn load_blizzards_at_times(input: RawInput) -> Vec<Blizzards> {
    let mut blizzards: Blizzards = input.as_str().parse().unwrap();
    let mut blizzards_at_time: Vec<Blizzards> = vec![];
    for _ in 0..1000 {
        let next_blizzards = blizzards.next();
        blizzards_at_time.push(blizzards);
        blizzards = next_blizzards;
    }
    blizzards_at_time
}
