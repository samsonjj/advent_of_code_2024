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

    fn count_cheats(&self, path: &HashMap<Position, i64>, limit: i64) -> i64 {
        let mut count = 0;
        for (&pos, _) in path.iter() {
            count += self.count_cheats_single(pos, path, limit);
        }
        count
    }

    fn get(&self, pos: Position) -> SquareState {
        if pos.0 >= self.map.len() || pos.1 >= self.map[pos.0].len() {
            return SquareState::Wall; // little hack so we don't have to return option :)
        }
        self.map[pos.0][pos.1]
    }

    fn count_cheats_single(&self, pos: Position, path: &HashMap<Position, i64>, limit: i64) -> i64 {
        let mut count = 0;
        for dir in DIRECTIONS.iter() {
            let in_between = (*dir + pos).unwrap();
            let Some(next) = *dir + in_between else {
                continue;
            };
            println!(
                "{:?} -> {:?} -> {:?} = {}",
                pos,
                in_between,
                next,
                self.get(in_between) == SquareState::Wall && self.get(next) == SquareState::Open,
            );
            if self.get(in_between) == SquareState::Wall && self.get(next) == SquareState::Open {
                // valid cheat
                let time_saved = path.get(&next).unwrap() - path.get(&pos).unwrap() - 2;
                println!("time_saves={}", time_saved);
                if time_saved >= limit {
                    count += 1;
                }
            }
        }
        count
    }

    // fn solve(&self, dists: HashMap<>) {
    //     let mut dist = HashMap::new();
    //     let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    //     let mut queue = VecDeque::new();
    //     queue.push_front(self.start);
    //     dist.insert(self.start, 0);

    //     while !queue.is_empty() {
    //         let pos = queue.pop_back().unwrap();
    //

    //         for dir in DIRECTIONS {
    //             let next_pos = (dir + pos).unwrap();
    //             let State::Open = self.map[next_pos.0][next_pos.1] else { continue; };
    //             if dist.contains_key(&next_pos) {
    //                 continue;
    //             }
    //             dist.insert(next_pos, dist.get(&pos).unwrap() + 1);
    //             prev.insert(next_pos, pos);
    //             queue.push_front(next_pos);
    //         }
    //     }

    //     dist
    // }
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i64 {
    let track = Track::from_input(input);
    track.display();
    let path = track.find_path();
    track.count_cheats(&path, 100)
}

fn part_2(input: &str) -> i64 {
    0
}
