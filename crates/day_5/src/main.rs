use advent_of_code_2024::aoc;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Rule(i32, i32);

struct Data {
    updates: Vec<Vec<i32>>,
    rules: Vec<Rule>,
}

fn parse_input(input: &str) -> Data {
    let rules_re = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let rules = rules_re.captures_iter(input).map(|c| {
        let (_, [first, second]) = c.extract();
        Rule(first.parse::<i32>().unwrap(), second.parse::<i32>().unwrap())
    })
        .collect_vec();

    let updates_re = Regex::new(r"((?:\d+,)+(?:\d+))").unwrap();
    let updates = updates_re.captures_iter(input).map(|c| {
        let (_, [nums]) = c.extract();
        nums.split(',').map(|s: &str| s.parse::<i32>().unwrap()).collect_vec()
    }).collect_vec();

    Data {
        updates,
        rules,
    }
}

fn aggregate_rules(rules: &Vec<Rule>) -> HashSet<Rule> {
    let mut set = HashSet::new();
    for rule in rules.iter() {
        set.insert(Rule(rule.0, rule.1));
    }
    set
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", aoc::format_with_time(|| part_1(input)));
    println!("{}", aoc::format_with_time(|| part_2(input)));
}

fn part_1(input: &str) -> i32 {
    let Data { updates, rules } = parse_input(input);
    let rules_set = aggregate_rules(&rules);

    let mut sum = 0;
    'outer: for update in updates.into_iter() {
        for i in 0..update.len().saturating_sub(1) {
            let rule = Rule(update[i], update[i + 1]);
            if !rules_set.get(&rule).is_some() {
                continue 'outer;
            }
        }
        sum += update[update.len() / 2];
    }

    sum
}

// returns true if the update required reordering
fn reorder(update: &mut Vec<i32>, rule_set: &HashSet<Rule>) -> bool {
    let mut updated = false;
    'outer: loop {
        for i in 0..update.len().saturating_sub(1) {
            let rule = Rule(update[i], update[i + 1]);
            if rule_set.get(&rule).is_none() {
                (update[i], update[i + 1]) = (update[i + 1], update[i]);
                updated = true;
                continue 'outer;
            }
        }
        break;
    }
    updated
}

fn part_2(input: &str) -> i32 {
    let Data { updates, rules } = parse_input(input);
    let rules_set = aggregate_rules(&rules);

    let mut sum = 0;
    'outer: for mut update in updates.into_iter() {
        if reorder(&mut update, &rules_set) {
            sum += update[update.len() / 2];
        }
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
