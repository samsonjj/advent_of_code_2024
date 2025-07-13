use advent_of_code_2024::aoc;
use std::collections::HashSet;

struct LabSim {
    states: Vec<Vec<char>>, 
    visited: HashSet<(i32, i32)>,
    guard: (i32, i32),
}

impl LabSim {
    fn from_input(input: &str) -> Self {
        let mut states = vec![];
        let mut guard = (0, 0);
        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                row.push(c);
                if c == '^' {
                    guard = (j as i32, i as i32);
                }
            }
            states.push(row);
        }
        Self {
            states,
            visited: HashSet::new(),
            guard,
        }
    }

    fn run_loop_sim(&mut self) -> i32 {
        let mut sum = 0;

        // optimiziation: we only need to attemp to insert an obstruction in
        // the original path of the guard. All other positionsn will not
        // result in any changes to the guard's path.
        self.run_sim();
        let candidates = self.visited.iter().map(|item| item.clone()).collect::<Vec<(i32, i32)>>();
        for candidate in candidates.iter() {
            let (i, j) = (candidate.0 as usize, candidate.1 as usize);
            // don't run if guard is there or obstructed
            if self.states[j][i] != '.'{
                continue;
            }

            // set obstruction
            self.states[j][i] = '#';

            // run guard sim
            if self.run_sim() {
                sum += 1;
            }

            // reset
            self.states[j][i] = '.';
        }
        sum
    }

    // returns true if is loop
    fn run_sim(&mut self) -> bool {
        let mut position = self.guard;
        let mut direction: (i32, i32)  = (0i32, -1i32); 
        let width = self.width();
        let height = self.height();

        // key is (position, direction)
        let mut loop_states: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        loop {
            self.visited.insert(position);
            let in_loop = !loop_states.insert((position, direction));
            if in_loop {
                return true;
            }

            // potential_position
            let pp: (i32, i32) = (position.0 + direction.0, position.1 + direction.1);
            let in_bounds = (0..width).contains(&pp.0) && (0..height).contains(&pp.1);
            if !in_bounds {
                break;
            }

            let obstructed = self.states[pp.1 as usize][pp.0 as usize] == '#';
            if obstructed {
                direction = next_direction(direction);
                continue;
            }
            
            position = pp;
        }
        false
    }

    fn width(&self) -> i32 {
        self.states[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.states.len() as i32
    }

    fn count_visited(&self) -> i32 {
        self.visited.len() as i32
    }
}

fn next_direction(curr_dir: (i32, i32)) -> (i32, i32) {
    match curr_dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("part_1: {}", aoc::format_with_time(|| part_1(input)));
    println!("part_2: {}", aoc::format_with_time(|| part_2(input)));
}



fn part_1(input: &str) -> i32 {
    let mut lab_sim = LabSim::from_input(input);
    lab_sim.run_sim();
    lab_sim.count_visited()
}

fn part_2(input: &str) -> i32 {
    let mut lab_sim = LabSim::from_input(input);
    lab_sim.run_loop_sim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_direction() {
        assert_eq!(next_direction((0, 1)), (-1, 0));
        assert_eq!(next_direction((0, -1)), (1, 0));
        assert_eq!(next_direction((1, 0)), (0, 1));
        assert_eq!(next_direction((-1, 0)), (0, -1));
    }
}
