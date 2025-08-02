use advent_of_code_2024::aoc;
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines(); 
    let towels = lines.next().unwrap().split(", ").collect_vec();
    lines.next(); // throw away empty line
    let designs = lines.collect();
    (towels, designs)
}

fn can_make_design(design: &str, towels: &Vec<&str>) -> i64 {
    if design.len() == 0 {
        return 1;
    }
    let mut count = 0;
    for towel in towels.iter() {
        if design.starts_with(towel) {
            let sub_design = &design[towel.len()..];
            count += can_make_design(sub_design, towels);
        }
    }
    count
}

fn reduce_towels<'a>(towels: &'a Vec<&str>) -> (Vec<&'a str>, &HashMap<&str, i64>) {
    let mut best_towels = vec![];
    for i in 0..towels.len() {
        let towel = towels[i];
        let mut towels_clone = towels.clone();
        towels_clone.remove(i);
        if can_make_design(towel, &towels_clone) == 0 {
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
        can_make_design(design, &towels) > 0
    }).count() as i64
}

fn part_2(input: &str) -> i64 {
    let (towels, designs) = parse_input(input);
    let towels = reduce_towels(&towels);
    designs.into_iter().map(|design| {
        can_make_design(design, &towels)
    }).sum::<i64>() as i64
}
