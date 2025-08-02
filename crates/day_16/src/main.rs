#![allow(unreachable_code)]

use advent_of_code_2024::aoc;
use itertools::Itertools;
use std::collections::{HashMap, BinaryHeap, HashSet};


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_delta(delta: (i64, i64)) -> Self {
        match delta {
            (-1, 0) => Direction::North,
            (1, 0) => Direction::South,
            (0, -1) => Direction::West,
            (0, 1) => Direction::East,
            x => panic!("{x:?}"),
        }
    }
    fn deltas(&self) -> (i64, i64) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
    fn turnC(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
    fn turnCC(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }
}

impl std::ops::Add<(usize, usize)> for Direction {
    type Output = (usize, usize);
    fn add(self, other: (usize, usize)) -> (usize, usize) {
        let (dx, dy) = self.deltas();
        let x = (other.0 as i64 + dx) as usize;
        let y = (other.1 as i64 + dy) as usize;
        (x, y)
    }
}

impl Direction {
    fn sub_from(self, other: (usize, usize)) -> (usize, usize) {
        let (dx, dy) = self.deltas();
        let x = (other.0 as i64 - dx) as usize;
        let y = (other.1 as i64 - dy) as usize;
        (x, y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

pub mod state_cost {
    use std::cmp::{Ordering, PartialEq, Eq, PartialOrd, Ord};
    use super::Direction;
    use super::State;

    const TURN_COST: i64 = 1000;
    const STEP_COST: i64 = 1;

    #[derive(Clone, Debug, PartialEq)]
    pub struct StateCost {
        pub state: State,
        pub cost: i64,
    }

    impl StateCost {
        pub fn new(pos: (usize, usize), dir: Direction, cost: i64) -> Self {
            Self {
                state: State {
                    pos,
                    dir,
                },
                cost,
            }
        }
        pub fn turnC(&self) -> Self {
            let mut result = self.clone();
            result.cost += TURN_COST;
            result.state.dir.turnC();
            result
        }
        pub fn turnCC(&self) -> Self {
            let mut result = self.clone();
            result.cost += TURN_COST;
            result.state.dir.turnCC();
            result
        }
        pub fn step(&self) -> Self {
            let mut result = self.clone();
            result.cost += STEP_COST;
            result.state.pos = self.state.dir + result.state.pos;
            result
        }
    }

    // Ordered strictly by cost
    impl Ord for StateCost {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for StateCost {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    
    impl Eq for StateCost {}
}

use state_cost::StateCost;

struct Maze {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

#[derive(Clone, Debug)]
struct Edge {
    prev: State,
    next: State,
}

#[derive(Clone, Debug)]
struct EdgeCost {
    edge: Edge,
    cost: i64,
}

const TURN_COST: i64 = 1000;
const STEP_COST: i64 = 1;

impl EdgeCost {

    pub fn turnC(&self) -> Self {
        let mut result = self.clone();
        result.cost += TURN_COST;
        result.edge.prev = result.edge.next;
        result.edge.next.dir.turnC();
        result
    }
    pub fn turnCC(&self) -> Self {
        let mut result = self.clone();
        result.cost += TURN_COST;
        result.edge.prev = result.edge.next;
        result.edge.next.dir.turnCC();
        result
    }
    pub fn step(&self) -> Self {
        let mut result = self.clone();
        result.cost += STEP_COST;
        result.edge.prev = result.edge.next;
        result.edge.next.pos = result.edge.next.dir + result.edge.next.pos;
        result
    }
}

type EdgeCostStore = HashMap<State, Vec<EdgeCost>>;

impl Maze {
    fn get(&self, pos: (usize, usize)) -> char {
        self.map[pos.0][pos.1]
    }

    fn search2(&self) -> (i64, EdgeCostStore) {
        let mut paths: EdgeCostStore = HashMap::new();
        let mut queue: Vec<EdgeCost> = vec![];

        // init start state
        let start_state = State { pos: self.start, dir: Direction::East };
        queue.push(
            EdgeCost { edge: Edge { prev: start_state, next: start_state }, cost: 0 },
        );

        loop {
            queue.sort_by(|a, b| b.cost.cmp(&a.cost));
            let edge_cost = queue.pop().unwrap();
            let EdgeCost { edge: Edge { prev, next }, cost } = edge_cost;

            // visited
            if let Some(edges) = paths.get(&next) {
                let acceptable_cost = edges.first().unwrap().cost;
                if cost == acceptable_cost {
                    paths.get_mut(&next).unwrap().push(edge_cost.clone());
                }
                continue; // TODO
            }
            
            // unvisited
            // dbg!(&edge_cost);
            paths.insert(next, vec![edge_cost.clone()]);

            if self.get(next.pos) == 'E' {
                return (cost, paths);
            }

            let next_edge_costs = vec![
                edge_cost.turnC(),
                edge_cost.turnCC(),
                edge_cost.step(),
            ].into_iter()
                .filter(|edge_cost| !paths.contains_key(&edge_cost.edge.next))
                .filter(|edge_cost| self.get(edge_cost.edge.next.pos) != '#')
                .collect_vec();

            for edge_cost in next_edge_costs.into_iter() {
                dbg!(&edge_cost);
                queue.push(edge_cost);
            }
        }
    }

    fn search(
        &self,
    ) -> (i64, HashMap<(usize, usize), Direction>, HashMap<(usize, usize), Vec<(usize, usize)>>, (usize, usize)) {
        // map of state -> cost
        let mut visited: HashMap<State, i64> = HashMap::new();
        let mut queue: BinaryHeap<StateCost> = BinaryHeap::new();
        let mut paths: HashMap<(usize, usize), Direction> = HashMap::new();
        // store the preceeding tiles which can reach each tile via the same cost
        let mut paths2: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        let start_state = State { pos: self.start, dir: Direction::East };
        let start_state_cost = StateCost { state: start_state, cost: 0 };
        queue.push(start_state_cost);

        loop {
            let curr_state = queue.pop().unwrap();

            if visited.contains_key(&curr_state.state) {
                let prev_cost = visited.get(&curr_state.state).unwrap();
                // alternative path with same cost
                if *prev_cost == curr_state.cost {
                    let entry = paths2.entry(curr_state.state.pos).or_insert_with(|| panic!());
                    (*entry).push(curr_state.state.dir.sub_from(curr_state.state.pos));
                }
                continue;
            }

            visited.insert(curr_state.state, curr_state.cost);
            paths.insert(curr_state.state.pos, curr_state.state.dir);
            let entry = paths2.entry(curr_state.state.pos).or_insert(vec![]);
            (*entry).push(curr_state.state.dir.sub_from(curr_state.state.pos));


            if self.get(curr_state.state.pos) == 'E' {
                return (curr_state.cost, paths, paths2, curr_state.state.pos);
            }

            let next_states = vec![
                curr_state.turnC(),
                curr_state.turnCC(),
                curr_state.step(),
            ].into_iter()
                .filter(|state_cost| !visited.contains_key(&state_cost.state))
                .filter(|state_cost| self.get(state_cost.state.pos) != '#')
                .collect_vec();

            for state_cost in next_states.into_iter() {
                queue.push(state_cost.clone());
            }
        }
    }

    fn count_seats(&self, state: State, paths: &EdgeCostStore, seats: &mut HashSet<(usize, usize)>, i: i64) {
        let edge_costs: &Vec<EdgeCost> = paths.get(&state).unwrap();
        //dbg!(&edge_costs[0].edge.next.pos, &edge_costs[0].edge.next.dir);
        //dbg!(&edge_costs[0].edge.prev.pos, &edge_costs[0].edge.prev.dir);
        for edge_cost in edge_costs.iter() {
            seats.insert(edge_cost.edge.next.pos);
            seats.insert(edge_cost.edge.prev.pos);
            if edge_cost.edge.prev.pos == self.start {
                continue;
            }
            self.count_seats(edge_cost.edge.prev, paths, seats, i + 1);
        }
    }
}

fn parse_input(input: &str) -> Maze {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'S' {
                return Maze {
                    map,
                    start: (row, col),
                };
            }
        }
    }
    unreachable!()
}

fn display(maze: &Maze, paths: &HashMap<(usize, usize), Direction>) {
    for i in 0..maze.map.len() {
        for j in 0..maze.map[i].len() {
            if let Some(dir) = paths.get(&(i, j)) {
                print!("{}", match dir {
                    Direction::North => 'v',
                    Direction::East => '<',
                    Direction::West => '>',
                    Direction::South => '^',
                });
            } else {
                print!("{}", maze.map[i][j]);
            }
        }
        println!();
    }
}

fn display2(maze: &Maze, paths2: &EdgeCostStore) {
    let mut paths: HashMap<(usize, usize), Direction> = HashMap::new();
    for (state, edge_costs) in paths2.iter() {
        for edge_cost in edge_costs.iter() {
            let pos = edge_cost.edge.next.pos;
            let prev = edge_cost.edge.prev.pos;
            let delta = ((pos.0 - prev.0) as i64, (pos.1 - prev.1) as i64);

            // diff between states can be zero if it was a turn
            if delta == (0, 0) {
                continue;
            }

            let direction = Direction::from_delta(delta);
            paths.insert(pos, direction);
        }
    }
    display(maze, &paths);
}

fn main() {
    let input = include_str!("input.txt");
    aoc::run_parts(input, part_1, part_2);
}

fn part_1(input: &str) -> i64 {
    let maze = parse_input(input);
    let (result, paths, _paths2, _end) = maze.search();
    display(&maze, &paths);
    result
}

fn part_2(input: &str) -> i64 {
    let maze = parse_input(input);
    let (result, paths) = maze.search2();
    display2(&maze, &paths);
    let mut seats = HashSet::new();
    // dbg!(&paths);
    dbg!(&paths.len());
    // maze.count_seats((1, 139), &paths, &mut seats, 0);
    maze.count_seats(State { pos: (1, 139), dir: Direction::North }, &paths, &mut seats, 0);
    // maze.count_seats(State { pos: (1, 15), dir: Direction::North }, &paths, &mut seats, 0);
    seats.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let result = part_1(input);
        assert_eq!(result, 11048);
    }

    #[test]
    fn test_2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let result = part_1(input);
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_binary_heap() {
        let mut binary_heap = BinaryHeap::new();
        binary_heap.push(StateCost::new((1, 0), Direction::East, 10));
        binary_heap.push(StateCost::new((2, 0), Direction::East, 10));
        binary_heap.push(StateCost::new((3, 0), Direction::East, 9));
        binary_heap.push(StateCost::new((4, 0), Direction::East, 10));
        binary_heap.push(StateCost::new((4, 0), Direction::East, 9));
        binary_heap.push(StateCost::new((4, 0), Direction::East, 8));
        binary_heap.push(StateCost::new((4, 0), Direction::East, 7));

        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((4, 0), Direction::East, 7));
        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((4, 0), Direction::East, 8));
        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((3, 0), Direction::East, 9));
        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((4, 0), Direction::East, 9));
        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((1, 0), Direction::East, 10));
        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((4, 0), Direction::East, 10));
        assert_eq!(binary_heap.pop().unwrap(), StateCost::new((2, 0), Direction::East, 10));
    }
}
