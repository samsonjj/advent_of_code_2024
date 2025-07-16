use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut garden = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    garden.insert(0, vec!['.'; garden[0].len()]);
    for row in garden.iter_mut() {
        row.insert(0, '.');
        row.insert(row.len(), '.');
    }
    garden.insert(garden.len(), vec!['.'; garden[0].len()]);
    garden
}

fn main() {
    let input = include_str!("example.txt");
    aoc::run_parts(input, part_1, part_2);
}

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

#[derive(Clone, Debug)]
struct Specs {
    area: i32,
    perimeter: i32,
    sides: i32,
}

fn turn(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, direction.0)
}

fn flip(direction: (i32, i32)) -> (i32, i32) {
    (direction.1 * -1, direction.1 * -1)
}

fn add(pos: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (pos.0 + direction.0, pos.1 + direction.1)
}

fn dfs(row: usize, col: usize, garden: &mut Vec<Vec<char>>) -> Specs {
    let curr = garden[col][row];

    // prevent visiting the same spot twice
    garden[col][row] = curr.to_ascii_lowercase();

    let mut perimeter = 0;
    let mut sides = 0;
    let mut area = 1;

    for (dx, dy) in DIRECTIONS {
        let new_row = (row as i32 + dx) as usize;
        let new_col = (col as i32 + dy) as usize;
        let peek = garden[new_col][new_row];
        if curr == peek {
            let specs = dfs(new_row, new_col, garden);
            perimeter += specs.perimeter;
            area += specs.area;
        } else if peek != curr.to_ascii_lowercase() {
            perimeter += 1;
            // check if it's a new side
            // there will be two possible locations of perimeter pieces of the same side which may have already been discovered
            //
            //    | <-
            // -> |
            //    | <-
            //
            // if they have already been discovered, then one of the two spots adjacent to that perimeter piece will have been visited

            //let spot_1 = (row

            let pos = (row as i32, col as i32);
            let direction = (dx, dy);
            let a = add(pos, turn(direction));
            let b = add(a, direction);
            let c = add(pos, turn(flip(direction)));
            let d = add(c, direction);
            dbg!(pos, a, b, c, d);
            let discovered_side = garden[a.1 as usize][a.0 as usize].is_lowercase()
                || garden[b.1 as usize][b.0 as usize].is_lowercase()
                || garden[c.1 as usize][c.0 as usize].is_lowercase()
                || garden[d.1 as usize][d.0 as usize].is_lowercase();
            dbg!(discovered_side);
            if !discovered_side {
                sides += 1;
            }
            continue;
        }
    }

    Specs { area, perimeter, sides }
}

fn dfs_entry(garden: &mut Vec<Vec<char>>) -> HashMap<char, Vec<Specs>> {
    let mut agg = HashMap::new();
    for col in 0..garden.len() {
        for row in 0..garden[col].len() {
            let curr = garden[col][row];
            if curr == '.' || curr.is_lowercase() {
                continue;
            }
            let specs = dfs(row, col, garden);
            agg.entry(curr).or_insert(vec![]).push(specs);
        }
    }
    agg
}


fn part_1(input: &str) -> i32 {
    let mut garden = parse_input(input);
    let agg = dfs_entry(&mut garden);
    let mut sum = 0;
    dbg!(&agg);
    for (_, groups) in agg.iter() {
        for group in groups.iter() {
            sum += group.area * group.perimeter;
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
0
    // let mut garden = parse_input(input);
    // let agg = dfs_entry(&mut garden);
    // let mut sum = 0;
    // dbg!(&agg);
    // for (_, groups) in agg.iter() {
    //     for group in groups.iter() {
    //         sum += group.area * group.sides;
    //     }
    // }
    // sum
}
