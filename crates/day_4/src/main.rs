use advent_of_code_2024::aoc;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect_vec()).collect_vec()
}

type PatternItem = ((usize, usize), char);
type Pattern = Vec<PatternItem>;

fn xmas_patterns() -> Vec<Pattern> {
    let x = |positions: Vec<(usize, usize)>| {
        vec![
            vec![
                (positions[0], 'X'),
                (positions[1], 'M'),
                (positions[2], 'A'),
                (positions[3], 'S'),
            ],
            vec![
                (positions[3], 'X'),
                (positions[2], 'M'),
                (positions[1], 'A'),
                (positions[0], 'S'),
            ]
        ]
    };
    vec![
        x(vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
        ]),
        x(vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
        ]),
        x(vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
        ]),
        x(vec![
            (3, 0),
            (2, 1),
            (1, 2),
            (0, 3),
        ]),
    ]
        .into_iter().flatten().collect_vec()
}

fn x_mas_patterns() -> Vec<Pattern> {
    vec![
        vec![
            ((0, 0), 'M'),
            ((1, 1), 'A'),
            ((2, 2), 'S'),
            ((2, 0), 'M'),
            ((0, 2), 'S'),
        ],
        vec![
            ((0, 0), 'M'),
            ((1, 1), 'A'),
            ((2, 2), 'S'),
            ((2, 0), 'S'),
            ((0, 2), 'M'),
        ],
        vec![
            ((0, 0), 'S'),
            ((1, 1), 'A'),
            ((2, 2), 'M'),
            ((2, 0), 'M'),
            ((0, 2), 'S'),
        ],
         vec![
            ((0, 0), 'S'),
            ((1, 1), 'A'),
            ((2, 2), 'M'),
            ((2, 0), 'S'),
            ((0, 2), 'M'),
        ],
    ]
}

fn match_pattern_at_position(pattern: &Pattern, position: (usize, usize), data: &Vec<Vec<char>>) -> bool {
    for pattern_item in pattern.iter() {
        if !match_pattern_item_at_position(*pattern_item, position, data) {
            return false;
        }
    }
    true
}

fn match_pattern_item_at_position(pattern_item: PatternItem, position: (usize, usize), data: &Vec<Vec<char>>) -> bool {
    let (x, y) = position;
    let (x, y) = (x + pattern_item.0.0, y + pattern_item.0.1);

    // check bounds
    if y >= data.len() {
        return false;
    }
    if x >= data[y].len() {
        return false;
    }

    // check pattern
    data[y][x] == pattern_item.1
}


fn main() {
    let input = include_str!("input.txt");
    println!("part_1: {}", aoc::format_with_time(|| part_1(input)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(input)));
}

fn part_1(input: &str) -> i32 {
    let data: Vec<Vec<char>> = parse_input(input);
    let patterns = xmas_patterns();
    let mut sum = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let position = (i, j);
            for pattern in patterns.iter() {
                if match_pattern_at_position(pattern, (i, j), &data) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let data: Vec<Vec<char>> = parse_input(input);
    let patterns = x_mas_patterns();
    let mut sum = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let position = (i, j);
            for pattern in patterns.iter() {
                if match_pattern_at_position(pattern, (i, j), &data) {
                    sum += 1;
                }
            }
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

    #[test]
    fn test_match_pattern_item_at_position() {
        let pattern_item = ((0, 0), 'X');
        // (0, 0)
        assert!(match_pattern_item_at_position(pattern_item, (0, 0), &vec![vec!['X']]));
        assert!(!match_pattern_item_at_position(pattern_item, (0, 0), &vec![vec!['M']]));

        // (1, 1)
        let pattern_item = ((1, 1), 'X');
        assert!(match_pattern_item_at_position(pattern_item, (0, 0), &vec![vec![], vec!['.', 'X']]));
        assert!(!match_pattern_item_at_position(pattern_item, (0, 0), &vec![vec![], vec!['.', 'M']]));
    }

    fn test_match_pattern_at_position() {
        let pattern = vec![
            ((0, 0), 'X'),
            ((1, 1), 'M'),
        ];
        let data = &vec![
            vec!['X', '.'],
            vec!['.', 'M']
        ];
        assert!(match_pattern_at_position(&pattern, (0, 0), &data));
        let data = &vec![
            vec!['X', '.'],
            vec!['.', 'X']
        ];
        assert!(!match_pattern_at_position(&pattern, (0, 0), &data));
    }
}

