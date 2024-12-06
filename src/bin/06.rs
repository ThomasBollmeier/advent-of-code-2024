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
                if !area.obstructions.contains(&(row, col)) {
                    let (_, is_loop) = area.guard_walk(Some((row, col)));
                    if is_loop {
                        total += 1;
                    }
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
type Direction = (i32, i32);

#[derive(Debug)]
struct Area {
    width: i32,
    height: i32,
    guard: Position,
    guard_direction: Direction,
    obstructions: HashSet<Position>,
}

impl Area {
    fn from_input<R: BufRead>(reader: R) -> Result<Area> {
        let lines = read_lines(reader);

        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        let mut guard: Option<Position> = None;
        let mut guard_direction: Option<Direction> = None;
        let mut obstructions: HashSet<Position> = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        obstructions.insert((row as i32, col as i32));
                    }
                    '<' | '>' | 'v' | '^' => {
                        guard = Some((row as i32, col as i32));
                        guard_direction = match ch {
                            '<' => Some((0, -1)),
                            '>' => Some((0, 1)),
                            'v' => Some((1, 0)),
                            '^' => Some((-1, 0)),
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
            })
        } else {
            Err(anyhow!("Guard was not found"))
        }
    }

    fn guard_walk(&self, additional_obstruction: Option<Position>) -> (usize, bool) {
        let mut visited = HashSet::new();
        let mut visited_states = HashSet::new();
        let mut curr_pos = self.guard;
        let mut curr_dir = self.guard_direction;

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

            let next_pos = (curr_pos.0 + curr_dir.0, curr_pos.1 + curr_dir.1);
            if !(0..self.width).contains(&next_pos.1) || !(0..self.height).contains(&next_pos.0) {
                return (visited.len(), false);
            }

            if !obstructions.contains(&next_pos) {
                curr_pos = next_pos;
            } else {
                curr_dir = (curr_dir.1, -curr_dir.0);
            }
        }
    }
}
