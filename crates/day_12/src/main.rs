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

fn print(garden: &Vec<Vec<char>>) {
    for row in 0..garden.len() {
        for col in 0..garden.len() {
            print!("{}", garden[row][col]);
        }
        println!();
    }
}

fn turn(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, direction.0)
}

fn flip(direction: (i32, i32)) -> (i32, i32) {
    (direction.0 * -1, direction.1 * -1)
}

fn add(pos: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (pos.0 + direction.0, pos.1 + direction.1)
}

fn dfs(row: usize, col: usize, garden: &mut Vec<Vec<char>>) -> Specs {
    let curr = garden[row][col];

    // prevent visiting the same spot twice
    let lower = curr.to_ascii_lowercase();
    garden[row][col] = lower;

    let mut perimeter = 0;
    let mut sides = 0;
    let mut area = 1;

    // do sides first
    for (dx, dy) in DIRECTIONS {
        let new_row = (row as i32 + dx) as usize;
        let new_col = (col as i32 + dy) as usize;
        let peek = garden[new_row][new_col];
        if peek != curr && peek != lower {
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
            let c = add(pos, flip(turn(direction)));
            let d = add(c, direction);
            let ag = garden[a.0 as usize][a.1 as usize];
            let bg = garden[b.0 as usize][b.1 as usize];
            let cg = garden[c.0 as usize][c.1 as usize];
            let dg = garden[d.0 as usize][d.1 as usize];
            let previously_discovered_side = (ag == lower && (bg != curr && bg != lower))
                || (cg == lower && (dg != curr && dg != lower));

            // println!("{} {} {} {}", ag, bg, cg, dg);

            if !previously_discovered_side {
                println!("O {}, ({:?}) -> {:?}) [{curr}, {lower}]", garden[row][col], pos, direction);
                print(garden);
                sides += 1;
            } else {
                println!("X {}, ({:?}) -> {:?}) [{curr}, {lower}]", garden[row][col], pos, direction);
            }
        }
    }

    for (dx, dy) in DIRECTIONS {
        let new_row = (row as i32 + dx) as usize;
        let new_col = (col as i32 + dy) as usize;
        let peek = garden[new_row][new_col];
        if curr == peek {
            // explore
            let specs = dfs(new_row, new_col, garden);
            perimeter += specs.perimeter;
            area += specs.area;
            sides += specs.sides;
        } else if peek != lower {
            // hit wall to the outside of the group
            perimeter += 1;
       }
    }

    // dbg!(curr, sides);

    Specs { area, perimeter, sides }
}

fn dfs_entry(garden: &mut Vec<Vec<char>>) -> HashMap<char, Vec<Specs>> {
    let mut agg = HashMap::new();
    for row in 0..garden.len() {
        for col in 0..garden[row].len() {
            let curr = garden[row][col];
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
0
    // let mut garden = parse_input(input);
    // let agg = dfs_entry(&mut garden);
    // let mut sum = 0;
    // dbg!(&agg);
    // for (_, groups) in agg.iter() {
    //     for group in groups.iter() {
    //         sum += group.area * group.perimeter;
    //     }
    // }
    // sum
}

fn part_2(input: &str) -> i32 {
    let mut garden = parse_input(input);
    let agg = dfs_entry(&mut garden);
    let mut sum = 0;
    dbg!(&agg);
    for (_, groups) in agg.iter() {
        for group in groups.iter() {
            sum += group.area * group.sides;
        }
    }
    sum
}
