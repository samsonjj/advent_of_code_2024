use advent_of_code_2024::aoc;
use itertools::Itertools;

#[derive(Clone, Debug, Copy, PartialEq)]
enum Spot {
    Wall,
    Box,
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
                    Spot::Wall => '#',
                });
            }
            println!();
        }
        println!();
    }
    fn run_sim(&mut self) {
        for i in 0..self.moves.len() {
            self.push(self.robot, self.moves[i]);
        }
    }

    // returns true if pushing the box was successful
    fn push(&mut self, pos: (i32, i32), direction: (i32, i32)) -> bool {
        let (dy, dx) = direction;
        let next_pos = (pos.0 + dy, pos.1 + dx);
        let next_spot = self.map[next_pos.0 as usize][next_pos.1 as usize];
        match next_spot {
            Spot::Empty => {
                if pos == self.robot {
                    self.robot = next_pos;
                }
                self.map[next_pos.0 as usize][next_pos.1 as usize] = Spot::Box;
                self.map[pos.0 as usize][pos.1 as usize] = Spot::Empty;
                true
            }
            Spot::Box => {
                if self.push(next_pos, direction) {
                    if pos == self.robot {
                        self.robot = next_pos;
                    }
                    self.map[next_pos.0 as usize][next_pos.1 as usize]
                        = self.map[pos.0 as usize][pos.1 as usize];
                    self.map[pos.0 as usize][pos.1 as usize] = Spot::Empty;
                    true
                } else {
                    false
                }
            }
            Spot::Wall => {
                false
            }
        }
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col] != Spot::Box || self.robot == (row as i32, col as i32) {
                    continue;
                }
                score += (row * 100 + col) as i32;
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
    game.print();
    game.score()
}

fn part_2(input: &str) -> i32 {
    0
}
