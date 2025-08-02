use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Direction {
    fn from_delta(delta: (i64, i64)) -> Self {
        match delta {
            (-1, 0) => Direction::North,
            (1, 0) => Direction::South,
            (0, -1) => Direction::West,
            (0, 1) => Direction::East,
            x => panic!("{x:?}"),
        }
    }
    fn deltas(&self) -> (i64, i64) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
    fn turnC(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
    fn turnCC(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }
}

impl std::ops::Add<(usize, usize)> for Direction {
    type Output = Option<(usize, usize)>;
    fn add(self, other: (usize, usize)) -> Option<(usize, usize)> {
        let (dx, dy) = self.deltas();
        if other.0 as i64 + dx < 0 {
            return None;
        }
        if other.1 as i64 + dy < 0 {
            return None
        }
        let x = (other.0 as i64 + dx) as usize;
        let y = (other.1 as i64 + dy) as usize;
        Some((x, y))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SquareState {
    Corrupted,
    Open,
}

impl SquareState {
    fn as_char(&self) -> char {
        match self {
            SquareState::Corrupted => '#',
            SquareState::Open => '.',
        }
    }
}

#[derive(Clone, Debug)]
struct Maze {
    squares: Vec<Vec<SquareState>>,
}

impl Maze {
    fn new(w: usize, h: usize) -> Self {
        let squares = vec![vec![SquareState::Open; w]; h];
        Self { squares }
    }

    fn set(&mut self, pos: (usize, usize), state: SquareState) {
        self.squares[pos.0][pos.1] = state;
    }

    fn corrupt(&mut self, squares: &[(usize, usize)]) {
        for pos in squares.iter() {
            self.set(*pos, SquareState::Corrupted);
        }
    }
    
    fn display(&self) {
        for row in self.squares.iter() {
            for item in row.iter() {
                print!("{}", item.as_char());
            }
            println!();
        }
        println!();
    }

    fn state_of(&self, pos: (usize, usize)) -> Option<SquareState> {
        if pos.0 >= self.squares.len() || pos.1 >= self.squares[0].len() {
            None
        } else {
            Some(self.squares[pos.0][pos.1])
        }
    }

    fn bfs(&self) -> Option<usize> {
        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        dist.insert((0, 0), 0);
        queue.push_front((0, 0));

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            for dir in DIRECTIONS.iter() {
                let Some(next_pos) = *dir + pos else { continue; };
                let Some(state) = self.state_of(next_pos) else { continue; };

                if state == SquareState::Corrupted {
                    continue;
                }

                if prev.contains_key(&next_pos) {
                    continue;
                }

                prev.insert(next_pos, pos);
                dist.insert(next_pos, dist.get(&pos).unwrap() + 1);
                queue.push_back(next_pos);
            }
        }

        dist.get(&(self.squares.len()-1, self.squares[0].len()-1)).copied()
    }
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| line.split(',').map(|x| x.parse::<usize>().expect(format!("couldn't parse: {x}").as_str())).collect_tuple().unwrap())
        .collect()
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> String {
    let bytes = parse_input(input);
    let mut maze = Maze::new(71, 71);
    maze.corrupt(&bytes[0..1024]);
    // maze.display();
    format!("{}", maze.bfs().unwrap())
}

fn part_2(input: &str) -> String {
    let bytes = parse_input(input);
    let maze = Maze::new(71, 71);
    for i in 0..1_000_000 {
        let mut maze = maze.clone();
        maze.corrupt(&bytes[0..i]);
        if let None = maze.bfs() {
            // maze.display();
            return format!("{:?}", bytes[i-1]);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input("1,2
3,4");
        let expected = vec![(1,2),(3,4)];
        assert_eq!(result,expected);
    }
}
