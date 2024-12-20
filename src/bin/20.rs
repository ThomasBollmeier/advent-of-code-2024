use std::cmp::{PartialEq, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;
use adv_code_2024::grid::{Grid, Position, Direction};
use adv_code_2024::grid::Direction::{East, North, South, West};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let racemap = read_race_map(reader)?;
        let orig_time = racemap.fatest_time().unwrap();
        
        let mut stats = HashMap::new();
        
        for (start, end) in &racemap.find_possible_cheats() {
            let mut cheat_map = racemap.clone();
            cheat_map.remove_wall(start);
            cheat_map.remove_wall(end);
            let difference = match cheat_map.fatest_time() {
                Some(time) => orig_time - time,
                None => continue,
            };
            
            stats.entry(difference)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        
        dbg!(stats);
        
        match racemap.fatest_time() {
            Some(time) => Ok(time),
            None => Err(anyhow!("No path found in race map")),
        }
    }

    assert_eq!(84, part1(BufReader::new(TEST.as_bytes()))?);

    //let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part1(input_file)?);
    //println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    position: Position,
    remaining_cheats: usize,
}

impl State {
    fn new(position: Position,) -> Self {
        Self { position, remaining_cheats: 2 }
    }
}

type Path = Vec<State>;

#[derive(Debug, Clone)]
struct RaceMap {
    grid: Grid<Cell>,
    start: Position,
    end: Position,
}

impl RaceMap {
    fn new(grid: Grid<Cell>, start: Position, end: Position) -> Self {
        Self { grid, start, end }
    }
    
