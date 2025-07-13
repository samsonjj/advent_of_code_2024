use regex::Regex;
use advent_of_code_2024::aoc;

const INPUT: &'static str = include_str!("input.txt");
const EXAMPLE: &'static str = include_str!("example.txt");

fn main() {
    println!("part_1: {}", aoc::format_with_time(|| part_1(INPUT)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(INPUT)));
}

fn parse_int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

fn part_1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum: i64 = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c: regex::Captures| c.extract()) {
        sum += parse_int(a) * parse_int(b);
    }

    sum
}

const MUL_RE: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
// must have same number of capture groups, as the above regex, so we add a dummy caputre group
const DO_RE: &str = r"(do\(\))()";
const DONT_RE: &str = r"(don't\(\))()";

fn part_2(input: &str) -> i64 {
    let re_string = format!("{}|{}|{}", MUL_RE, DO_RE, DONT_RE);
    let re = Regex::new(re_string.as_str()).unwrap();

    let mut sum: i64 = 0;
    let mut enabled = true;
    for (_, [a, b]) in re.captures_iter(input).map(|c: regex::Captures| c.extract()) {
        // if "do" or "don't", will be present in the first capture group
        match a {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => if enabled { sum += parse_int(a) * parse_int(b) },
        };
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stupid_test() {
        assert_eq!(1 + 1, 2);
    }
}
