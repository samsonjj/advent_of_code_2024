use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("part_1: {}", aoc::format_with_time(|| part_1(input)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(input)));
}

// returns (width, height, antenna_locations)
fn parse_input(input: &str) -> (i32, i32, HashMap<char, Vec<(i32, i32)>>) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height = std::cmp::max(height, y+1);
        for (x, c) in line.chars().enumerate() {
            width = std::cmp::max(width, x+1);
            if c != '.' && c != '#' {
                let entry = antennas.entry(c).or_insert(vec![]);
                entry.push((x as i32, y as i32));
            }
        }
    }
    (width as i32, height as i32, antennas)
}

fn print_nodes(width: i32, height: i32, set: &HashSet<(i32, i32)>) {
    for i in 0..height {
        for j in 0..width {
            if set.get(&(j, i)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_1(input: &str) -> i32 {
    let (width, height, antennas) = parse_input(input);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for freq in antennas.keys() {
        let positions: &Vec<(i32, i32)> = antennas.get(freq).unwrap();
        for i in 0..positions.len().saturating_sub(1) {
            for j in i+1..positions.len() {
                let a = positions[i];
                let b = positions[j];

                let diff = (a.0 - b.0, a.1 - b.1);
                let first_node = (a.0 + diff.0, a.1 + diff.1);
                let second_node = (b.0 - diff.0, b.1 - diff.1);
                set.insert(first_node);
                set.insert(second_node);
            }
        }
    }
    let nodes = set.into_iter().filter(|(x, y)| {
        *x >= 0 && *x < width && *y >= 0 && *y < height
    }).collect::<HashSet<(i32, i32)>>();

    // print_nodes(width, height, &nodes);

    nodes.len() as i32
}

fn part_2(input: &str) -> i32 {
    let (width, height, antennas) = parse_input(input);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for freq in antennas.keys() {
        let positions: &Vec<(i32, i32)> = antennas.get(freq).unwrap();
        for i in 0..positions.len().saturating_sub(1) {
            for j in i+1..positions.len() {
                let a = positions[i];
                let b = positions[j];

                let diff = (a.0 - b.0, a.1 - b.1);
                for mult in 0..1000 {
                    let node = (a.0 + mult * diff.0, a.1 + mult * diff.1);
                    let (x, y) = (node.0, node.1);
                    let in_bounds = 
                        x >= 0 && x < width && y >= 0 && y < height;
                    if !in_bounds {
                        break;
                    }
                    set.insert(node);
                }
                for mult in 0..1000 {
                    let node = (a.0 + -1 * mult * diff.0, a.1 + -1 * mult * diff.1);
                    let (x, y) = (node.0, node.1);
                    let in_bounds = 
                        x >= 0 && x < width && y >= 0 && y < height;
                    if !in_bounds {
                        break;
                    }
                    set.insert(node);
                }
                let second_node = (b.0 - diff.0, b.1 - diff.1);
                set.insert(second_node);
            }
        }
    }
    let nodes = set.into_iter().filter(|(x, y)| {
        *x >= 0 && *x < width && *y >= 0 && *y < height
    }).collect::<HashSet<(i32, i32)>>();

    // print_nodes(width, height, &nodes);

    nodes.len() as i32
}
