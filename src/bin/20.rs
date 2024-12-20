use adv_code_2024::grid::Direction::{East, North, South, West};
use adv_code_2024::grid::{Grid, Position};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let orig_time = racemap.bfs().unwrap();

        let mut stats = HashMap::new();

        let cheats = racemap.get_possible_cheats();
        let num_cheats = cheats.len();
        let mut num_cheats_better_99 = 0;

        for (i, (cheat1, cheat2)) in cheats.iter().enumerate() {
            let mut rm = racemap.clone();
            rm.set_cheat(&cheat1, 1);
            rm.set_cheat(&cheat2, 2);
            match rm.bfs() {
                None => continue,
                Some(time) => {
                    let saved_time = orig_time - time;
                    if saved_time > 0 {
                        println!("{}/{}: {}", i+1, num_cheats, saved_time);
                        stats.entry(saved_time)
                            .and_modify(|cnt| *cnt += 1)
                            .or_insert(1);
                    }
                    if saved_time > 99 {
                        num_cheats_better_99 += 1;
                    }
                }
            }
        }

        Ok(num_cheats_better_99)
    }

    //assert_eq!(84, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
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
    Cheat(u8),
}

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

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let pos = Position::new(row, col);
                let cell = self.grid.value_at(&pos).unwrap();
                match *cell {
                    Cell::Empty => {
                        if pos == self.start {
                            print!("S");
                        } else if pos == self.end {
                            print!("E");
                        } else {
                            print!(".")
                        }
                    }
                    Cell::Wall => print!("#"),
                    Cell::Cheat(time) => print!("{}", time),
                }
            }
            println!();
        }

        println!();
    }

    fn set_cheat(&mut self, pos: &Position, time: u8) {
        self.grid.set_value_at(pos, Cell::Cheat(time));
    }

    fn get_possible_cheats(&self) -> Vec<(Position, Position)> {
        let mut ret = Vec::new();
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let pos = Position::new(row, col);
                let cell = self.grid.value_at(&pos).unwrap(); 
                if *cell == Cell::Wall {
                    let nb_positions = [North, South, East, West]
                        .iter()
                        .map(|dir| pos.make_step(dir))
                        .filter(|pos| self.grid.is_valid_position(pos))
                        .filter(|pos| {
                            let nb_cell = self.grid.value_at(&pos).unwrap();
                            *nb_cell == Cell::Empty
                        })
                        .collect::<Vec<_>>();
                    for nb_pos in nb_positions {
                        ret.push((pos.clone(), nb_pos));
                    }
                    
                }
            }
        }

        ret
    }

    fn bfs(&self) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back((self.start.clone(), 0));

        let mut visited = HashSet::new();

        while let Some((pos, distance)) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos.clone());

            if pos == self.end {
                return Some(distance);
            }

            for nb in self.get_neighbors(&pos) {
                if !visited.contains(&nb) {
                    queue.push_back((nb.clone(), distance + 1));
                }
            }
        }

        None
    }

    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut ret = Vec::new();
        let cell = self.grid.value_at(pos).unwrap();

        let positions = [North, South, East, West]
            .iter()
            .map(|dir| pos.make_step(dir))
            .filter(|pos| self.grid.is_valid_position(pos))
            .collect::<Vec<_>>();

        for nb in positions {
            let nb_cell = self.grid.value_at(&nb).unwrap();
            match *cell {
                Cell::Cheat(1) => {
                    if *nb_cell != Cell::Cheat(2) {
                        continue;
                    } 
                }
                _ => {
                    if *nb_cell == Cell::Wall {
                        continue;
                    }
                }
            }
            ret.push(nb.clone());
        }

        ret
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
