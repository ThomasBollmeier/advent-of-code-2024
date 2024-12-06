use std::collections::{HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let area = Area::from_input(reader)?;
        let (num_visited, _) = area.guard_walk(None);
        Ok(num_visited)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let area = Area::from_input(reader)?;
        let mut total = 0;
        for row in 0..area.height {
            for col in 0..area.width {
                if area.obstructions.contains(&(row, col)) {
                    continue;
                }
                if area.guard == (row, col) {
                    continue;
                }
                let mut obstructions_per_row = area.obstructions_per_row.clone();
                let cols = obstructions_per_row.entry(row).or_default();
                cols.push(col);
                let mut obstructions_per_col = area.obstructions_per_col.clone();
                let rows = obstructions_per_col.entry(col).or_default();
                rows.push(row);

                if is_loop(&area.guard, &area.guard_direction, &obstructions_per_row, &obstructions_per_col) {
                    total += 1;
                }
            }
        }
        
        Ok(total)
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Position = (i32, i32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self) -> &Direction {
        match self {
            Direction::North => &Direction::East,
            Direction::South => &Direction::West,
            Direction::East => &Direction::South,
            Direction::West => &Direction::North,
        }
    }

    fn step(&self, pos: &Position) -> Position {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }

    fn step_back(&self, pos: &Position) -> Position {
        match self {
            Direction::North => (pos.0 + 1, pos.1),
            Direction::South => (pos.0 - 1, pos.1),
            Direction::East => (pos.0, pos.1 - 1),
            Direction::West => (pos.0, pos.1 + 1),
        }
    }
}



#[derive(Debug)]
struct Area {
    width: i32,
    height: i32,
    guard: Position,
    guard_direction: Direction,
    obstructions: HashSet<Position>,
    obstructions_per_row: HashMap<i32, Vec<i32>>,
    obstructions_per_col: HashMap<i32, Vec<i32>>,
}

impl Area {
    fn from_input<R: BufRead>(reader: R) -> Result<Area> {
        let lines = read_lines(reader);

        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        let mut guard: Option<Position> = None;
        let mut guard_direction: Option<Direction> = None;
        let mut obstructions: HashSet<Position> = HashSet::new();
        let mut obstructions_per_row: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut obstructions_per_col: HashMap<i32, Vec<i32>> = HashMap::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        obstructions.insert((row as i32, col as i32));
                        let mut entry = obstructions_per_row.entry(row as i32).or_default();
                        entry.push(col as i32);
                        entry = obstructions_per_col.entry(col as i32).or_default();
                        entry.push(row as i32);
                    }
                    '<' | '>' | 'v' | '^' => {
                        guard = Some((row as i32, col as i32));
                        guard_direction = match ch {
                            '<' => Some(Direction::West),
                            '>' => Some(Direction::East),
                            'v' => Some(Direction::South),
                            '^' => Some(Direction::North),
                            _ => None,
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(guard) = guard {
            Ok(Area{
                height,
                width,
                guard,
                guard_direction: guard_direction.unwrap(),
                obstructions,
                obstructions_per_row,
                obstructions_per_col,
            })
        } else {
            Err(anyhow!("Guard was not found"))
        }
    }

    fn guard_walk(&self, additional_obstruction: Option<Position>) -> (usize, bool) {
        let mut visited = HashSet::new();
        let mut visited_states = HashSet::new();
        let mut curr_pos = self.guard;
        let mut curr_dir = &self.guard_direction;

        let mut obstructions = self.obstructions.clone();
        if let Some(obstruction) = additional_obstruction {
            obstructions.insert(obstruction);
        }

        loop {
            if visited_states.contains(&(curr_pos, curr_dir)) {
                return (visited.len(), true);
            }
            visited.insert(curr_pos);
            visited_states.insert((curr_pos, curr_dir));

            let next_pos = curr_dir.step(&curr_pos);
            if !(0..self.width).contains(&next_pos.1) || !(0..self.height).contains(&next_pos.0) {
                return (visited.len(), false);
            }

            if !obstructions.contains(&next_pos) {
                curr_pos = next_pos;
            } else {
                curr_dir = curr_dir.turn()
            }
        }
    }
}

fn is_loop(start_pos: &Position,
           start_dir: &Direction,
           obstacles_per_row: &HashMap<i32, Vec<i32>>,
           obstacles_per_col: &HashMap<i32, Vec<i32>>) -> bool {

    let mut state = (*start_pos, start_dir.clone());
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();

    loop {
        if visited.contains(&state) {
            return true;
        }
        visited.insert(state.clone());
        let state_opt = next_state(&state.0,
                                   &state.1,
                                   obstacles_per_row,
                                   obstacles_per_col);
        if let Some(st) = state_opt {
            state = st;
        } else {
            return false;
        }
    }
}

fn next_state(pos: &Position,
              dir: &Direction,
              obstacles_per_row: &HashMap<i32, Vec<i32>>,
              obstacles_per_col: &HashMap<i32, Vec<i32>>
) -> Option<(Position, Direction)> {
    let next_obstacle = get_next_obstacle(pos, dir, obstacles_per_row, obstacles_per_col);
    next_obstacle.map(|pos| (dir.step_back(&pos), dir.turn().clone()))
}

fn get_next_obstacle(pos: &Position,
                     dir: &Direction,
                     obstacles_per_row: &HashMap<i32, Vec<i32>>,
                     obstacles_per_col: &HashMap<i32, Vec<i32>>
) -> Option<Position> {
    match dir {
        Direction::North => {
            let obstacle_rows = obstacles_per_col.get(&pos.1);
            if let Some(obstacle_rows) = obstacle_rows {
                let mut obstacle_rows = obstacle_rows.clone();
                obstacle_rows.sort();
                let mut row_opt: Option<i32> = None;
                for row in obstacle_rows.iter() {
                    if *row < pos.0 {
                        row_opt = Some(*row);
                    } else {
                        break
                    }
                }
                row_opt.map(|row| (row, pos.1))
            } else {
                None
            }
        }
        Direction::South => {
            let obstacle_rows = obstacles_per_col.get(&pos.1);
            if let Some(obstacle_rows) = obstacle_rows {
                let mut obstacle_rows = obstacle_rows.clone();
                obstacle_rows.sort();
                let mut row_opt: Option<i32> = None;
                for row in obstacle_rows.iter().rev() {
                    if *row > pos.0 {
                        row_opt = Some(*row);
                    } else {
                        break
                    }
                }
                row_opt.map(|row| (row, pos.1))
            } else {
                None
            }
        }
        Direction::East => {
            let obstacle_cols = obstacles_per_row.get(&pos.0);
            if let Some(obstacle_cols) = obstacle_cols {
                let mut obstacle_cols = obstacle_cols.clone();
                obstacle_cols.sort();
                let mut col_opt: Option<i32> = None;
                for col in obstacle_cols.iter().rev() {
                    if *col > pos.1 {
                        col_opt = Some(*col);
                    } else {
                        break
                    }
                }
                col_opt.map(|col| (pos.0, col))
            } else {
                None
            }
        }
        Direction::West => {
            let obstacle_cols = obstacles_per_row.get(&pos.0);
            if let Some(obstacle_cols) = obstacle_cols {
                let mut obstacle_cols = obstacle_cols.clone();
                obstacle_cols.sort();
                let mut col_opt: Option<i32> = None;
                for col in obstacle_cols.iter() {
                    if *col < pos.1 {
                        col_opt = Some(*col);
                    } else {
                        break
                    }
                }
                col_opt.map(|col| (pos.0, col))
            } else {
                None
            }
        }
    }
}




