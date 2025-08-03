use advent_of_code_2024::aoc;
use advent_of_code_2024::direction::DIRECTIONS;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SquareState {
    Open,
    Wall,
}

impl SquareState {
    fn as_char(&self) -> char {
        match self {
            SquareState::Open => '.',
            SquareState::Wall => '#',
        }
    }
}
#[derive(Clone, Debug)]
struct Track {
    map: Vec<Vec<SquareState>>,
    start: Position,
    end: Position,
}

struct State {
    row: usize,
    col: usize,
    cheat: bool,
}

type Position = (usize, usize);

impl Track {
    fn from_input(input: &str) -> Track {
        let map: Vec<Vec<SquareState>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => SquareState::Wall,
                        _ => SquareState::Open,
                    })
                    .collect()
            })
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (i, j)
                };
                if c == 'E' {
                    end = (i, j)
                };
            }
        }

        Track { map, start, end }
    }

    fn display(&self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                print!("{}", self.map[row][col].as_char());
            }
            println!();
        }
    }

    // bgf from end to start, in order to find dist from end for all nodes
    fn find_path(&self) -> HashMap<Position, i64> {
        let mut path: HashMap<Position, i64> = HashMap::new();
        let mut curr: Position = self.start;
        let mut prev: Position = self.start;
        let mut dist = 1;
        path.insert(curr, 0);

        'outer: while curr != self.end {
            for dir in DIRECTIONS {
                let next_pos = (dir + curr).unwrap();
                if next_pos != prev && self.map[next_pos.0][next_pos.1] == SquareState::Open {
                    path.insert(next_pos, dist);
                    dist += 1;
                    prev = curr;
                    curr = next_pos;
                    continue 'outer;
                }
            }
            unreachable!();
        }

        path
    }

    fn count_cheats(&self, path: &HashMap<Position, i64>, limit: i64, cheat_time: i64) -> i64 {
        let mut count = 0;
        for (&pos, _) in path.iter() {
            count += self.count_cheats_single(pos, path, limit, cheat_time);
        }
        count
    }

    fn get(&self, pos: Position) -> SquareState {
        if pos.0 >= self.map.len() || pos.1 >= self.map[pos.0].len() {
            return SquareState::Wall; // little hack so we don't have to return option :)
        }
        self.map[pos.0][pos.1]
    }

    fn add(pos: Position, dy: i64, dx: i64) -> Option<Position> {
        let row = pos.0 as i64 + dy;
        let col = pos.1 as i64 + dx;
        if row < 0 || col < 0 {
            None
        } else {
            Some((row as usize, col as usize))
        }
    }

    fn count_cheats_single(
        &self,
        pos: Position,
        path: &HashMap<Position, i64>,
        limit: i64,
        cheat_time: i64,
    ) -> i64 {
        let mut count = 0;
        for dy in -cheat_time..=cheat_time {
            for dx in -cheat_time..=cheat_time {
                if dx.abs() + dy.abs() > cheat_time {
                    continue;
                }
                let Some(next) = Self::add(pos, dy, dx) else {
                    continue;
                };
                if self.get(next) == SquareState::Open {
                    // valid cheat
                    let time_saved = path.get(&next).unwrap() - path.get(&pos).unwrap() - dy.abs() - dx.abs();
                    if time_saved >= limit {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i64 {
    let track = Track::from_input(input);
    // track.display();
    let path = track.find_path();
    track.count_cheats(&path, 100, 2)
}

fn part_2(input: &str) -> i64 {
    let track = Track::from_input(input);
    // track.display();
    let path = track.find_path();
    track.count_cheats(&path, 100, 20)
}
