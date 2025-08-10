use advent_of_code_2024::aoc;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum NumericKey {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    A,
}

impl NumericKey {
    fn from_char(c: char) -> Self {
        match c {
            '0' => Self::_0,
            '1' => Self::_1,
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'A' => Self::A,
            _ => panic!("invalid char"),
        }
    }
    fn as_pos(&self) -> (i32, i32) {
        use NumericKey::*;
        match self {
            _0 => (3, 1),
            _1 => (2, 0),
            _2 => (2, 1),
            _3 => (2, 2),
            _4 => (1, 0),
            _5 => (1, 1),
            _6 => (1, 2),
            _7 => (0, 0),
            _8 => (0, 1),
            _9 => (0, 2),
            A => (3, 2),
        }
    }
    fn from_pos(pos: (i32, i32)) -> Option<Self> {
        use NumericKey::*;
        match pos {
            (3, 1) => Some(_0),
            (2, 0) => Some(_1),
            (2, 1) => Some(_2),
            (2, 2) => Some(_3),
            (1, 0) => Some(_4),
            (1, 1) => Some(_5),
            (1, 2) => Some(_6),
            (0, 0) => Some(_7),
            (0, 1) => Some(_8),
            (0, 2) => Some(_9),
            (3, 2) => Some(A),
            _ => None,
        }
    }
    fn apply(&self, dirkey: DirectionalKey) -> Option<Self> {
        let pos = self.as_pos();
        let (d0, d1) = dirkey.as_delta();
        let new_pos = (pos.0 + d0, pos.1 + d1);
        Self::from_pos(new_pos)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirectionalKey {
    const ALL: [Self; 5] = [Self::Up, Self::Down, Self::Left, Self::Right, Self::A];

    fn as_delta(&self) -> (i32, i32) {
        match self {
            DirectionalKey::Up => (-1, 0),
            DirectionalKey::Down => (1, 0),
            DirectionalKey::Left => (0, -1),
            DirectionalKey::Right => (0, 1),
            _ => panic!("A does not have a delta"),
        }
    }

    fn from_pos(pos: (i32, i32)) -> Option<Self> {
        match pos {
            (0, 1) => Some(Self::Up),
            (0, 2) => Some(Self::A),
            (1, 0) => Some(Self::Left),
            (1, 1) => Some(Self::Down),
            (1, 2) => Some(Self::Right),
            _ => None,
        }
    }

    fn as_pos(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::A => (0, 2),
            Self::Left => (1, 0),
            Self::Down => (1, 1),
            Self::Right => (1, 2),
        }
    }

    fn apply(&self, dirkey: DirectionalKey) -> Option<Self> {
        let (d0, d1) = dirkey.as_delta();
        let pos = self.as_pos();
        let new_pos = (pos.0 + d0, pos.1 + d1);
        Self::from_pos(new_pos)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    codelen: usize,
    dpads: [DirectionalKey; N],
    npad: NumericKey,
}

impl<const N: usize> State<N> {
    fn new() -> Self {
        Self {
            codelen: 0,
            dpads: [DirectionalKey::A; N],
            npad: NumericKey::A,
        }
    }

    fn is_complete(&self, pattern: &Vec<NumericKey>) -> bool {
        self.codelen == pattern.len()
    }

    /// Returns Some(state) if the next state would be valid.
    /// Returns None if the next state is invalid:
    /// 1. incorrect code entered
    /// 2. hovering over blank space
    fn apply(&self, dirkey: DirectionalKey, pattern: &Vec<NumericKey>) -> Option<Self> {
        let mut next_state = self.clone();

        let mut curr_dirkey = Some(dirkey);
        for i in 0..next_state.dpads.len() {
            let Some(key) = curr_dirkey else { break };
            if key == DirectionalKey::A {
                curr_dirkey = Some(next_state.dpads[i]);
            } else {
                curr_dirkey = None;
                let Some(next_dirkey) = next_state.dpads[i].apply(key) else { return None };
                next_state.dpads[i] = next_dirkey;
            }
        }

        let Some(key) = curr_dirkey else { return Some(next_state) };
        if key == DirectionalKey::A {
            let numkey = next_state.npad;
            if pattern[next_state.codelen] != numkey {
                return None;
            }
            next_state.codelen += 1;
        } else {
            let Some(next_numkey) = next_state.npad.apply(key) else { return None };
            next_state.npad = next_numkey;
        }

        Some(next_state)
    }
}

/// Returns the number of keys which must be pressed by the human
/// in order to achieve the desired numeric pattern.
fn solve<const N: usize>(pattern: Vec<NumericKey>) -> i64 {
    let mut curr = State::new();

    let mut queue: VecDeque<State<N>> = VecDeque::new();
    let mut dist: HashMap<State<N>, i64> = HashMap::new();

    queue.push_back(curr);
    dist.insert(curr, 0);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        // dbg!(curr);

        let curr_dist = *dist.get(&curr).unwrap();
        for dirkey in DirectionalKey::ALL.iter() {
            let Some(next_state) = curr.apply(*dirkey, &pattern) else {
                // dbg!(dirkey);
                continue;
            };

            if dist.get(&next_state).is_some() {
                // dbg!(dirkey, next_state);
                continue;
            }

            if next_state.is_complete(&pattern) {
                // dbg!(dirkey);
                return curr_dist + 1;
            }

            dist.insert(next_state, curr_dist + 1);
            queue.push_back(next_state);
        }
    }

    panic!("queue is empty :(")
}

fn pattern_from_line(line: &str) -> Vec<NumericKey> {
    line.chars().map(NumericKey::from_char).collect()
}

fn main() {
    let input = include_str!("../input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i64 {
    let patterns: Vec<Vec<NumericKey>> = input.lines().map(pattern_from_line).collect();
    let mut sum = 0;
    for (pattern, s) in patterns.into_iter().zip(input.lines()) {
        // println!("{}", solve::<2>(pattern));
        let complexity = s[..s.len()-1].parse::<i64>().unwrap();
        sum += complexity * solve::<2>(pattern);
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let patterns: Vec<Vec<NumericKey>> = input.lines().map(pattern_from_line).collect();
    let mut sum = 0;
    for (pattern, s) in patterns.into_iter().zip(input.lines()) {
        let complexity = s[..s.len()-1].parse::<i64>().unwrap();
        sum += complexity * solve::<7>(pattern);
        println!("hi");
    }
    sum
}