    fn find_possible_cheats(&self) -> Vec<(Position, Position)> {
        let mut ret = Vec::new();
        
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let start = Position::new(row, col);
                
                let start_value = self.grid.value_at(&start).unwrap();
                for dir in &[North, South, East, West] {
                    let end = start.make_step(dir);
                    if !self.grid.is_valid_position(&end) {
                        continue;
                    }
                    let end_value = self.grid.value_at(&end).unwrap();
                    match *start_value {
                        Cell::Empty => {
                            if *end_value == Cell::Empty {
                                continue;
                            }
                        }
                        Cell::Wall => {}
                    }
                    ret.push((start.clone(), end));
                }
            }
        }
        
        ret
    }
    
    fn remove_wall(&mut self, position: &Position) {
        if !self.grid.is_valid_position(position) {
            return;
        }
        self.grid.set_value_at(position, Cell::Empty);
    }
    
    fn fatest_paths(&self) -> Vec<Path> {
        self.dijkstra_with_all_paths(State::new(self.start.clone()), &self.end)
    }
    
    fn fatest_time(&self) -> Option<usize> {
        self.dijkstra(State::new(self.start.clone()), &self.end)
    }

    fn dijkstra_with_all_paths(&self, start: State, goal: &Position) -> Vec<Path> {
        let mut priority_queue = BinaryHeap::new();
        // A HashMap to store the minimum cost to reach each position.
        let mut distances: HashMap<State, usize> = HashMap::new();
        // A HashMap to store all minimal-cost paths to each position.
        let mut paths: HashMap<State, Vec<Path>> = HashMap::new();

        // Initialize the start state with a cost of 0 and an initial path.
        distances.insert(start.clone(), 0);
        paths.insert(start.clone(), vec![vec![start.clone()]]);
        priority_queue.push(Reverse((0, start.clone(), vec![start.clone()])));

        while let Some(Reverse((current_cost, current_state, current_path))) = priority_queue.pop()
        {
            // If we already processed this state with a lower cost, skip it.
            if current_cost > *distances.get(&current_state).unwrap_or(&usize::MAX) {
                continue;
            }

            // If we reached the goal, continue collecting paths.
            if current_state.position == *goal {
                paths
                    .entry(current_state.clone())
                    .or_default()
                    .push(current_path.clone());
                continue;
            }

            // Get next states and their costs.
            for (next_state, additional_cost) in self.get_next_states(&current_state) {
                let new_cost = current_cost + additional_cost;
                let prev_cost = distances.get(&next_state).cloned().unwrap_or(usize::MAX);

                if new_cost <= prev_cost {
                    if new_cost < prev_cost {
                        // Found a cheaper path, update the distance and reset paths.
                        distances.insert(next_state.clone(), new_cost);
                        paths.insert(next_state.clone(), vec![]);
                    }

                    // Add this path to the list of minimal-cost paths for `next_state`.
                    let mut new_path = current_path.clone();
                    new_path.push(next_state.clone());
                    paths
                        .entry(next_state.clone())
                        .or_default()
                        .push(new_path.clone());

                    // Push the next state into the priority queue.
                    priority_queue.push(Reverse((new_cost, next_state, new_path)));
                }
            }
        }

        // Return all paths to the goal.
        let goal_state = State::new(goal.clone());
        
        paths.get(&goal_state).unwrap_or(&vec![]).clone()
    }

    fn dijkstra(&self, start: State, goal: &Position) -> Option<usize> {
        let mut priority_queue = BinaryHeap::new();
        let mut distances: HashMap<State, usize> = HashMap::new();

        // Initialize the start state with a cost of 0.
        distances.insert(start.clone(), 0);
        priority_queue.push(Reverse((0, start)));

        while let Some(Reverse((current_cost, current_state))) = priority_queue.pop() {
            // If we reached the goal, return the cost.
            if current_state.position == *goal {
                return Some(current_cost);
            }

            // Skip processing if we already found a cheaper way to this state.
            if current_cost > *distances.get(&current_state).unwrap_or(&usize::MAX) {
                continue;
            }

            // Get next states and their costs.
            for (next_state, additional_cost) in self.get_next_states(&current_state) {
                let new_cost = current_cost + additional_cost;

                // If this path to `next_state` is cheaper, record it and add to the queue.
                if new_cost < *distances.get(&next_state).unwrap_or(&usize::MAX) {
                    distances.insert(next_state.clone(), new_cost);
                    priority_queue.push(Reverse((new_cost, next_state)));
                }
            }
        }

        // If we exhaust the queue without finding the goal, return None.
        None
    }
    
    fn get_next_states(&self, state: &State) -> Vec<(State, usize)> {
        
        [North, South, East, West]
            .iter()
            .map(|dir| {
                State::new(state.position.make_step(dir))
            })
            .filter(|state| {
                self.grid.is_valid_position(&state.position) &&
                    *self.grid.value_at(&state.position).unwrap() != Cell::Wall
            })
            .map(|state| (state, 1))
            .collect::<Vec<_>>()
    }

}

fn read_race_map(reader: impl BufRead) -> Result<RaceMap> {
    let lines = read_lines(reader);
    let mut start_pos: Option<Position> = None;
    let mut end_pos: Option<Position> = None;
    let mut cells = Vec::new();

    for (row, line) in lines.into_iter().enumerate() {
        let mut cell_row = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => cell_row.push(Cell::Wall),
                _ => {
                    cell_row.push(Cell::Empty);
                    match ch {
                        'S' => {
                            if start_pos.is_some() {
                                return Err(anyhow!("there can be only one start"));
                            }
                            start_pos = Some(Position::new(row as i32, col as i32));        
                        }
                        'E' => {
                            if end_pos.is_some() {
                                return Err(anyhow!("there can be only one end"));
                            }
                            end_pos = Some(Position::new(row as i32, col as i32));
                        }
                        _ => {}
                    }
                }
            }
        }
        cells.push(cell_row);
    }

    let num_rows = cells.len() as i32;
    let num_cols = cells[0].len() as i32;
    let grid = Grid::new(num_rows, num_cols, cells);

    Ok(RaceMap::new(grid, start_pos.unwrap(), end_pos.unwrap()))
}