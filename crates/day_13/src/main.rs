use advent_of_code_2024::aoc;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let input: &str = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

#[derive(Clone, Debug)]
struct Game {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn parse_input(input: &str) -> Vec<Game> {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)").expect("failed to parse regex");
    
    let mut games = vec![];
    for (_, matches) in re.captures_iter(input).map(|c| c.extract::<6>()) {
        let [ax, ay, bx, by, px, py] = matches.iter().map(|s| s.parse::<i64>().unwrap()).collect_vec()[..] else { unreachable!() };
        games.push(Game { ax, ay, bx, by, px, py });
    }

    games
}

fn min_tokens_for(game: Game) -> Option<i64> {
    let mut min_tokens: Option<i64> = None;
    for a in 0..=100 {
        for b in 0..=100 {
            if (a * game.ax + b * game.bx) == game.px
            && (a * game.ay + b * game.by) == game.py
            {
                min_tokens = if let Some(val) = min_tokens {
                    Some(std::cmp::min(val, 3 * a + b))
                } else {
                    Some(3 * a + b)
                };
            }
        }
    }
    min_tokens
}

fn gcd(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if r == 0 {
        b
    } else {
        gcd(b, r)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}


/**
* Solves a system of equations of the following form:
*
* A1x + B1y = C1
* A2x + B2y = C2
*
* where all constants are natural numbers.
*/
fn solve_system(mut a1: i64, mut b1: i64, mut c1: i64, mut a2: i64, mut b2: i64, mut c2: i64) -> Option<(i64, i64)> {
    // find the lcm
    let b_lcm = lcm(b1, b2);

    // multiply the equations to make the b's equal
    let m1 = b_lcm / b1;
    let m2 = b_lcm / b2;
    a1 *= m1;
    b1 *= m1;
    c1 *= m1;
    a2 *= m2;
    b2 *= m2;
    c2 *= m2;

    // subtract the equations
    let a = a1 - a2;
    // b is zero
    let c = c1 - c2;

    if a == 0 {
        if c == 0 {
            // infinite solutions
            // pick the one that requires the least tokens
            // the one that requires the least tokens is the one with smallest x
            // which would be a solution of only y
            let x = 0;
            let y = c1 / b1;
            if c1 % b1 == 0 {
                // no integer solutions
                return None;
            }
            return Some((x, y));
        } else {
            // no solutions
            return None;
        }
    }

    // solve for x
    let x = c / a;
    let xr = c % a;
    if xr != 0 {
        // no integer solution
        return None;
    }

    let y = (c1 - a1 * x) / b1;
    let yr = (c1 - a1 * x) % b1;
    if yr != 0 {
        // no integer solution
        return None;
    }

    Some((x, y))
}

fn min_tokens_for_optimized(game: Game) -> Option<i64> {
    let solution = solve_system(
        game.ax,
        game.bx,
        game.px,
        game.ay,
        game.by,
        game.py,
    );
    if let Some((a, b)) = solution {
        Some(3 * a + b)
    } else {
        None
    }
}

fn part_1(input: &str) -> i64 {
    let games = parse_input(input);
    let all_games = games.into_iter().map(min_tokens_for_optimized).collect_vec();
    let winnable_games = all_games.into_iter().flatten();
    winnable_games.sum()
}

fn part_2(input: &str) -> i64 {
    let mut games = parse_input(input);
    for game in games.iter_mut() {
        game.px += 10000000000000;
        game.py += 10000000000000;
    }
    let all_games = games.into_iter().map(min_tokens_for_optimized).collect_vec();
    let winnable_games = all_games.into_iter().flatten();

    winnable_games.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 4), 2);
        assert_eq!(gcd(4, 10), 2);
        assert_eq!(gcd(1071, 462), 21);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(6, 21), 42);
    }

    #[test]
    fn test_solve_system() {
        let solution = solve_system(
            94,
            22,
            8400,
            34,
            67,
            5400,
        );

        assert_eq!(solution, Some((80, 40)));

        let solution = solve_system(
            1,
            2,
            6,
            2,
            4,
            12,
        );
        assert_eq!(solution, None);
    }
}
