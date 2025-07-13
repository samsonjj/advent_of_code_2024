use itertools::Itertools;
use advent_of_code_2024::aoc;

const INPUT: &'static str = include_str!("input.txt");
const EXAMPLE: &'static str = include_str!("example.txt");

fn main() {
    println!("part_1: {}", aoc::format_with_time(|| part_1(INPUT)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(INPUT)));
}

fn increasing(data: &Vec<i32>) -> bool {
    for i in 0..data.len().saturating_sub(1) {
        let curr = data[i];
        let next = data[i+1];

        if next <= curr {
            return false;
        }
    }
    return true;
}

fn decreasing(data: &Vec<i32>) -> bool {
    for i in 0..data.len().saturating_sub(1) {
        let curr = data[i];
        let next = data[i+1];

        if next >= curr {
            return false;
        }
    }
    return true;
}

// returns true if abs diff between all adjacent numbers is 3 or less
fn bounded(data: &Vec<i32>) -> bool {
    for i in 0..data.len().saturating_sub(1) {
        let curr = data[i];
        let next = data[i+1];

        if (next - curr).abs() > 3 {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line.split_whitespace().map(|s: &str| s.parse::<i32>().unwrap()).collect_vec()).collect_vec()
}

fn part_1(input: &str) -> i32 {
    let data = parse_input(input);
    data.into_iter().filter(|row| is_safe(row)).count() as i32
}

fn is_safe(row: &Vec<i32>) -> bool {
    (increasing(&row) || decreasing(&row)) && bounded(&row)
}

fn is_close_to_safe(row: &Vec<i32>) -> bool {
    if is_safe(row) {
        return true;
    }

    let mut row = row.clone();
    for i in 0..row.len() {
        let item = row.remove(i);     
        if is_safe(&row) {
            return true;
        }
        row.insert(i, item);
    }

    false
}

fn part_2(input: &str) -> i32 {
    let data = parse_input(input);
    data.into_iter().filter(|row| is_close_to_safe(row)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        assert!(increasing(&vec![1, 2, 3, 4, 5]));
        assert!(!increasing(&vec![1, 2, 2, 3, 4, 5]));
        assert!(!increasing(&vec![5, 4, 3, 2, 1]));
        assert!(increasing(&vec![]));
    }

    #[test]
    fn test_decreasing() {
        assert!(!decreasing(&vec![1, 2, 3, 4, 5]));
        assert!(!decreasing(&vec![1, 2, 2, 3, 4, 5]));
        assert!(decreasing(&vec![5, 4, 3, 2, 1]));
        assert!(decreasing(&vec![]));
    }

    #[test]
    fn test_bounded() {
        assert!(bounded(&vec![1, 2, 3, 4, 5]));
        assert!(bounded(&vec![1, 2, 2, 3, 4, 5]));
        assert!(bounded(&vec![5, 4, 3, 2, 1]));
        assert!(bounded(&vec![]));
        assert!(!bounded(&vec![1, 5]));
        assert!(!bounded(&vec![1, 2, 3, -1, 1]));
    }

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 4);
    }
}
