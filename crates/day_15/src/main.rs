use advent_of_code_2024::aoc;
use itertools::Itertools;

#[derive(Clone, Debug, Copy, PartialEq)]
enum Spot {
    Wall,
    Box,
    LBox,
    RBox,
    Empty,
}

#[derive(Clone, Debug)]
struct Game {
    map: Vec<Vec<Spot>>,
    robot: (i32, i32),
    moves: Vec<(i32, i32)>,
}

impl Game {
    fn print(&self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.robot == (row as i32, col as i32) {
                    print!("@");
                    continue; 
                }
                print!("{}", match self.map[row][col] {
                    Spot::Empty => '.',
                    Spot::Box=> 'O',
                    Spot::LBox=> '[',
                    Spot::RBox=> ']',
                    Spot::Wall => '#',
                });
            }
            println!();
        }
        println!();
    }
    fn run_sim(&mut self) {
        // self.print();
        for i in 0..self.moves.len() {
            if self.push(self.robot, self.moves[i], false) {
                self.push(self.robot, self.moves[i], true);
            }
            // println!("Move: {}", match self.moves[i] {
            //     (1, 0) => 'v',
            //     (-1, 0) => '^',
            //     (0, 1) => '>',
            //     (0, -1) => '<',
            //     _ => unreachable!(),
            // });
            // self.print();
        }
    }

    fn widen(mut self) -> Self {
        self.map = self.map.into_iter().map(|row| {
            row.into_iter().map(|spot| {
                match spot {
                    Spot::Empty => vec![Spot::Empty, Spot::Empty],
                    Spot::Wall => vec![Spot::Wall, Spot::Wall],
                    Spot::Box => vec![Spot::LBox, Spot::RBox],
                    _ => unreachable!(),
                }
            })
        }).map(|row| row.flatten().collect_vec()).collect_vec();
        self.robot = (self.robot.0, self.robot.1 * 2);
        self
    }

    fn get(&self, pos: (i32, i32)) -> Spot {
        self.map[pos.0 as usize][pos.1 as usize]
    }

    // returns true if pushing the box was successful
    fn push(&mut self, pos: (i32, i32), direction: (i32, i32), execute: bool) -> bool {
        let (dy, dx) = direction;
        let next_pos = (pos.0 + dy, pos.1 + dx);
        let next_spot = self.map[next_pos.0 as usize][next_pos.1 as usize];
        let can_move = match next_spot {
            Spot::Empty => true,
            Spot::Box => self.push(next_pos, direction, execute),
            Spot::LBox =>
                if direction.0 == 0 {
                // horizontal
                    self.push(next_pos, direction, execute)
                } else {
                // vertical
                    let adjacent = if self.get(next_pos) == Spot::LBox { (next_pos.0, next_pos.1 + 1) } else { (next_pos.0, next_pos.1 - 1) };
                    self.push(next_pos, direction, execute)
                        && self.push(adjacent, direction, execute)
                }
            Spot::RBox =>
                if direction.0 == 0 {
                // horizontal
                    self.push(next_pos, direction, execute)
                } else {
                // vertical
                    let adjacent = if self.get(next_pos) == Spot::LBox { (next_pos.0, next_pos.1 + 1) } else { (next_pos.0, next_pos.1 - 1) };
                    // println!("hello, ({}, {})|({}, {})", pos.0, pos.1, adjacent.0, adjacent.1);
                    self.push(next_pos, direction, execute)
                        && self.push(adjacent, direction, execute)
                }
            Spot::Wall => false,
        };
        // println!("({}, {})", pos.0, pos.1);
        if can_move && execute {
            if pos == self.robot {
                self.robot = next_pos;
            }
            self.map[next_pos.0 as usize][next_pos.1 as usize]
                = self.map[pos.0 as usize][pos.1 as usize];
            self.map[pos.0 as usize][pos.1 as usize] = Spot::Empty;
        }
        can_move
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col] == Spot::Box || self.map[row][col] == Spot::LBox {
                    score += (row * 100 + col) as i32;
                }
            }
        }
        score
    }
}

fn parse_input(input: &str) -> Game {
    let mut parts = input.split("\n\n");
    let map_str = parts.next().unwrap();
    let moves_str = parts.next().unwrap();

    let mut robot = (0, 0);
    let map = map_str.lines().enumerate().map(|(row, line)| line.chars().enumerate().map(|(col, c)| {
        if c == '@' {
            robot = (row as i32, col as i32); // row, col
            return Spot::Empty;
        }
        match c {
            '#' => Spot::Wall,
            '.' => Spot::Empty,
            'O' => Spot::Box,
            _ => unreachable!(),
        }
    }).collect_vec()).collect_vec(); 

    let moves = moves_str.replace('\n', "").trim().chars().map(|c|
        match c {
            'v' => (1, 0), // drow, dcol
            '^' => (-1, 0), // drow, dcol
            '>' => (0, 1), // drow, dcol
            '<' => (0, -1), // drow, dcol
            _ => unreachable!(),
        }
    ).collect_vec();

    Game {map, robot, moves }
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i32 {
    let mut game = parse_input(input);
    game.run_sim();
    // game.print();
    game.score()
}

fn part_2(input: &str) -> i32 {
    let mut game = parse_input(input);
    game = game.widen();
    game.run_sim();
    // game.print();
    game.score()
}
