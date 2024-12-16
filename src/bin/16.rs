use adv_code_2024::grid::{Direction, Grid, Position};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
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
###############
";

const TEST2: &str = "\
#################
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
#################
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let maze = read_maze(reader)?;

        match maze.dijkstra(
            State::new(maze.start_pos.clone(), Direction::East),
            &maze.end_pos,
        ) {
            Some(cost) => Ok(cost),
            None => Err(anyhow!("No solution found")),
        }
    }

    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let maze = read_maze(reader)?;

        match maze.dijkstra_with_all_paths(
            State::new(maze.start_pos.clone(), Direction::East),
            &maze.end_pos,
        ) {
            (Some(_), paths) => {
                //maze.print_with_paths(&paths);
                let positions = get_num_affected_positions(&paths);
                Ok(positions.len())
            }
            (None, _) => Err(anyhow!("No solution found")),
        }
    }

    assert_eq!(45, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    position: Position,
    facing: Direction,
}

impl State {
    fn new(position: Position, facing: Direction) -> Self {
        Self { position, facing }
    }

    fn next_states(&self) -> Vec<(State, usize)> {
        vec![
            (
                State::new(self.position.make_step(&self.facing), self.facing.clone()),
                1,
            ),
            (
                State::new(self.position.clone(), self.facing.turn_left()),
                1000,
            ),
            (
                State::new(self.position.clone(), self.facing.turn_right()),
                1000,
            ),
        ]
    }
}

type Path = Vec<State>;

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone)]
struct Maze {
    grid: Grid<Cell>,
    start_pos: Position,
    end_pos: Position,
}

impl Maze {
    fn new(grid: Grid<Cell>, start_pos: Position, end_pos: Position) -> Maze {
        Maze {
            grid,
            start_pos,
            end_pos,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let pos = Position::new(row, col);
                let cell = self.grid.value_at(&pos).unwrap();
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Wall => print!("#"),
                    Cell::Start => print!("S"),
                    Cell::End => print!("E"),
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_with_paths(&self, paths: &[Path]) {
        let positions = get_num_affected_positions(paths);
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let pos = Position::new(row, col);
                if positions.contains(&pos) {
                    print!("O");
                } else {
                    let cell = self.grid.value_at(&pos).unwrap();
                    match cell {
                        Cell::Empty => print!("."),
                        Cell::Wall => print!("#"),
                        Cell::Start => print!("S"),
                        Cell::End => print!("E"),
                    }
                }
            }
            println!();
        }
    }

    fn dijkstra_with_all_paths(&self, start: State, goal: &Position) -> (Option<usize>, Vec<Path>) {
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

        // Return the minimal cost and all paths to the goal.
        let goal_states = [Direction::East,
            Direction::West,
            Direction::North,
            Direction::South]
        .iter()
        .map(|direction| State::new(goal.clone(), direction.clone()))
        .filter(|st| distances.contains_key(st))
        .collect::<Vec<State>>();

        let min_goal_cost = goal_states
            .iter()
            .flat_map(|st| distances.get(st)).copied()
            .min();

        if min_goal_cost.is_none() {
            return (None, vec![]);
        }
        let min_goal_cost = min_goal_cost.unwrap();

        let min_goal_states = goal_states
            .into_iter()
            .filter(|st| *distances.get(st).unwrap() == min_goal_cost)
            .collect::<Vec<State>>();

        let mut all_paths = vec![];
        for goal_state in min_goal_states {
            all_paths.append(&mut paths.get(&goal_state).unwrap().clone());
        }

        (Some(min_goal_cost), all_paths)
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

    /// Placeholder for the `get_next_states` function.
    /// Replace this with your actual implementation.
    fn get_next_states(&self, state: &State) -> Vec<(State, usize)> {
        let mut ret = Vec::new();
        for (st, cost) in state.next_states() {
            if !self.grid.is_valid_position(&st.position) {
                continue;
            }
            let cell = self.grid.value_at(&st.position).unwrap();
            if let Cell::Wall = *cell {
                continue;
            }
            ret.push((st, cost));
        }

        ret
    }
}

fn get_num_affected_positions(paths: &[Path]) -> HashSet<Position> {
    let mut positions: HashSet<Position> = HashSet::new();
    for path in paths {
        for state in path {
            positions.insert(state.position.clone());
        }
    }
    positions
}

fn read_maze(reader: impl BufRead) -> Result<Maze> {
    let lines = read_lines(reader);
    let mut start_pos: Option<Position> = None;
    let mut end_pos: Option<Position> = None;
    let mut cells = Vec::new();

    for (row, line) in lines.into_iter().enumerate() {
        let mut cell_row = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => cell_row.push(Cell::Wall),
                'S' => {
                    cell_row.push(Cell::Start);
                    if start_pos.is_some() {
                        return Err(anyhow!("there can be only one start"));
                    }
                    start_pos = Some(Position::new(row as i32, col as i32));
                }
                'E' => {
                    cell_row.push(Cell::End);
                    if end_pos.is_some() {
                        return Err(anyhow!("there can be only one end"));
                    }
                    end_pos = Some(Position::new(row as i32, col as i32));
                }
                _ => cell_row.push(Cell::Empty),
            }
        }
        cells.push(cell_row);
    }

    let num_rows = cells.len() as i32;
    let num_cols = cells[0].len() as i32;
    let grid = Grid::new(num_rows, num_cols, cells);

    let maze = Maze::new(grid, start_pos.unwrap(), end_pos.unwrap());

    Ok(maze)
}
