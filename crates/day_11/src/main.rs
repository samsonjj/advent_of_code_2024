use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::HashMap;

fn blink(stone: i64) -> (i64, Option<i64>) {
    if stone == 0 {
        return (1, None);
    }

    let stone_string = format!("{stone}");
    if stone_string.len() % 2 == 0 {
        let left = (&stone_string[0..stone_string.len()/2]).parse::<i64>().unwrap();
        let right = (&stone_string[stone_string.len()/2..stone_string.len()]).parse::<i64>().unwrap();
        return (left, Some(right))
    }

    (stone * 2024, None)
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect_vec()
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i64 {
    let mut stones = parse_input(input);

    for iteration in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            let result = blink(stone);
            stones[i] = result.0;
            if let Some(right_stone) = result.1 {
                stones.insert(i + 1, right_stone);
                i += 1;
            }
            i += 1;
        }    
    }
    stones.len() as i64
}

fn part_2(input: &str) -> i64 {
    let data = parse_input(input);

    let mut stone_counts: HashMap<i64, i64> = data.iter().map(|stone| (*stone, 1)).collect::<HashMap<_, _>>();
    for iteration in 0..75 {
        let mut next_stone_counts = HashMap::new();
        for (&stone, &count) in stone_counts.iter() {
            let (left_stone, right_stone) = blink(stone);        
            *next_stone_counts.entry(left_stone).or_insert(0) += count;
            if let Some(right_stone) = right_stone {
                *next_stone_counts.entry(right_stone).or_insert(0) += count;
            }
        }
        stone_counts = next_stone_counts;
    }
    stone_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), (1, None));
        assert_eq!(blink(1), (2024, None));
        assert_eq!(blink(10), (1, Some(0)));
        assert_eq!(blink(2024), (20, Some(24)));
        assert_eq!(blink(1000), (10, Some(0)));
    }
}
