use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque, HashSet};

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
    fn values() -> [Self; 11] {
        [
            Self::_0,
            Self::_1,
            Self::_2,
            Self::_3,
            Self::_4,
            Self::_5,
            Self::_6,
            Self::_7,
            Self::_8,
            Self::_9,
            Self::A,
        ]
    }

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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position(i32, i32);

impl Position {
    fn sub(&self, other: Position) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
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
    const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn values() -> [Self; 5] {
        Self::ALL
    }

    fn directional_values() -> [Self; 4] {
        Self::DIRECTIONS
    }

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

    /// Returns the two directions which will need to be used to get from a to b
    fn components_of(a: Position, b: Position) -> [Self; 2] {
        let delta = b.sub(a);
        match delta {
            _ => unimplemented!(),
            // (d0, d1) if d0 > 0 && d1 > 0 => 
        }
    }
}

fn numeric_transition_dfs(
    curr: NumericKey,
    target: NumericKey,
    visited: &mut HashSet<NumericKey>,
    path: &mut Vec<DirectionalKey>,
    paths: &mut Vec<Vec<DirectionalKey>>,
) {
    dbg!(curr, target);
    if curr == target {
        paths.push(path.clone());
        return;
    }
    for direction in DirectionalKey::directional_values() {
        let Some(next) = curr.apply(direction) else { continue };
        if visited.contains(&next) { continue };
        visited.insert(next);
        path.push(direction);
        numeric_transition_dfs(next, target, visited, path, paths);
        path.remove(path.len() - 1);
        visited.remove(&next);
    }
}

fn numeric_transitions() -> HashMap<(NumericKey, NumericKey), Vec<Vec<DirectionalKey>>> {
    let mut hm = HashMap::new();
    for start in NumericKey::values() {
        for end in NumericKey::values() {
            let mut visited = HashSet::new();
            visited.insert(start);
            let mut path = vec![];
            let mut paths = vec![];
            numeric_transition_dfs(start, end, &mut visited, &mut path, &mut paths);
            hm.insert((start, end), paths.clone());
        }
    }

    // Optimize for path length
    for (key, paths) in hm.iter_mut() {
        let shortest_path_length = paths.iter().map(|path| path.len()).min().unwrap();
        let shortest_paths = paths.iter().cloned().filter(|path| path.len() == shortest_path_length).collect_vec();
        *paths = shortest_paths;
    }

    // Optimize for zigzagging
    for ((start, end), paths) in hm.iter_mut() {
        let mut start_i = 0;
        let mut end_i = 0;
        if *start == NumericKey::A {
            start_i = 1;
        }
        if *end == NumericKey::A {
            end_i = 1;
        }

        // detect and filter zig zag (WIP)
        *paths = paths.iter().cloned().filter(|path| {
            let path_slice = path[start_i..path.len()]
        }).collect_vec();
    }
    hm
}

fn main() {
    let input = include_str!("../example.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i32 {
    let hm = numeric_transitions();
    dbg!(&hm);
    1
}

fn part_2(input: &str) -> i32 {
    0
}
