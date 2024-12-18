use adv_code_2024::grid::{Direction, Grid, Position};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(
        reader: R,
        num_rows: usize,
        num_cols: usize,
        time: usize,
    ) -> Result<usize> {
        let memory = read_memory(reader, num_rows, num_cols);
        let start = State::new(Position::new(0, 0));
        let goal = Position::new((num_rows - 1) as i32, (num_cols - 1) as i32);

        memory
            .dijkstra(start, &goal, time)
            .ok_or(anyhow!("no solution"))
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 7, 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 71, 71, 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, num_rows: usize, num_cols: usize) -> Result<String> {
        let memory = read_memory(reader, num_rows, num_cols);
        let start = State::new(Position::new(0, 0));
        let goal = Position::new((num_rows - 1) as i32, (num_cols - 1) as i32);
        let mut time_min = 0;
        let mut time_max = memory.max_time;

        loop {
            if time_max - time_min <= 1 {
                let blocking_pos = memory.find_cell(time_max).unwrap();
                return Ok(format!("{},{}", blocking_pos.col(), blocking_pos.row()));
            }

            let time = (time_min + time_max) / 2;
            match memory.dijkstra(start.clone(), &goal, time) {
                Some(_) => {
                    time_min = time;
                }
                None => {
                    time_max = time;
                }
            }
        }
    }

    assert_eq!("6,1", part2(BufReader::new(TEST.as_bytes()), 7, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 71, 71)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    position: Position,
}

impl State {
    fn new(position: Position) -> Self {
        Self { position }
    }

    fn next_states(&self) -> Vec<(State, usize)> {
        use Direction::*;

        [North, South, East, West]
            .iter()
            .map(|d| (State::new(self.position.make_step(&d)), 1))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
struct Memory {
    grid: Grid<usize>,
    max_time: usize,
}

impl Memory {
    fn new(grid: Grid<usize>, max_time: usize) -> Self {
        Self { grid, max_time }
    }

    #[allow(dead_code)]
    fn print(&self, time: usize) {
        for y in 0..self.grid.num_rows {
            for x in 0..self.grid.num_cols {
                let value = self.grid.value_at(&Position::new(y, x)).unwrap();
                if *value == 0 || *value > time {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    fn find_cell(&self, time: usize) -> Option<Position> {
        for y in 0..self.grid.num_rows {
            for x in 0..self.grid.num_cols {
                let value = self.grid.value_at(&Position::new(y, x)).unwrap();
                if *value == time {
                    return Some(Position::new(y, x));
                }
            }
        }
        None
    }

    fn dijkstra(&self, start: State, goal: &Position, time: usize) -> Option<usize> {
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
            for (next_state, additional_cost) in self.get_next_states(&current_state, time) {
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
    fn get_next_states(&self, state: &State, time: usize) -> Vec<(State, usize)> {
        let mut ret = Vec::new();
        for (st, cost) in state.next_states() {
            if !self.grid.is_valid_position(&st.position) {
                continue;
            }
            let value = self.grid.value_at(&st.position).unwrap();
            if *value == 0 || *value > time {
                ret.push((st, cost));
            }
        }

        ret
    }
}

fn read_memory<R: BufRead>(reader: R, num_rows: usize, num_cols: usize) -> Memory {
    let mut cells = vec![vec![0_usize; num_cols]; num_rows];
    let lines = read_lines(reader);

    for (t, line) in lines.iter().enumerate() {
        let coords = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        cells[coords[1]][coords[0]] = t + 1;
    }

    let grid = Grid::new(num_rows as i32, num_cols as i32, cells);

    Memory::new(grid, lines.len())
}
