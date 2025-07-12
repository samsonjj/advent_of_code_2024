use itertools::Itertools;
use std::collections::HashMap;

use advent_of_code_2024::aoc;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("input.txt");
#[allow(dead_code)]
const EXAMPLE: &'static str = include_str!("example.txt");

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    for line in input.lines() {
        let mut parts = line.split_whitespace()
            .map(|s: &str| s.parse::<i32>().unwrap());
        let (x1, x2) = (parts.next().unwrap(), parts.next().unwrap());

        l1.push(x1);
        l2.push(x2);
    }
    
    (l1, l2)
}

fn main() {
    let input: &str = INPUT;
    println!("part_1: {}", aoc::format_with_time(|| part_1(input)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(input)));
}

fn get_counts(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for item in list.iter() {
        let entry = counts.entry(*item).or_insert(0);
        *entry += 1;
    }
    counts
}

fn part_1(input: &str) -> i32 {
    let (mut l1, mut l2) = parse_input(input);
    l1.sort();
    l2.sort();

    let mut sum = 0;
    for (x1, x2) in l1.iter().zip(l2.iter()) {
        let dist = (x1 - x2).abs();
        sum += dist;
    }

    sum
}

fn part_2(input: &str) -> i32 {
    let (l1, l2) = parse_input(input);
    let right_counts = get_counts(&l2);
    
    let mut sum = 0;
    for item in l1 {
        sum += item * right_counts.get(&item).unwrap_or(&0);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let test_input = "1 2
        3 4
        5 6";
        
        let result: (Vec<i32>, Vec<i32>) = parse_input(test_input);
        let expected = (vec![1, 3, 5], vec![2, 4, 6]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_counts() {
        let test_data = vec![1, 1, 3, 3, 3, 2];
        let result = get_counts(&test_data);
        let mut expected = HashMap::new();
        expected.insert(1, 2);
        expected.insert(3, 3);
        expected.insert(2, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 31);
    }
}
