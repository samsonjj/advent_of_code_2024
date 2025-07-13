use advent_of_code_2024::aoc;
use regex::Regex;
use itertools::Itertools;

struct Equation {
    result: i64,
    values: Vec<i64>,
}

impl Equation {
    fn vec_from_input(input: &str) -> Vec<Self> {
        input.lines().map(|line| {
            let re = Regex::new(r"(\d+): ((?:\d+ )*\d+)").unwrap();
            let (_, [result_str, values_str]) = re.captures(line).unwrap().extract();
            let result = result_str.parse::<i64>().unwrap();
            let values = values_str.split_whitespace().map(&str::parse::<i64>).map(|x| x.unwrap()).collect_vec();
            Equation { result, values }
        }).collect::<Vec<Equation>>()
    }

    fn is_valid_slice(value: i64, slice: &[i64], allow_concat: bool) -> bool {
        dbg!(slice.len());
        if slice.len() == 0 {
            panic!("slice was empty");
        }
        if slice.len() == 1 {
            return slice[0] == value;
        }

        // start from the right side of the vec
        // if we would add (+)
        let last_index = slice.len()-1;
        Self::is_valid_slice(value - slice[last_index], &slice[..last_index], allow_concat)
            || (value % slice[last_index] == 0 && Self::is_valid_slice(value / slice[last_index], &slice[..last_index], allow_concat))
    }

    fn is_valid(&self, allow_concat: bool) -> bool {
        Self::is_valid_slice(self.result, &self.values[..], allow_concat)
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("part_1: {}", aoc::format_with_time(|| part_1(input)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(input)));
}

fn part_1(input: &str) -> i64 {
    let equations = Equation::vec_from_input(input);
    equations.into_iter().filter(|eq| eq.is_valid(false)).map(|eq| eq.result).sum::<i64>() as i64
}

fn part_2(input: &str) -> i64 {
    let equations = Equation::vec_from_input(input);
    equations.into_iter().filter(|eq| eq.is_valid(true)).map(|eq| eq.result).sum::<i64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stupid() {
        assert_eq!(1 + 1, 2);
    }
}
