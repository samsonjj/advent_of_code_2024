use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines(); 
    let towels = lines.next().unwrap().split(", ").collect_vec();
    lines.next(); // throw away empty line
    let designs = lines.collect();
    (towels, designs)
}

fn can_make_design(design: &str, towels: &Vec<&str>, memoize: &mut HashMap<usize, i64>) -> i64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(val) = memoize.get(&design.len()) {
        return *val;
    }

    let mut count = 0;
    for towel in towels.iter() {
        if design.starts_with(towel) {
            let sub_design = &design[towel.len()..];
            count += can_make_design(sub_design, towels, memoize);
        }
    }

    memoize.insert(design.len(), count);

    count
}

fn reduce_towels<'a>(towels: &'a Vec<&str>) -> Vec<&'a str> {
    let mut best_towels = vec![];
    for i in 0..towels.len() {
        let towel = towels[i];
        let mut towels_clone = towels.clone();
        towels_clone.remove(i);
        let mut memoize = HashMap::new();
        if can_make_design(towel, &towels_clone, &mut memoize) == 0 {
            best_towels.push(towel);
        }
    }
    best_towels
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i64 {
    let (towels, designs) = parse_input(input);
    let towels = reduce_towels(&towels);
    designs.into_iter().filter(|design| {
        let mut memoize = HashMap::new();
        can_make_design(design, &towels, &mut memoize) > 0
    }).count() as i64
}

fn part_2(input: &str) -> i64 {
    let (towels, designs) = parse_input(input);
    designs.into_iter().map(|design| {
        let mut memoize = HashMap::new();
        can_make_design(design, &towels, &mut memoize)
    }).sum::<i64>() as i64
}
