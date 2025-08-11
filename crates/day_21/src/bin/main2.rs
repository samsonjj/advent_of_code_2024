use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use day_21::keys::{DirectionalKey, Key, NumericKey};

const NEVER_ALLOW_ZIG_ZAGS: bool = true;

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
    for direction in DirectionalKey::directional_values().iter().copied() {
        let Some(next) = curr.apply(direction) else {
            continue;
        };
        if visited.contains(&next) {
            continue;
        };
        visited.insert(next);
        path.push(direction);
        numeric_transition_dfs(next, target, visited, path, paths);
        path.remove(path.len() - 1);
        visited.remove(&next);
    }
}

fn numeric_transitions() -> HashMap<(NumericKey, NumericKey), Vec<Vec<DirectionalKey>>> {
    let mut hm = HashMap::new();
    for start in NumericKey::values().iter().copied() {
        for end in NumericKey::values().iter().copied() {
            let mut visited = HashSet::new();
            visited.insert(start);
            let mut path = vec![];
            let mut paths = vec![];
            numeric_transition_dfs(start, end, &mut visited, &mut path, &mut paths);
            hm.insert((start, end), paths.clone());
        }
    }

    // Optimize for path length
    for (_key, paths) in hm.iter_mut() {
        let shortest_path_length = paths.iter().map(|path| path.len()).min().unwrap();
        let shortest_paths = paths
            .iter()
            .cloned()
            .filter(|path| path.len() == shortest_path_length)
            .collect_vec();
        *paths = shortest_paths;
    }

    // Optimize for zigzagging
    for ((start, end), paths) in hm.iter_mut() {
        let mut start_i = 0;
        let mut end_i = 0;

        if !NEVER_ALLOW_ZIG_ZAGS {
            if *start == NumericKey::A {
                start_i = 1;
            }
            if *end == NumericKey::A {
                end_i = 1;
            }
        }

        // detect and filter zig zag
        *paths = paths
            .iter()
            .cloned()
            .filter(|path| {
                if path.len() == 0 {
                    return true;
                }
                dbg!(&path, end_i);
                let path_slice = &path[start_i..path.len() - end_i];

                let mut zigzag_count = 0;
                for i in 1..path_slice.len() {
                    let prev = path_slice[i - 1];
                    let curr = path_slice[i];
                    if prev != curr {
                        zigzag_count += 1;
                    }
                }

                zigzag_count <= 1
            })
            .collect_vec();
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
