use advent_of_code_2024::aoc;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=((?:-?)\d+),((?:-?)\d+) v=((?:-?)\d+),((?:-?)\d+)").unwrap();
    let mut robots = vec![];
    for (_, [px, py, vx, vy]) in re.captures_iter(input).map(|c| c.extract()) {
        let [px, py, vx, vy] = [px, py, vx, vy].map(|val| val.parse::<i64>().unwrap());
        robots.push(Robot { px, py, vx, vy });
    }
    robots
}

fn main() {
    let input: &str = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn resulting_position(robot: &Robot, bx: i64, by: i64, iterations: i64) -> (i64, i64) {
    let x = (robot.px + robot.vx * iterations).rem_euclid(bx);
    let y = (robot.py + robot.vy * iterations).rem_euclid(by);
    (x, y)
}

fn part_1(input: &str) -> i64 {
    let robots = parse_input(input);
    dbg!(&robots);
    let (bx, by) = (101, 103);
    let resulting_positions = robots.into_iter().map(|robot| resulting_position(&robot, bx, by, 100)).collect_vec();
    dbg!(&resulting_positions);
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for (x, y) in resulting_positions.into_iter() {
        let left = x < bx / 2;
        let right = x > bx / 2;
        let up = y < by / 2;
        let down = y > by / 2;

        if left && up { a += 1 }
        else if left && down { b += 1}
        else if right && up { c += 1 }
        else if right && down { d += 1 }
    }
    dbg!(&a, &b, &c, &d);
    a * b * c * d
}

/**
 * This part doesn't give a direct answer, but is rather a tool to find the
 * answer. So I'll comment the methodology used ot find the tree:
 * 
 * On visual inspection of repeated frames, printing out the grid of robots,
 * I saw multiple frames where some vague pattern was forming.
 * 
 * Two of these frames fell on frame numbers 13 and 114. The difference between
 * these being 101, I felt that the image was likely to occur on the same
 * cadence. Thus, we display frame numbers 13 + 101n where n is an integer.
 * 
 * We soon find the first christmas tree frame on 7083.
 */
fn part_2(input: &str) -> i64 {
    let robots = parse_input(input);
    let (bx, by) = (101, 103);
    
    let mut i = 13;

    loop {
        let resulting_positions = robots.iter().map(|robot| resulting_position(robot, bx, by, i)).collect_vec();
        let mut data = vec![vec![false; 101]; 103];
        for (x, y) in resulting_positions {
            data[y as usize][x as usize] = true;
        }

        println!("iteration {i}");
        for row in data.iter() {
            for col in row.iter() {
                print!("{}", if *col { 'O' } else {'.'});
            }
            println!();
        }
        println!();
        let mut a = String::new();
        std::io::stdin().read_line(&mut a);
        i += 101;
    }
}
