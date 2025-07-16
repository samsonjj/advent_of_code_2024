use advent_of_code_2024::aoc;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut data = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).expect(format!("failed to parse '{c}' to digit").as_str()) as i32).collect_vec()).collect_vec();
    for row in data.iter_mut() {
        row.insert(0, 10);
        row.insert(row.len(), 10);
    }
    data.insert(0, vec![10; data[0].len()]);
    data.push(vec![10; data[0].len()]);
    data
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

// returns the score from a given trailhead
fn traverse(row: usize, col: usize, data: &mut Vec<Vec<i32>>, distinct: bool) -> i32 {
    // prevent traversal to this spot ever again, because why would you ever wanna go somewhere twice??
    let curr_height = data[row][col];
    if !distinct {
        data[row][col] = 10;
    }

    // end case
    if curr_height == 9 {
        return 1;
    }

    // try to move all directions
    let mut sum = 0;
    for direction in DIRECTIONS.iter() {
        let next_row = (row as i32 + direction.0) as usize;
        let next_col = (col as i32 + direction.1) as usize;
        if data[next_row][next_col] == curr_height + 1 {
            sum += traverse(next_row, next_col, data, distinct);
        }
    }

    sum
}

fn part_1(input: &str) -> i32 {
    let mut data = parse_input(input);
    let mut sum = 0;
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            if data[row][col] == 0 {
                let mut data = data.clone(); // hack
                let score = traverse(row, col, &mut data, false);
                // println!("{row}, {col}: score = {score}");
                sum += score;
            }
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let mut data = parse_input(input);
    let mut sum = 0;
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            if data[row][col] == 0 {
                let score = traverse(row, col, &mut data, true);
                // println!("{row}, {col}: score = {score}");
                sum += score;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parsed = parse_input("123\n456");
        assert_eq!(parsed, vec![
            vec![10, 10, 10, 10, 10],
            vec![10, 1, 2, 3, 10],
            vec![10, 4, 5, 6, 10],
            vec![10, 10, 10, 10, 10],
        ]);
    }
}
